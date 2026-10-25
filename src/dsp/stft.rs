//! Single-Instruction SIMD short-time Fourier transform (STFT).
//!
//! Configured for ultra-low latency real-time voice processing:
//! - Sample rate: 48,000 Hz
//! - Frame hop size: 64 samples (1.33 ms algorithmic time-step)
//! - Window length: 128 samples (2.67 ms analysis window)
//! - Overlap: 50%
//! - Frequency bins: 65 (DC to 24 kHz Nyquist)
//! - Window function: Symmetrical Sine analysis & synthesis window (satisfying Principle of Overlap-Add)

use std::sync::Arc;
use realfft::{ComplexToReal, RealFftPlanner, RealToComplex};
use realfft::num_complex::Complex32;

/// Audio sample rate standard for professional communications and broadcasting.
pub const SAMPLE_RATE: u32 = 48000;
/// Hop size in samples per real-time frame (1.33 ms).
pub const HOP_SIZE: usize = 64;
/// FFT window size in samples (2.67 ms).
pub const WINDOW_SIZE: usize = 128;
/// Number of complex frequency bins (N/2 + 1).
pub const FFT_BINS: usize = WINDOW_SIZE / 2 + 1; // 65 bins

/// Zero-allocation, SIMD-optimized STFT and iSTFT engine for real-time streaming.
pub struct StftProcessor {
    r2c: Arc<dyn RealToComplex<f32>>,
    c2r: Arc<dyn ComplexToReal<f32>>,
    forward_scratch: Vec<Complex32>,
    inverse_scratch: Vec<Complex32>,

    // Analysis window (sine window for exact overlap-add)
    analysis_window: [f32; WINDOW_SIZE],
    synthesis_window: [f32; WINDOW_SIZE],

    // Input analysis buffer (holds previous hop + current hop)
    input_history: [f32; HOP_SIZE],
    analysis_buffer: [f32; WINDOW_SIZE],

    // Overlap-add synthesis memory
    ola_buffer: [f32; HOP_SIZE],
    synthesis_buffer: [f32; WINDOW_SIZE],

    // Normalization constant (1.0 / N) for realfft IFFT
    inv_window_norm: f32,
}

impl StftProcessor {
    /// Creates a new STFT processor initialized with pre-calculated sine windows
    /// and pre-allocated zero-heap scratch buffers.
    pub fn new() -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let r2c = planner.plan_fft_forward(WINDOW_SIZE);
        let c2r = planner.plan_fft_inverse(WINDOW_SIZE);

        let forward_scratch = r2c.make_scratch_vec();
        let inverse_scratch = c2r.make_scratch_vec();

        let mut analysis_window = [0.0f32; WINDOW_SIZE];
        let mut synthesis_window = [0.0f32; WINDOW_SIZE];

        // Symmetrical sine window: w[n] = sin(pi * (n + 0.5) / N)
        // Satisfies w[n]^2 + w[n + N/2]^2 = 1.0 for exact overlap-add reconstruction.
        for n in 0..WINDOW_SIZE {
            let val = ((std::f32::consts::PI * (n as f32 + 0.5)) / WINDOW_SIZE as f32).sin();
            analysis_window[n] = val;
            synthesis_window[n] = val;
        }

