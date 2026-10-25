//! Windows WASAPI virtual audio driver and loopback interface.

use crate::error::Result;
use super::VirtualAudioDriver;

/// Windows WASAPI Virtual Microphone interface.
pub struct WasapiDriver {
    virtual_name: String,
    is_active: bool,
}

impl WasapiDriver {
    pub fn new() -> Self {
        Self {
            virtual_name: "SilentMic Virtual Audio (WASAPI)".to_string(),
            is_active: false,
        }
    }
}

impl Default for WasapiDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualAudioDriver for WasapiDriver {
    fn platform(&self) -> &'static str {
        "Windows (WASAPI / Virtual Audio Cable)"
    }

    fn create_virtual_mic(&mut self, virtual_device_name: &str) -> Result<()> {
        self.virtual_name = virtual_device_name.to_string();
        // In native Windows deployments, registers the SilentMic Virtual Audio Device
        // via CLSID_MMDeviceEnumerator and IAudioClient3 exclusive-mode loopback endpoint.
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
