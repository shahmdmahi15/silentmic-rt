"use client";

import React, { useState } from "react";
import { Layers, Cpu, Waves, ShieldCheck, Binary, ChevronRight, Zap } from "lucide-react";

export default function ArchitectureSection() {
  const [activeStep, setActiveStep] = useState(2);

  const steps = [
    {
      id: 0,
      title: "1. SPSC Ring Buffer",
      subtitle: "Zero-Allocation Audio Ingest",
      icon: <ShieldCheck className="w-5 h-5 text-emerald-400" />,
      tag: "Kernel Layer",
      timing: "0.1 μs",
      formula: "\\text{Head} \\leftarrow (\\text{Head} + 1) \\pmod C",
      desc: "Single-Producer Single-Consumer lock-free ring buffer aligned to 64-byte CPU cachelines. Decouples the OS audio driver callback thread from the neural DSP processor with zero locks and zero allocations.",
    },
    {
      id: 1,
      title: "2. SIMD STFT & Sine Window",
      subtitle: "Spectral Analysis (65 Complex Bins)",
      icon: <Waves className="w-5 h-5 text-cyan-400" />,
      tag: "DSP Core",
      timing: "4.2 μs",
      formula: "w[n] = \\sin\\left(\\frac{\\pi(n + 0.5)}{N}\\right), \\quad w^2[n] + w^2[n+H] = 1.0",
      desc: "Applies symmetric Sine analysis windowing to 128-sample frames with 50% overlap (64-sample hop = 1.33ms). Real-to-Complex FFT generates 65 complex frequency bins with strictly preserved DC and Nyquist real symmetries.",
    },
    {
      id: 2,
      title: "3. 32-Band ERB Filterbank",
      subtitle: "Psychoacoustic Auditory Feature Extraction",
      icon: <Binary className="w-5 h-5 text-purple-400" />,
      tag: "Auditory Model",
      timing: "2.8 μs",
      formula: "E(f) = 21.4 \\cdot \\log_{10}(0.00437 f + 1.0)",
      desc: "Applies Glasberg & Moore (1990) Equivalent Rectangular Bandwidth scaling across 50 Hz to 20,000 Hz. Compressively maps 65 FFT bins into 32 biological auditory excitation bands, capturing speech formant resonance.",
    },
    {
      id: 3,
      title: "4. Int8 Causal Deep Filter",
      subtitle: "Causal Conv1D + 2-Layer Recurrent GRU",
      icon: <Cpu className="w-5 h-5 text-amber-400" />,
      tag: "Neural Engine",
      timing: "16.5 μs",
      formula: "h_t = (1 - z_t) \\odot n_t + z_t \\odot h_{t-1}",
      desc: "Strictly causal 1D convolution over temporal history followed by a 2-layer causal Gated Recurrent Unit (hidden size 64). Operates on quantized int8 weights with pre-calibrated speech and transient noise discriminating priors.",
    },
    {
      id: 4,
      title: "5. Complex Ratio Masking (cIRM)",
      subtitle: "Phase & Magnitude Speech Isolation",
      icon: <Zap className="w-5 h-5 text-pink-400" />,
      tag: "Synthesis",
      timing: "4.8 μs",
      formula: "S(f, t) = (M_r + i M_i)(Y_r + i Y_i)",
      desc: "Dual-head predictor yields continuous ERB magnitude attenuation gains and fine complex phase rotations ($M_r + i M_i$). Overlap-add inverse FFT reconstructs the continuous clean speech time-domain audio stream.",
    },
  ];

  return (
    <section id="architecture" className="py-20 relative">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-3xl mx-auto mb-16">
          <div className="inline-flex items-center space-x-2 px-3 py-1 rounded-full bg-purple-500/10 border border-purple-500/20 text-purple-400 text-xs font-mono font-semibold uppercase mb-4">
            <Layers className="w-3.5 h-3.5" />
            <span>Mathematical &amp; DSP Foundations</span>
          </div>
          <h2 className="text-3xl sm:text-4xl font-extrabold text-white tracking-tight">
            How SilentMic-RT Operates in 28.4 Microseconds
          </h2>
          <p className="mt-3 text-slate-400 text-base">
            Every pipeline stage is engineered with strict deterministic upper bounds. Click any module below to inspect its mathematical derivation and execution cycle budget.
          </p>
        </div>

        {/* Pipeline Navigation Steps */}
        <div className="grid grid-cols-1 sm:grid-cols-5 gap-2 mb-8 bg-[#0A0F1D] p-2 rounded-2xl border border-slate-800">
          {steps.map((s) => (
            <button
              key={s.id}
              onClick={() => setActiveStep(s.id)}
              className={`p-3 rounded-xl text-left transition-all flex flex-col justify-between ${
                activeStep === s.id
                  ? "bg-slate-800 border border-emerald-500/40 text-white shadow-lg"
                  : "hover:bg-slate-900/60 text-slate-400 border border-transparent"
              }`}
            >
              <div className="flex items-center justify-between">
                {s.icon}
                <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-slate-900 border border-slate-700 text-slate-300">
                  {s.timing}
                </span>
              </div>
              <div className="mt-3">
                <div className="text-xs font-bold leading-snug">{s.title}</div>
                <div className="text-[10px] text-slate-500 truncate">{s.tag}</div>
              </div>
            </button>
          ))}
        </div>

        {/* Selected Step Deep Dive Card */}
        {steps[activeStep] && (
          <div className="bg-gradient-to-br from-[#0B1021] to-[#060914] border border-slate-800 rounded-3xl p-6 sm:p-10 shadow-2xl relative overflow-hidden">
            <div className="flex flex-col lg:flex-row gap-8 items-start justify-between">
              <div className="max-w-2xl">
                <div className="flex items-center space-x-3 mb-3">
                  <span className="px-3 py-1 rounded-full bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 text-xs font-mono font-bold">
                    {steps[activeStep].tag}
                  </span>
                  <span className="text-xs font-mono text-slate-400">
                    Cycle Budget: <strong className="text-white">{steps[activeStep].timing}</strong>
                  </span>
                </div>

                <h3 className="text-2xl font-bold text-white tracking-tight">{steps[activeStep].title}</h3>
                <p className="text-sm font-medium text-emerald-400 mt-1">{steps[activeStep].subtitle}</p>

                <p className="mt-4 text-slate-300 text-sm leading-relaxed">{steps[activeStep].desc}</p>
              </div>

              {/* Mathematical Formula Display */}
              <div className="w-full lg:w-96 bg-slate-950 p-6 rounded-2xl border border-slate-800 flex flex-col justify-center">
                <span className="text-xs font-mono uppercase text-slate-500 tracking-wider mb-2">
                  Governing Formulation
                </span>
                <div className="p-4 rounded-xl bg-slate-900/80 border border-slate-800 font-mono text-sm text-cyan-300 overflow-x-auto">
                  <code>{steps[activeStep].formula}</code>
                </div>
                <span className="text-[11px] text-slate-500 mt-3 font-mono">
                  Compiled with Fat LTO, AVX-512 &amp; ARM NEON SIMD vectorization.
                </span>
              </div>
            </div>
          </div>
        )}
      </div>
    </section>
  );
}
