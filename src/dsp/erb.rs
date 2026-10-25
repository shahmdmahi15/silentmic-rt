//! Equivalent Rectangular Bandwidth (ERB) 32-band psychoacoustic filterbank.
//!
//! Implements Glasberg & Moore (1990) auditory scale mapping from 65 STFT bins (0 to 24 kHz)
//! into 32 biologically inspired auditory sub-bands. Also provides synthesis interpolation
//! back to 65 bins for mask application.

use realfft::num_complex::Complex32;
use crate::dsp::stft::{FFT_BINS, SAMPLE_RATE, WINDOW_SIZE};

/// Number of psychoacoustic ERB bands.
pub const NUM_ERB_BANDS: usize = 32;
/// Lower frequency bound for human speech psychoacoustic analysis (Hz).
pub const LOW_FREQ: f32 = 50.0;
/// Upper frequency bound for human speech psychoacoustic analysis (Hz).
pub const HIGH_FREQ: f32 = 20000.0;

/// Equivalent Rectangular Bandwidth (ERB) filterbank with precomputed analysis
/// and synthesis matrix projections.
#[derive(Clone)]
pub struct ErbFilterbank {
    /// Analysis weighting matrix: [NUM_ERB_BANDS][FFT_BINS]
    analysis_weights: [[f32; FFT_BINS]; NUM_ERB_BANDS],
    /// Synthesis interpolation matrix: [FFT_BINS][NUM_ERB_BANDS]
    synthesis_weights: [[f32; NUM_ERB_BANDS]; FFT_BINS],
    /// Center frequencies of the 32 ERB bands in Hertz
    center_frequencies: [f32; NUM_ERB_BANDS],
}

impl ErbFilterbank {
    /// Computes the ERB filterbank matrices for 48kHz, 128-pt FFT (65 bins), and 32 bands.
    pub fn new() -> Self {
        let mut filterbank = Self {
            analysis_weights: [[0.0; FFT_BINS]; NUM_ERB_BANDS],
            synthesis_weights: [[0.0; NUM_ERB_BANDS]; FFT_BINS],
            center_frequencies: [0.0; NUM_ERB_BANDS],
        };

        filterbank.init_filterbank();
        filterbank
    }

    /// Converts frequency in Hz to ERB rate (Glasberg & Moore, 1990).
    #[inline(always)]
    pub fn hz_to_erb(freq: f32) -> f32 {
        21.4 * (0.00437 * freq + 1.0).log10()
    }

    /// Converts ERB rate to frequency in Hz.
    #[inline(always)]
    pub fn erb_to_hz(erb: f32) -> f32 {
        (10.0f32.powf(erb / 21.4) - 1.0) / 0.00437
    }

    /// Initializes triangular filter weighting matrices.
    fn init_filterbank(&mut self) {
        let min_erb = Self::hz_to_erb(LOW_FREQ);
        let max_erb = Self::hz_to_erb(HIGH_FREQ);
        let erb_step = (max_erb - min_erb) / (NUM_ERB_BANDS as f32 + 1.0);

        // Compute band center frequencies in Hz
        for i in 0..NUM_ERB_BANDS {
            let erb_val = min_erb + (i as f32 + 1.0) * erb_step;
            self.center_frequencies[i] = Self::erb_to_hz(erb_val);
        }

        let bin_width = SAMPLE_RATE as f32 / WINDOW_SIZE as f32; // 375 Hz per bin

        // Build triangular filters for analysis
        for k in 0..NUM_ERB_BANDS {
            let left_freq = if k == 0 {
                LOW_FREQ
            } else {
                self.center_frequencies[k - 1]
            };
            let center_freq = self.center_frequencies[k];
            let right_freq = if k == NUM_ERB_BANDS - 1 {
                HIGH_FREQ
            } else {
                self.center_frequencies[k + 1]
            };

            let mut sum_weights = 0.0f32;

            for b in 0..FFT_BINS {
                let bin_freq = b as f32 * bin_width;
                let weight = if bin_freq >= left_freq && bin_freq <= center_freq {
                    (bin_freq - left_freq) / (center_freq - left_freq).max(1e-6)
                } else if bin_freq > center_freq && bin_freq <= right_freq {
                    (right_freq - bin_freq) / (right_freq - center_freq).max(1e-6)
                } else {
                    0.0
                };

                self.analysis_weights[k][b] = weight;
                sum_weights += weight;
            }

            // Normalize analysis filter energy so unit energy in bins maps to unit band energy
            if sum_weights > 1e-6 {
                for b in 0..FFT_BINS {
                    self.analysis_weights[k][b] /= sum_weights;
                }
            }
        }

        // Build synthesis interpolation matrix (maps 32 ERB gains -> 65 FFT bins)
        for b in 0..FFT_BINS {
            let mut bin_sum = 0.0f32;
            for k in 0..NUM_ERB_BANDS {
                // Transpose analysis weights with normalization across bands per bin
                let w = self.analysis_weights[k][b];
                self.synthesis_weights[b][k] = w;
                bin_sum += w;
            }

            if bin_sum > 1e-6 {
                for k in 0..NUM_ERB_BANDS {
                    self.synthesis_weights[b][k] /= bin_sum;
                }
            } else {
                // Nearest neighbor fallback for edge bins (e.g. DC bin 0 or extreme Nyquist)
                let nearest = if b < FFT_BINS / 2 { 0 } else { NUM_ERB_BANDS - 1 };
                self.synthesis_weights[b][nearest] = 1.0;
            }
        }
    }

    /// Analyzes the complex STFT spectrum and extracts 32 ERB log-energy features.
    ///
    /// Computes $E_k = \ln(\sum_b W_{k, b} |X_b|^2 + \epsilon)$ for each band $k$.
    #[inline]
    pub fn extract_log_energies(&self, spectrum: &[Complex32; FFT_BINS], log_energies: &mut [f32; NUM_ERB_BANDS]) {
        // Precompute bin power: |X_b|^2 = Re^2 + Im^2
        let mut bin_powers = [0.0f32; FFT_BINS];
        for (b, p) in bin_powers.iter_mut().enumerate() {
            let c = spectrum[b];
            *p = c.re * c.re + c.im * c.im;
        }

        // Apply analysis matrix: E_k = sum_b W_{k, b} * P_b
        for k in 0..NUM_ERB_BANDS {
            let weights = &self.analysis_weights[k];
            let mut band_energy = 0.0f32;
            for b in 0..FFT_BINS {
                band_energy += weights[b] * bin_powers[b];
            }
            // Logarithmic compression with safety floor (1e-10)
            log_energies[k] = (band_energy + 1e-10).ln();
        }
    }

    /// Synthesizes 65 bin gains from 32 ERB band gains using smooth interpolation.
    #[inline]
    pub fn synthesize_bin_gains(&self, erb_gains: &[f32; NUM_ERB_BANDS], bin_gains: &mut [f32; FFT_BINS]) {
        for b in 0..FFT_BINS {
            let weights = &self.synthesis_weights[b];
            let mut gain = 0.0f32;
            for k in 0..NUM_ERB_BANDS {
                gain += weights[k] * erb_gains[k];
            }
            bin_gains[b] = gain.clamp(0.0, 1.0);
        }
    }

    /// Returns the center frequencies of the 32 bands.
    pub fn center_frequencies(&self) -> &[f32; NUM_ERB_BANDS] {
        &self.center_frequencies
    }
}

impl Default for ErbFilterbank {
    fn default() -> Self {
        Self::new()
    }
}
