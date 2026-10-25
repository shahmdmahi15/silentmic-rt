"use client";

import React, { useState } from "react";
import { Terminal, Copy, Check, Code2, Cpu } from "lucide-react";

export default function ApiExplorer() {
  const [activeTab, setActiveTab] = useState<"rust" | "python" | "c" | "cli">("rust");
  const [copied, setCopied] = useState(false);

  const snippets = {
    rust: `// Cargo.toml: silentmic-rt = "0.1.0"
use silentmic::{SilentMicEngine, HOP_SIZE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SilentMicEngine::new();
    engine.set_attenuation_db(-48.0); // -48 dB suppression floor

    let mut in_frame = [0.0f32; HOP_SIZE];  // 64 samples @ 48kHz
    let mut out_frame = [0.0f32; HOP_SIZE];

    // Inside real-time audio thread (Zero Dynamic Allocations!)
    engine.process_frame(&in_frame, &mut out_frame);

    let stats = engine.stats();
    println!("Frame Latency: {:.2} us (Budget: {:.1}%)", 
        stats.last_frame_micros, stats.cpu_budget_pct);

    Ok(())
}`,
    python: `# pip install pysilentmic
import numpy as np
from pysilentmic import SilentMic

# Initialize 48kHz real-time neural isolator
isolator = SilentMic(sample_rate=48000, attenuation_db=-48.0)

# Process 64-sample frame in real-time (< 30 us)
frame = np.random.randn(64).astype(np.float32)
clean_frame = isolator.process_frame(frame)

# Denoise complete NumPy audio stream
noisy_stream = np.random.randn(48000 * 5).astype(np.float32)
pristine_voice = isolator.process(noisy_stream)

print(isolator.get_stats())`,
    c: `// gcc app.c -L./target/release -lsilentmic -o app
#include <stdio.h>
#include "silentmic.h"

int main() {
    printf("SilentMic-RT Version: %s\\n", silentmic_version());

    silentmic_t engine = silentmic_create(48000);
    silentmic_set_attenuation_db(engine, -48.0f);

    float in_frame[SILENTMIC_FRAME_SIZE];
    float out_frame[SILENTMIC_FRAME_SIZE];

    // Real-time audio loop callback
    int res = silentmic_process_frame(engine, in_frame, out_frame);

    silentmic_destroy(engine);
    return 0;
}`,
    cli: `# 1. Run real-time background virtual microphone daemon
silentmic-rt run --input "default" --output "SilentMic_Virtual_Mic" --attenuation-db -48.0

# 2. Offline batch denoise WAV audio file
silentmic-rt process --input noisy_podcast.wav --out clean_podcast.wav

# 3. High-precision hardware cycle latency benchmark
silentmic-rt bench --duration 5.0`,
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(snippets[activeTab]);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <section id="api" className="py-20 bg-[#050811]/60 border-t border-slate-900 relative">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center max-w-3xl mx-auto mb-16">
          <div className="inline-flex items-center space-x-2 px-3 py-1 rounded-full bg-amber-500/10 border border-amber-500/20 text-amber-400 text-xs font-mono font-semibold uppercase mb-4">
            <Terminal className="w-3.5 h-3.5" />
            <span>Multi-Language Integration</span>
          </div>
          <h2 className="text-3xl sm:text-4xl font-extrabold text-white tracking-tight">
            Developer Friendly. Built for Production.
          </h2>
          <p className="mt-3 text-slate-400 text-base">
            Embed SilentMic-RT directly into Rust engines, Python ML pipelines, C/C++ media servers, or use the instant CLI daemon.
          </p>
        </div>

        {/* Code Card */}
        <div className="bg-[#0A0F1D] border border-slate-800 rounded-3xl overflow-hidden shadow-2xl">
          {/* Header Bar */}
          <div className="flex items-center justify-between px-6 py-4 border-b border-slate-800 bg-slate-900/60">
            <div className="flex items-center space-x-2">
              <button
                onClick={() => setActiveTab("rust")}
                className={`px-3 py-1.5 rounded-lg text-xs font-mono font-bold transition-all ${
                  activeTab === "rust" ? "bg-emerald-500/20 text-emerald-400 border border-emerald-500/30" : "text-slate-400 hover:text-white"
                }`}
              >
                Rust Crate
              </button>
              <button
                onClick={() => setActiveTab("python")}
                className={`px-3 py-1.5 rounded-lg text-xs font-mono font-bold transition-all ${
                  activeTab === "python" ? "bg-cyan-500/20 text-cyan-400 border border-cyan-500/30" : "text-slate-400 hover:text-white"
                }`}
              >
                Python (pysilentmic)
              </button>
              <button
                onClick={() => setActiveTab("c")}
                className={`px-3 py-1.5 rounded-lg text-xs font-mono font-bold transition-all ${
                  activeTab === "c" ? "bg-purple-500/20 text-purple-400 border border-purple-500/30" : "text-slate-400 hover:text-white"
                }`}
              >
                C99 ABI Header
              </button>
              <button
                onClick={() => setActiveTab("cli")}
                className={`px-3 py-1.5 rounded-lg text-xs font-mono font-bold transition-all ${
                  activeTab === "cli" ? "bg-amber-500/20 text-amber-400 border border-amber-500/30" : "text-slate-400 hover:text-white"
                }`}
              >
                Standalone CLI
              </button>
            </div>

            <button
              onClick={handleCopy}
              className="inline-flex items-center space-x-1.5 text-xs text-slate-400 hover:text-white bg-slate-800/80 hover:bg-slate-800 px-3 py-1.5 rounded-md border border-slate-700 transition-colors"
            >
              {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
              <span>{copied ? "Copied!" : "Copy Code"}</span>
            </button>
          </div>

          {/* Syntax Code Box */}
          <div className="p-6 bg-[#050811] overflow-x-auto font-mono text-sm leading-relaxed text-slate-300">
            <pre>
              <code>{snippets[activeTab]}</code>
            </pre>
          </div>
        </div>
      </div>
    </section>
  );
}
