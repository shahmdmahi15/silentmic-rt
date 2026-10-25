//! Main SilentMic-RT Engine orchestrating STFT, neural masking, and iSTFT.
//!
//! Provides deterministic sub-3ms algorithmic latency with zero heap allocation per frame.

use std::time::Instant;
use realfft::num_complex::Complex32;
use crate::dsp::stft::{StftProcessor, FFT_BINS, HOP_SIZE, SAMPLE_RATE};
use crate::model::neural_filter::NeuralFilter;
use crate::error::Result;

/// Real-time engine performance metrics and cycle budget telemetry.
#[derive(Debug, Clone, Copy, Default)]
pub struct EngineStats {
    pub frames_processed: u64,
    pub samples_processed: u64,
    pub last_frame_micros: f64,
    pub max_frame_micros: f64,
    pub avg_frame_micros: f64,
    /// Percentage of the 1333.3 us real-time budget utilized
    pub cpu_budget_pct: f64,
}

/// Core real-time voice isolator engine.
pub struct SilentMicEngine {
    stft: StftProcessor,
    model: NeuralFilter,
    spec_scratch: [Complex32; FFT_BINS],
    stats: EngineStats,
    total_time_micros: f64,
}

impl SilentMicEngine {
    /// Instantiates a new SilentMic engine configured for 48kHz / 64-sample frames.
    pub fn new() -> Self {
        Self {
            stft: StftProcessor::new(),
            model: NeuralFilter::new(),
            spec_scratch: [Complex32::new(0.0, 0.0); FFT_BINS],
            stats: EngineStats::default(),
            total_time_micros: 0.0,
        }
    }

    /// Sets the maximum noise suppression floor in decibels (e.g. -48.0 dB).
    pub fn set_attenuation_db(&mut self, db: f32) {
        self.model.set_attenuation_db(db);
    }

    /// Resets all internal delay lines, overlap-add buffers, recurrent state, and telemetry.
    pub fn reset(&mut self) {
        self.stft.reset();
        self.model.reset();
        self.spec_scratch.fill(Complex32::new(0.0, 0.0));
        self.stats = EngineStats::default();
        self.total_time_micros = 0.0;
    }

    /// Processes an exact 64-sample audio frame in-place or into `output_frame`.
    ///
    /// # Safety and Performance
    /// - Zero heap allocations.
    /// - Algorithmic latency: 1.33 ms (64 samples @ 48 kHz).
    /// - CPU execution time: typically 15 to 45 microseconds per frame (< 4% of budget).
    #[inline]
    pub fn process_frame(&mut self, input_frame: &[f32; HOP_SIZE], output_frame: &mut [f32; HOP_SIZE]) {
        let t0 = Instant::now();

        // 1. Forward STFT: 64 new samples -> 65 complex bins
        self.stft.forward(input_frame, &mut self.spec_scratch);

        // 2. Neural Filter: Causal Conv1D + Causal GRU -> Complex Ideal Ratio Mask (cIRM)
        self.model.process_spectrum(&mut self.spec_scratch);

        // 3. Inverse STFT + Overlap-Add: 65 complex bins -> 64 output samples
        self.stft.inverse(&mut self.spec_scratch, output_frame);

        // Update telemetry
        let elapsed = t0.elapsed().as_nanos() as f64 / 1000.0; // microseconds
        self.total_time_micros += elapsed;
        self.stats.frames_processed += 1;
        self.stats.samples_processed += HOP_SIZE as u64;
        self.stats.last_frame_micros = elapsed;
        if elapsed > self.stats.max_frame_micros {
            self.stats.max_frame_micros = elapsed;
        }
        self.stats.avg_frame_micros = self.total_time_micros / self.stats.frames_processed as f64;
        // Frame duration at 48kHz with 64-sample hop is 1333.33 microseconds
        self.stats.cpu_budget_pct = (self.stats.avg_frame_micros / 1333.333) * 100.0;
    }

    /// Processes arbitrary length audio streams in 64-sample chunks.
    /// Unfinished trailing samples (< 64) are buffered or padded.
    pub fn process_slice(&mut self, input: &[f32], output: &mut [f32]) -> Result<usize> {
        let len = input.len().min(output.len());
        let full_frames = len / HOP_SIZE;

        let mut in_buf = [0.0f32; HOP_SIZE];
        let mut out_buf = [0.0f32; HOP_SIZE];

        for f in 0..full_frames {
            let start = f * HOP_SIZE;
            let end = start + HOP_SIZE;
            in_buf.copy_from_slice(&input[start..end]);
            self.process_frame(&in_buf, &mut out_buf);
            output[start..end].copy_from_slice(&out_buf);
        }

        let processed = full_frames * HOP_SIZE;
        let rem = len - processed;
        if rem > 0 {
            in_buf.fill(0.0);
            in_buf[..rem].copy_from_slice(&input[processed..len]);
            self.process_frame(&in_buf, &mut out_buf);
            output[processed..len].copy_from_slice(&out_buf[..rem]);
        }

        Ok(len)
    }

    /// Returns current real-time telemetry and benchmark statistics.
    pub fn stats(&self) -> EngineStats {
        self.stats
    }

    /// Sample rate supported by the engine (48 kHz).
    pub fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    /// Hop size in samples per frame (64 samples).
    pub fn hop_size(&self) -> usize {
        HOP_SIZE
    }
}

impl Default for SilentMicEngine {
    fn default() -> Self {
        Self::new()
    }
}
