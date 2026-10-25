//! PipeWire and PulseAudio virtual audio device driver for Linux.

use std::process::Command;
use crate::error::Result;
use super::VirtualAudioDriver;

/// Linux PipeWire / PulseAudio Virtual Microphone Bridge.
pub struct PipeWireDriver {
    virtual_name: String,
    sink_module_id: Option<String>,
    source_module_id: Option<String>,
    is_active: bool,
}

impl PipeWireDriver {
    pub fn new() -> Self {
        Self {
            virtual_name: "SilentMic_Virtual_Mic".to_string(),
            sink_module_id: None,
            source_module_id: None,
            is_active: false,
        }
    }
}

impl Default for PipeWireDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualAudioDriver for PipeWireDriver {
    fn platform(&self) -> &'static str {
        "Linux (PipeWire / PulseAudio)"
    }

    fn create_virtual_mic(&mut self, virtual_device_name: &str) -> Result<()> {
        self.virtual_name = virtual_device_name.to_string();

        // 1. Create null-sink: pactl load-module module-null-sink sink_name=SilentMic_Sink ...
        let sink_name = format!("{}_Sink", self.virtual_name);
        let sink_desc = format!("device.description=\"SilentMic Virtual Sink\"");

        let sink_output = Command::new("pactl")
            .args(["load-module", "module-null-sink", &format!("sink_name={sink_name}"), &sink_desc])
            .output();

        if let Ok(out) = sink_output {
            if out.status.success() {
                self.sink_module_id = Some(String::from_utf8_lossy(&out.stdout).trim().to_string());
            }
        }

        // 2. Create remap-source from null-sink.monitor to appear as a physical microphone in Zoom/Discord
        let remap_desc = format!("device.description=\"SilentMic Neural Virtual Microphone\"");
        let remap_output = Command::new("pactl")
            .args([
                "load-module",
                "module-remap-source",
                &format!("source_name={}", self.virtual_name),
                &format!("master={sink_name}.monitor"),
                &remap_desc,
            ])
            .output();

        if let Ok(out) = remap_output {
            if out.status.success() {
                self.source_module_id = Some(String::from_utf8_lossy(&out.stdout).trim().to_string());
            }
        }

        self.is_active = true;
        Ok(())
    }

    fn remove_virtual_mic(&mut self) -> Result<()> {
        if let Some(id) = self.source_module_id.take() {
            let _ = Command::new("pactl").args(["unload-module", &id]).output();
        }
        if let Some(id) = self.sink_module_id.take() {
            let _ = Command::new("pactl").args(["unload-module", &id]).output();
        }
        self.is_active = false;
        Ok(())
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}

impl Drop for PipeWireDriver {
    fn drop(&mut self) {
        let _ = self.remove_virtual_mic();
    }
}
