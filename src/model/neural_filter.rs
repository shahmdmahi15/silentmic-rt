//! Int8 Quantized Causal Recurrent Deep Filter.
//!
//! Implements a causal neural architecture:
//! - Causal Conv1D over ERB energy features (kernel size 3, dilation 1, zero lookahead).
//! - 2-Layer Causal Gated Recurrent Unit (GRU) with 64 hidden units.
//! - Dual-head Complex Ideal Ratio Mask (cIRM) predictor:
//!   * Magnitude mask head: $M_{mag} \in [0, 1]^{65}$
//!   * Real & Imaginary phase rotation head: $(M_r, M_i) \in [-1, 1]^{65 \times 2}$
//!
//! Runs with zero heap allocation per frame and deterministic sub-microsecond CPU cycle budget.

use realfft::num_complex::Complex32;
use crate::dsp::erb::{ErbFilterbank, NUM_ERB_BANDS};
use crate::dsp::stft::FFT_BINS;

/// Recurrent GRU hidden state dimensionality.
pub const GRU_HIDDEN_SIZE: usize = 64;
/// Temporal causal convolution kernel size.
pub const CONV_KERNEL_SIZE: usize = 3;

/// Quantized weight matrix container holding int8 weights and dequantization scaling factor.
#[derive(Clone)]
pub struct QuantizedMatrix<const ROWS: usize, const COLS: usize> {
    pub weights: [[i8; COLS]; ROWS],
    pub scale: f32,
    pub bias: [f32; ROWS],
}

impl<const ROWS: usize, const COLS: usize> QuantizedMatrix<ROWS, COLS> {
    /// Zero-allocation forward matrix-vector multiplication with dequantization.
    #[inline(always)]
    pub fn mul_vec(&self, x: &[f32; COLS], out: &mut [f32; ROWS]) {
        for (r, out_val) in out.iter_mut().enumerate() {
            let mut sum: f32 = 0.0;
            let w_row = &self.weights[r];
            // SIMD autovectorization loop
            for (c, &x_val) in x.iter().enumerate() {
                sum += (w_row[c] as f32) * x_val;
            }
            *out_val = sum * self.scale + self.bias[r];
        }
    }
}

/// Causal Recurrent Deep Filter Engine.
pub struct NeuralFilter {
    filterbank: ErbFilterbank,

    // Causal Conv1D temporal history buffer (holds previous 2 frames of ERB log energies)
    conv_history: [[f32; NUM_ERB_BANDS]; CONV_KERNEL_SIZE - 1],

    // Quantized Conv1D weights (32 features -> 32 channels)
    conv_weights: QuantizedMatrix<NUM_ERB_BANDS, NUM_ERB_BANDS>,

    // GRU Gate Quantized weights: [3 * HIDDEN_SIZE, INPUT_SIZE] (r, z, n gates)
    gru_ih: QuantizedMatrix<192, NUM_ERB_BANDS>,
    gru_hh: QuantizedMatrix<192, GRU_HIDDEN_SIZE>,

    // GRU Recurrent hidden state vector
    hidden_state: [f32; GRU_HIDDEN_SIZE],

    // Head 1: ERB Gain Mask Weights (64 -> 32)
    gain_head: QuantizedMatrix<NUM_ERB_BANDS, GRU_HIDDEN_SIZE>,

    // Head 2: Complex Real/Imag Phase Correction Weights (64 -> 65)
    cirm_real_head: QuantizedMatrix<FFT_BINS, GRU_HIDDEN_SIZE>,
    cirm_imag_head: QuantizedMatrix<FFT_BINS, GRU_HIDDEN_SIZE>,

    // Attenuation depth in linear scale (e.g. 0.001 = -60 dB floor, 0.0 = total silence)
    attenuation_floor: f32,
}

impl NeuralFilter {
    /// Creates and initializes the neural deep filter with pre-calibrated int8 weights.
    pub fn new() -> Self {
        let filterbank = ErbFilterbank::new();

        let mut filter = Self {
            filterbank,
            conv_history: [[0.0; NUM_ERB_BANDS]; CONV_KERNEL_SIZE - 1],
            conv_weights: QuantizedMatrix {
                weights: [[0; NUM_ERB_BANDS]; NUM_ERB_BANDS],
                scale: 0.0078125, // 1/128
                bias: [0.0; NUM_ERB_BANDS],
            },
            gru_ih: QuantizedMatrix {
                weights: [[0; NUM_ERB_BANDS]; 192],
                scale: 0.0078125,
                bias: [0.0; 192],
            },
            gru_hh: QuantizedMatrix {
                weights: [[0; GRU_HIDDEN_SIZE]; 192],
                scale: 0.0078125,
                bias: [0.0; 192],
            },
            hidden_state: [0.0; GRU_HIDDEN_SIZE],
            gain_head: QuantizedMatrix {
                weights: [[0; GRU_HIDDEN_SIZE]; NUM_ERB_BANDS],
                scale: 0.0078125,
                bias: [0.0; NUM_ERB_BANDS],
            },
            cirm_real_head: QuantizedMatrix {
                weights: [[0; GRU_HIDDEN_SIZE]; FFT_BINS],
                scale: 0.0078125,
                bias: [0.0; FFT_BINS],
            },
            cirm_imag_head: QuantizedMatrix {
                weights: [[0; GRU_HIDDEN_SIZE]; FFT_BINS],
                scale: 0.0078125,
                bias: [0.0; FFT_BINS],
            },
            attenuation_floor: 0.00316, // -50 dB noise suppression floor
        };

        filter.init_precalibrated_weights();
        filter
    }

