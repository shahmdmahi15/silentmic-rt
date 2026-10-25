//! # SilentMic-RT
//!
//! **Deterministic, Zero-Allocation Sub-3ms Hard Real-Time Neural Voice Isolator & Virtual Microphone Driver**
//!
//! SilentMic-RT is designed from the metal up for mission-critical, ultra-low latency real-time voice
//! communication. It eliminates background office chatter, dog barks, mechanical keyboard clicks,
//! baby cries, HVAC hum, and acoustic room reverberation directly on bare-metal CPU SIMD with zero cloud transmission.
//!
//! ## Architectural Highlights
//! - **Algorithmic Latency**: 1.33 ms (64 audio samples at 48 kHz).
//! - **Real-Time Safety**: Zero heap allocations, zero locks, zero mutexes on the audio thread.
//! - **Psychoacoustic Feature Extraction**: 32-band Equivalent Rectangular Bandwidth (ERB) filterbank.
//! - **Neural Masking**: Int8 quantized causal recurrent deep filter predicting Complex Ideal Ratio Masks ($M_r + i M_i$).
//! - **Multi-OS Virtual Mic Drivers**: PipeWire (Linux), WASAPI (Windows), and CoreAudio (macOS).

pub mod buffer;
pub mod dsp;
pub mod error;
pub mod driver;
pub mod engine;
pub mod ffi;
pub mod model;

pub use buffer::{make_spsc_ring_buffer, Consumer, Producer, SpscRingBuffer};
pub use dsp::{ErbFilterbank, StftProcessor, FFT_BINS, HOP_SIZE, NUM_ERB_BANDS, SAMPLE_RATE, WINDOW_SIZE};
pub use driver::{get_default_driver, CoreAudioDriver, PipeWireDriver, VirtualAudioDriver, WasapiDriver};
pub use engine::{EngineStats, SilentMicEngine};
pub use error::{Result, SilentMicError};
pub use model::NeuralFilter;
