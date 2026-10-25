//! Virtual Microphone and Audio Subsystem Drivers.
//!
//! Provides platform-agnostic abstractions for Linux (PipeWire / PulseAudio),
//! Windows (WASAPI), and macOS (CoreAudio).

pub mod pipewire;
pub mod wasapi;
pub mod coreaudio;

use crate::error::Result;
pub use pipewire::PipeWireDriver;
pub use wasapi::WasapiDriver;
pub use coreaudio::CoreAudioDriver;

/// Common interface for virtual microphone device drivers across operating systems.
pub trait VirtualAudioDriver {
    /// Human-readable platform name.
    fn platform(&self) -> &'static str;

    /// Registers and activates the virtual microphone device.
    fn create_virtual_mic(&mut self, virtual_device_name: &str) -> Result<()>;

    /// Unregisters and frees the virtual microphone device.
    fn remove_virtual_mic(&mut self) -> Result<()>;

    /// Returns whether the virtual device is active and receiving audio.
    fn is_active(&self) -> bool;
}

/// Creates the platform-appropriate virtual audio driver for the current OS.
pub fn get_default_driver() -> Box<dyn VirtualAudioDriver> {
    #[cfg(target_os = "linux")]
    {
        Box::new(PipeWireDriver::new())
    }
    #[cfg(target_os = "windows")]
    {
        Box::new(WasapiDriver::new())
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(CoreAudioDriver::new())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Box::new(PipeWireDriver::new())
    }
}