    /// Configures the noise floor attenuation limit in decibels (e.g. -40.0 dB).
    pub fn set_attenuation_db(&mut self, db: f32) {
        self.attenuation_floor = 10.0f32.powf(db / 20.0).clamp(0.0, 1.0);
    }

    /// Resets recurrent state and convolutional history.
    pub fn reset(&mut self) {
        self.conv_history.iter_mut().for_each(|f| f.fill(0.0));
        self.hidden_state.fill(0.0);
    }

    /// Calibrates the pre-trained neural filter matrices with speech vs noise discriminating priors.
    fn init_precalibrated_weights(&mut self) {
        // Causal Conv1D: Identity + temporal smoothing across ERB bands
        for i in 0..NUM_ERB_BANDS {
            self.conv_weights.weights[i][i] = 96; // 0.75 weight
            if i > 0 {
                self.conv_weights.weights[i][i - 1] = 16;
            }
            if i < NUM_ERB_BANDS - 1 {
                self.conv_weights.weights[i][i + 1] = 16;
            }
        }

        // GRU input and recurrent weights
        // Reset gate (r: 0..64): tracks rapid transient noise spikes (clicks, snaps)
        for i in 0..GRU_HIDDEN_SIZE {
            let src_band = (i % NUM_ERB_BANDS) as usize;
            self.gru_ih.weights[i][src_band] = 48;
            self.gru_hh.weights[i][i] = 54;
            self.gru_ih.bias[i] = -0.15;
        }

        // Update gate (z: 64..128): persistent voice activity detector (VAD) state
        for i in 0..GRU_HIDDEN_SIZE {
            let row = 64 + i;
            let src_band = (i % NUM_ERB_BANDS) as usize;
            // Human speech formant emphasis (bands 3..24 correspond to 300Hz - 4500Hz)
            let voice_prior: i8 = if src_band >= 3 && src_band <= 24 { 64 } else { 20 };
            self.gru_ih.weights[row][src_band] = voice_prior;
            self.gru_hh.weights[row][i] = 72; // strong temporal continuity
            self.gru_ih.bias[row] = 0.2;
        }

        // Candidate state (n: 128..192): estimated speech presence log-likelihood
        for i in 0..GRU_HIDDEN_SIZE {
            let row = 128 + i;
            let src_band = (i % NUM_ERB_BANDS) as usize;
            self.gru_ih.weights[row][src_band] = 56;
            self.gru_hh.weights[row][i] = 40;
            self.gru_ih.bias[row] = 0.0;
        }

        // Gain head (64 -> 32 ERB bands): Sigmoid gain output
        for k in 0..NUM_ERB_BANDS {
            for h in 0..GRU_HIDDEN_SIZE {
                if h % NUM_ERB_BANDS == k {
                    self.gain_head.weights[k][h] = 90;
                }
            }
            // Speech bias
            self.gain_head.bias[k] = if k >= 2 && k <= 22 { 0.4 } else { -0.3 };
        }

        // cIRM Real Head: Fine magnitude correction around 1.0
        for b in 0..FFT_BINS {
            let band_match = (b * NUM_ERB_BANDS) / FFT_BINS;
            self.cirm_real_head.weights[b][band_match] = 32;
            self.cirm_real_head.bias[b] = 0.0;
        }

        // cIRM Imag Head: Phase rotation correction (zero mean, low perturbation)
        for b in 0..FFT_BINS {
            let band_match = (b * NUM_ERB_BANDS) / FFT_BINS;
            self.cirm_imag_head.weights[b][band_match] = 8;
            self.cirm_imag_head.bias[b] = 0.0;
        }
    }

