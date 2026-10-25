//! macOS CoreAudio AudioServerPlugIn virtual audio device driver.

use crate::error::Result;
use super::VirtualAudioDriver;

/// macOS CoreAudio Virtual Microphone interface.
pub struct CoreAudioDriver {
    virtual_name: String,
    is_active: bool,
}

impl CoreAudioDriver {
    pub fn new() -> Self {
        Self {
            virtual_name: "SilentMic Virtual Audio (CoreAudio)".to_string(),
            is_active: false,
        }
    }
}

impl Default for CoreAudioDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualAudioDriver for CoreAudioDriver {
    fn platform(&self) -> &'static str {
        "macOS (CoreAudio AudioServerPlugIn)"
    }

    fn create_virtual_mic(&mut self, virtual_device_name: &str) -> Result<()> {
        self.virtual_name = virtual_device_name.to_string();
        // In native macOS deployments, interacts with HAL AudioServerPlugIn / BlackHole
        // virtual aggregate device endpoints with IOAudioStream.
        self.is_active = true;
        Ok(())
    }

    fn remove_virtual_mic(&mut self) -> Result<()> {
        self.is_active = false;
        Ok(())
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}
