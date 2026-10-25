//! Error types for SilentMic-RT.

use thiserror::Error;

/// Result type shorthand for SilentMic operations.
pub type Result<T> = std::result::Result<T, SilentMicError>;

/// Root error enumeration covering DSP, model inference, buffer overruns, and I/O.
#[derive(Debug, Error)]
pub enum SilentMicError {
    #[error("Audio buffer overflow: attempted to write {written} samples but only {available} free")]
    BufferOverflow { written: usize, available: usize },

    #[error("Audio buffer underflow: requested {requested} samples but only {available} ready")]
    BufferUnderflow { requested: usize, available: usize },

    #[error("Invalid sample rate: expected {expected} Hz, received {received} Hz")]
    InvalidSampleRate { expected: u32, received: u32 },

    #[error("Invalid frame size: expected {expected} samples, received {received} samples")]
    InvalidFrameSize { expected: usize, received: usize },

    #[error("STFT configuration error: {0}")]
    StftError(String),

    #[error("Neural filter error: {0}")]
    NeuralModelError(String),

    #[error("Audio driver error: {0}")]
    DriverError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("WAV decoding/encoding error: {0}")]
    WavError(String),

    #[error("FFI null pointer exception")]
    NullPointer,
}