        Self {
            r2c,
            c2r,
            forward_scratch,
            inverse_scratch,
            analysis_window,
            synthesis_window,
            input_history: [0.0f32; HOP_SIZE],
            analysis_buffer: [0.0f32; WINDOW_SIZE],
            ola_buffer: [0.0f32; HOP_SIZE],
            synthesis_buffer: [0.0f32; WINDOW_SIZE],
            inv_window_norm: 1.0 / (WINDOW_SIZE as f32),
        }
    }

    /// Resets all internal delay lines and overlap-add memory.
    pub fn reset(&mut self) {
        self.input_history.fill(0.0);
        self.analysis_buffer.fill(0.0);
        self.ola_buffer.fill(0.0);
        self.synthesis_buffer.fill(0.0);
    }

    /// Executes forward STFT on the next 64-sample input frame.
    ///
    /// Copies the previous 64 samples + the new 64 samples into the 128-sample window,
    /// applies SIMD sine windowing, and writes 65 complex frequency bins into `spectrum`.
    #[inline]
    pub fn forward(&mut self, input_frame: &[f32; HOP_SIZE], spectrum: &mut [Complex32; FFT_BINS]) {
        // Construct 128-sample frame from history + new frame
        self.analysis_buffer[..HOP_SIZE].copy_from_slice(&self.input_history);
        self.analysis_buffer[HOP_SIZE..].copy_from_slice(input_frame);
        // Save current input to history for next hop
        self.input_history.copy_from_slice(input_frame);

        // Apply analysis window with SIMD-friendly vectorization
        apply_window_simd(&mut self.analysis_buffer, &self.analysis_window);

        // Perform in-place forward Real-to-Complex FFT
        self.r2c
            .process_with_scratch(&mut self.analysis_buffer, spectrum, &mut self.forward_scratch)
            .expect("RealFFT forward execution failed");

        // Enforce strict real symmetry on DC and Nyquist bins
        spectrum[0].im = 0.0;
        spectrum[FFT_BINS - 1].im = 0.0;
    }

    /// Executes inverse STFT on the processed 65-bin complex spectrum,
    /// applies synthesis windowing and overlap-add, and outputs the next 64 audio samples.
    #[inline]
    pub fn inverse(&mut self, spectrum: &mut [Complex32; FFT_BINS], output_frame: &mut [f32; HOP_SIZE]) {
        // Strictly guarantee real symmetry on DC and Nyquist bins for RealFFT inverse transform
        spectrum[0].im = 0.0;
        spectrum[FFT_BINS - 1].im = 0.0;

        // Perform Complex-to-Real inverse FFT
        self.c2r
            .process_with_scratch(spectrum, &mut self.synthesis_buffer, &mut self.inverse_scratch)
            .expect("RealFFT inverse execution failed");

        // Scale by 1/N and apply synthesis window
        let norm = self.inv_window_norm;
        apply_window_and_scale_simd(&mut self.synthesis_buffer, &self.synthesis_window, norm);

        // Overlap-add:
        // output[0..64] = ola_buffer[0..64] + synthesis_buffer[0..64]
        // new ola_buffer[0..64] = synthesis_buffer[64..128]
        for i in 0..HOP_SIZE {
            output_frame[i] = self.ola_buffer[i] + self.synthesis_buffer[i];
            self.ola_buffer[i] = self.synthesis_buffer[i + HOP_SIZE];
        }
    }

    /// End-to-end frame processing with user-supplied spectral modification closure.
    ///
    /// Guarantees zero heap allocation and deterministic cycle count.
    #[inline]
    pub fn process_frame<F>(
        &mut self,
        input_frame: &[f32; HOP_SIZE],
        output_frame: &mut [f32; HOP_SIZE],
        mut transform: F,
    ) where
        F: FnMut(&[Complex32; FFT_BINS], &mut [Complex32; FFT_BINS]),
    {
        let mut in_spectrum = [Complex32::new(0.0, 0.0); FFT_BINS];
        let mut out_spectrum = [Complex32::new(0.0, 0.0); FFT_BINS];

        self.forward(input_frame, &mut in_spectrum);
        transform(&in_spectrum, &mut out_spectrum);
        self.inverse(&mut out_spectrum, output_frame);
    }
}

impl Default for StftProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD-vectorized element-wise windowing: buffer[i] *= window[i]
#[inline(always)]
fn apply_window_simd(buffer: &mut [f32; WINDOW_SIZE], window: &[f32; WINDOW_SIZE]) {
    for (b, w) in buffer.iter_mut().zip(window.iter()) {
        *b *= *w;
    }
}

/// SIMD-vectorized element-wise windowing and IFFT scaling: buffer[i] = buffer[i] * window[i] * scale
#[inline(always)]
fn apply_window_and_scale_simd(buffer: &mut [f32; WINDOW_SIZE], window: &[f32; WINDOW_SIZE], scale: f32) {
    for (b, w) in buffer.iter_mut().zip(window.iter()) {
        *b = (*b * *w) * scale;
    }
}