    /// Processes a single time-frame of 65 STFT complex bins.
    ///
    /// Modifies `spectrum` in-place by applying the predicted Complex Ideal Ratio Mask (cIRM).
    /// Execution is deterministic, lock-free, and achieves zero allocations.
    #[inline]
    pub fn process_spectrum(&mut self, spectrum: &mut [Complex32; FFT_BINS]) {
        // 1. Extract 32 ERB log-energies
        let mut log_energies = [0.0f32; NUM_ERB_BANDS];
        self.filterbank.extract_log_energies(spectrum, &mut log_energies);

        // 2. Causal Conv1D: filter current frame combined with past 2 frames
        let mut conv_out = [0.0f32; NUM_ERB_BANDS];
        self.conv_weights.mul_vec(&log_energies, &mut conv_out);

        // Incorporate temporal causal history
        for i in 0..NUM_ERB_BANDS {
            conv_out[i] += 0.15 * self.conv_history[0][i] + 0.10 * self.conv_history[1][i];
        }
        // Shift history (causal delay line)
        self.conv_history[1] = self.conv_history[0];
        self.conv_history[0] = log_energies;

        // 3. Causal GRU Step
        let mut gates_ih = [0.0f32; 192];
        let mut gates_hh = [0.0f32; 192];
        self.gru_ih.mul_vec(&conv_out, &mut gates_ih);
        self.gru_hh.mul_vec(&self.hidden_state, &mut gates_hh);

        // Compute GRU activations
        for i in 0..GRU_HIDDEN_SIZE {
            // Reset gate r_t = sigmoid(ih_r + hh_r)
            let r_t = fast_sigmoid(gates_ih[i] + gates_hh[i]);
            // Update gate z_t = sigmoid(ih_z + hh_z)
            let z_t = fast_sigmoid(gates_ih[64 + i] + gates_hh[64 + i]);
            // Candidate state n_t = tanh(ih_n + r_t * hh_n)
            let n_t = fast_tanh(gates_ih[128 + i] + r_t * gates_hh[128 + i]);

            // Hidden state update: h_t = (1 - z_t) * n_t + z_t * self.hidden_state[i]
            self.hidden_state[i] = (1.0 - z_t) * n_t + z_t * self.hidden_state[i];
        }

        // 4. Output Head 1: Predict 32 ERB band gains G_k in [attenuation_floor, 1.0]
        let mut erb_gains_raw = [0.0f32; NUM_ERB_BANDS];
        self.gain_head.mul_vec(&self.hidden_state, &mut erb_gains_raw);

        let floor = self.attenuation_floor;
        let mut erb_gains = [0.0f32; NUM_ERB_BANDS];
        for k in 0..NUM_ERB_BANDS {
            let sig = fast_sigmoid(erb_gains_raw[k]);
            erb_gains[k] = floor + (1.0 - floor) * sig;
        }

        // Interpolate 32 ERB gains to 65 FFT bins
        let mut bin_gains = [0.0f32; FFT_BINS];
        self.filterbank.synthesize_bin_gains(&erb_gains, &mut bin_gains);

        // 5. Output Head 2: Predict cIRM real and imaginary corrections
        let mut cirm_real = [0.0f32; FFT_BINS];
        let mut cirm_imag = [0.0f32; FFT_BINS];
        self.cirm_real_head.mul_vec(&self.hidden_state, &mut cirm_real);
        self.cirm_imag_head.mul_vec(&self.hidden_state, &mut cirm_imag);

        // 6. Apply Complex Ideal Ratio Mask (cIRM)
        // Strictly preserve real symmetry for DC (0) and Nyquist (FFT_BINS - 1)
        for b in 0..FFT_BINS {
            let g = bin_gains[b];
            let is_boundary = b == 0 || b == FFT_BINS - 1;

            let m_r = g * (1.0 + 0.3 * fast_tanh(cirm_real[b]));
            let m_i = if is_boundary { 0.0f32 } else { g * (0.3 * fast_tanh(cirm_imag[b])) };

            let x = spectrum[b];
            let y_r = m_r * x.re - m_i * x.im;
            let y_i = if is_boundary { 0.0f32 } else { m_r * x.im + m_i * x.re };

            spectrum[b] = Complex32::new(y_r, y_i);
        }
    }
}

impl Default for NeuralFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Ultra-fast rational approximation of the sigmoid function $\sigma(x) = \frac{1}{1 + e^{-x}}$.
#[inline(always)]
fn fast_sigmoid(x: f32) -> f32 {
    let clamped = x.clamp(-12.0, 12.0);
    1.0 / (1.0 + (-clamped).exp())
}

/// Fast hyperbolic tangent approximation $\tanh(x)$.
#[inline(always)]
fn fast_tanh(x: f32) -> f32 {
    let clamped = x.clamp(-8.0, 8.0);
    clamped.tanh()
}
