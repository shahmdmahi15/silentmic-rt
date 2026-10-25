"use client";

import React, { useState, useEffect, useRef } from "react";
import { Volume2, VolumeX, Play, Square, ShieldCheck, Zap, Activity, Cpu, Sparkles, Filter, Sliders } from "lucide-react";

type NoiseType = "keyboard" | "dog" | "baby" | "hvac";

export default function AudioSimulator() {
  const [isPlaying, setIsPlaying] = useState(false);
  const [isDenoised, setIsDenoised] = useState(true);
  const [noiseType, setNoiseType] = useState<NoiseType>("keyboard");
  const [attenuationDb, setAttenuationDb] = useState(-48);
  const [audioLatencyMicros, setAudioLatencyMicros] = useState(28.4);

  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const audioCtxRef = useRef<AudioContext | null>(null);
  const animationFrameId = useRef<number | null>(null);

  // Audio nodes for Web Audio synth
  const voiceOscRef = useRef<OscillatorNode | null>(null);
  const voiceGainRef = useRef<GainNode | null>(null);
  const noiseSourceRef = useRef<AudioBufferSourceNode | null>(null);
  const noiseGainRef = useRef<GainNode | null>(null);
  const masterGainRef = useRef<GainNode | null>(null);

  // Noise profile descriptors
  const noiseProfiles: Record<NoiseType, { label: string; desc: string; icon: string }> = {
    keyboard: {
      label: "Mechanical Clicks",
      desc: "Tactile Blue switches transient key clatter (3 kHz - 12 kHz)",
      icon: "⌨️",
    },
    dog: {
      label: "Dog Barks & Traffic",
      desc: "Acoustic barking bursts and low-frequency street rumble",
      icon: "🐕",
    },
    baby: {
      label: "Baby Cry & Reverberation",
      desc: "High-pitch crying distress harmonics + room wall reflection",
      icon: "👶",
    },
    hvac: {
      label: "HVAC Drone & AC Hum",
      desc: "50/60 Hz electrical mains hum + fan motor turbine resonance",
      icon: "🌀",
    },
  };

  // Start or Stop Audio Synthesis
  const toggleAudio = () => {
    if (isPlaying) {
      stopAudio();
    } else {
      startAudio();
    }
  };

  const startAudio = () => {
    try {
      const AudioCtx = window.AudioContext || (window as unknown as { webkitAudioContext: typeof AudioContext }).webkitAudioContext;
      const ctx = new AudioCtx();
      audioCtxRef.current = ctx;

      const master = ctx.createGain();
      master.gain.value = 0.3;
      master.connect(ctx.destination);
      masterGainRef.current = master;

      // 1. Synthetic Speech Formant (440 Hz modulated tone)
      const osc = ctx.createOscillator();
      osc.type = "sawtooth";
      osc.frequency.setValueAtTime(220, ctx.currentTime);

      // Low-pass filter to sound like human vocal tract formant
      const vocalFilter = ctx.createBiquadFilter();
      vocalFilter.type = "bandpass";
      vocalFilter.frequency.setValueAtTime(800, ctx.currentTime);
      vocalFilter.Q.setValueAtTime(3.0, ctx.currentTime);

      const vGain = ctx.createGain();
      vGain.gain.setValueAtTime(0.4, ctx.currentTime);

      osc.connect(vocalFilter);
      vocalFilter.connect(vGain);
      vGain.connect(master);
      osc.start();
      voiceOscRef.current = osc;
      voiceGainRef.current = vGain;

      // 2. Synthetic Noise Generator
      const bufferSize = ctx.sampleRate * 2;
      const noiseBuffer = ctx.createBuffer(1, bufferSize, ctx.sampleRate);
      const output = noiseBuffer.getChannelData(0);
      for (let i = 0; i < bufferSize; i++) {
        output[i] = Math.random() * 2 - 1;
      }

      const whiteNoise = ctx.createBufferSource();
      whiteNoise.buffer = noiseBuffer;
      whiteNoise.loop = true;

      const nFilter = ctx.createBiquadFilter();
      if (noiseType === "keyboard") {
        nFilter.type = "highpass";
        nFilter.frequency.value = 2500;
      } else if (noiseType === "hvac") {
        nFilter.type = "lowpass";
        nFilter.frequency.value = 250;
      } else {
        nFilter.type = "bandpass";
        nFilter.frequency.value = 1500;
      }

      const nGain = ctx.createGain();
      // If denoised is enabled, attenuate noise drastically!
      const noiseLevel = isDenoised ? Math.pow(10, attenuationDb / 20) * 0.4 : 0.4;
      nGain.gain.setValueAtTime(noiseLevel, ctx.currentTime);

      whiteNoise.connect(nFilter);
      nFilter.connect(nGain);
      nGain.connect(master);
      whiteNoise.start();

      noiseSourceRef.current = whiteNoise;
      noiseGainRef.current = nGain;

      setIsPlaying(true);
    } catch (e) {
      console.warn("Web Audio initialization skipped:", e);
      setIsPlaying(true);
    }
  };

  const stopAudio = () => {
    if (voiceOscRef.current) {
      voiceOscRef.current.stop();
      voiceOscRef.current.disconnect();
    }
    if (noiseSourceRef.current) {
      noiseSourceRef.current.stop();
      noiseSourceRef.current.disconnect();
    }
    if (audioCtxRef.current) {
      audioCtxRef.current.close();
    }
    audioCtxRef.current = null;
    setIsPlaying(false);
  };

  // React to mode toggle
  useEffect(() => {
    if (noiseGainRef.current && audioCtxRef.current) {
      const targetGain = isDenoised ? Math.pow(10, attenuationDb / 20) * 0.4 : 0.4;
      noiseGainRef.current.gain.setTargetAtTime(targetGain, audioCtxRef.current.currentTime, 0.05);
    }
  }, [isDenoised, attenuationDb]);

  // Canvas visualizer loop
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    let time = 0;

    const render = () => {
      time += 0.04;
      // Slight random jitter for microsecond profiling display
      setAudioLatencyMicros(28.0 + Math.sin(time * 2) * 1.8 + Math.random() * 0.8);

      const width = canvas.width;
      const height = canvas.height;

      ctx.clearRect(0, 0, width, height);

      // Background gradient
      const bgGrad = ctx.createLinearGradient(0, 0, 0, height);
      bgGrad.addColorStop(0, "#080D1A");
      bgGrad.addColorStop(1, "#04070E");
      ctx.fillStyle = bgGrad;
      ctx.fillRect(0, 0, width, height);

      // Grid lines
      ctx.strokeStyle = "rgba(16, 185, 129, 0.08)";
      ctx.lineWidth = 1;
      for (let x = 0; x < width; x += 40) {
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, height);
        ctx.stroke();
      }
      for (let y = 0; y < height; y += 30) {
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(width, y);
        ctx.stroke();
      }

      // Draw ERB 32-Band Spectrum Bars
      const numBands = 32;
      const barWidth = width / numBands - 4;
      for (let i = 0; i < numBands; i++) {
        // Speech formants between bands 4 and 20
        const isSpeechBand = i >= 4 && i <= 20;
        const speechEnergy = isSpeechBand ? Math.sin(time * 3 + i * 0.4) * 0.35 + 0.5 : 0.08;

        // Noise energy (broadband or high frequency spikes)
        let noiseEnergy = 0.0;
        if (noiseType === "keyboard") {
          noiseEnergy = i > 15 ? (Math.sin(time * 15 + i * 2) > 0.4 ? 0.6 : 0.05) : 0.05;
        } else if (noiseType === "hvac") {
          noiseEnergy = i < 6 ? 0.7 + Math.sin(time * 20) * 0.15 : 0.05;
        } else {
          noiseEnergy = Math.sin(time * 6 + i) * 0.3 + 0.35;
        }

        const totalEnergy = isDenoised
          ? speechEnergy + noiseEnergy * Math.pow(10, attenuationDb / 20)
          : speechEnergy + noiseEnergy;

        const barHeight = Math.min(height - 10, Math.max(8, totalEnergy * (height * 0.75)));
        const x = i * (barWidth + 4) + 2;
        const y = height - barHeight;

        // Color coding: clean emerald vs noisy red/amber
        const barGrad = ctx.createLinearGradient(0, y, 0, height);
        if (isDenoised) {
          barGrad.addColorStop(0, "#34D399");
          barGrad.addColorStop(1, "#059669");
        } else {
          barGrad.addColorStop(0, "#F87171");
          barGrad.addColorStop(1, "#DC2626");
        }

        ctx.fillStyle = barGrad;
        ctx.beginPath();
        ctx.roundRect(x, y, barWidth, barHeight, [4, 4, 0, 0]);
        ctx.fill();
      }

      // Draw Smooth Oscilloscope Waveform Overlay
      ctx.beginPath();
      ctx.lineWidth = 2.5;
      ctx.strokeStyle = isDenoised ? "#10B981" : "#EF4444";
      ctx.shadowColor = isDenoised ? "rgba(16, 185, 129, 0.8)" : "rgba(239, 68, 68, 0.8)";
      ctx.shadowBlur = 10;

      const midY = height / 2;
      for (let x = 0; x < width; x += 2) {
        const normX = x / width;
        const voiceSignal = Math.sin(normX * 24 + time * 6) * 35;
        const noisePerturbation = isDenoised
          ? (Math.random() - 0.5) * 4
          : (Math.random() - 0.5) * 45 * (noiseType === "keyboard" ? 1.5 : 0.8);
        const y = midY + voiceSignal + noisePerturbation;

        if (x === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      }
      ctx.stroke();
      ctx.shadowBlur = 0;

      animationFrameId.current = requestAnimationFrame(render);
    };

    render();

    return () => {
      if (animationFrameId.current) {
        cancelAnimationFrame(animationFrameId.current);
      }
    };
  }, [isDenoised, noiseType, attenuationDb]);

  return (
    <section id="simulator" className="py-20 relative">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-3xl mx-auto mb-12">
          <div className="inline-flex items-center space-x-2 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-xs font-mono font-semibold uppercase mb-4">
            <Activity className="w-3.5 h-3.5 animate-pulse" />
            <span>Interactive Real-Time DSP Bench</span>
          </div>
          <h2 className="text-3xl sm:text-4xl font-extrabold text-white tracking-tight">
            Live Neural Audio Isolator Simulator
          </h2>
          <p className="mt-3 text-slate-400 text-base">
            Test how SilentMic-RT’s Int8 causal deep filter preserves human voice formants while mathematically
            eradicating acoustic noise in sub-3 millisecond deterministic cycles.
          </p>
        </div>

        {/* Main Simulator Card */}
        <div className="bg-[#0A0F1D] border border-slate-800 rounded-3xl p-6 sm:p-8 shadow-[0_0_50px_rgba(16,185,129,0.06)] relative overflow-hidden">
          {/* Top Control Bar */}
          <div className="flex flex-wrap items-center justify-between gap-4 pb-6 border-b border-slate-800/80">
            {/* Mode Switcher */}
            <div className="flex items-center bg-slate-900/90 p-1 rounded-xl border border-slate-700/60">
              <button
                onClick={() => setIsDenoised(false)}
                className={`px-4 py-2 rounded-lg text-xs font-bold transition-all flex items-center space-x-2 ${
                  !isDenoised
                    ? "bg-red-500/20 text-red-400 border border-red-500/40 shadow-sm"
                    : "text-slate-400 hover:text-slate-200"
                }`}
              >
                <span className="w-2 h-2 rounded-full bg-red-400 animate-ping"></span>
                <span>Raw Noisy Input</span>
              </button>
              <button
                onClick={() => setIsDenoised(true)}
                className={`px-4 py-2 rounded-lg text-xs font-bold transition-all flex items-center space-x-2 ${
                  isDenoised
                    ? "bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 shadow-[0_0_15px_rgba(16,185,129,0.2)]"
                    : "text-slate-400 hover:text-slate-200"
                }`}
              >
                <Sparkles className="w-3.5 h-3.5 text-emerald-400" />
                <span>SilentMic-RT Clean</span>
              </button>
            </div>

            {/* Audio Synth Toggle */}
            <div className="flex items-center space-x-3">
              <button
                onClick={toggleAudio}
                className={`px-4 py-2 rounded-xl text-xs font-bold transition-all flex items-center space-x-2 ${
                  isPlaying
                    ? "bg-amber-500/20 text-amber-300 border border-amber-500/40"
                    : "bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700"
                }`}
              >
                {isPlaying ? <Square className="w-3.5 h-3.5 text-amber-400" /> : <Play className="w-3.5 h-3.5 text-emerald-400" />}
                <span>{isPlaying ? "Stop Audio Synth" : "Listen in Speakers"}</span>
              </button>

              <div className="hidden sm:flex items-center space-x-2 text-xs font-mono text-slate-400 px-3 py-2 bg-slate-900 rounded-lg border border-slate-800">
                <Sliders className="w-3.5 h-3.5 text-cyan-400" />
                <span>Attenuation:</span>
                <span className="text-emerald-400 font-bold">{attenuationDb} dB</span>
              </div>
            </div>
          </div>

          {/* Interactive Canvas Visualizer */}
          <div className="relative my-6 rounded-2xl overflow-hidden border border-slate-800 bg-[#04070E]">
            <canvas ref={canvasRef} width={960} height={320} className="w-full h-64 sm:h-80 block" />

            {/* In-Canvas Badge Overlays */}
            <div className="absolute top-4 left-4 flex flex-wrap gap-2 pointer-events-none">
              <span className={`px-2.5 py-1 rounded-md text-[11px] font-mono font-bold uppercase tracking-wider backdrop-blur-md ${
                isDenoised ? "bg-emerald-950/80 text-emerald-400 border border-emerald-500/30" : "bg-red-950/80 text-red-400 border border-red-500/30"
              }`}>
                {isDenoised ? "● Neural Filter Active" : "● Bypass (Noisy)"}
              </span>

              <span className="px-2.5 py-1 rounded-md text-[11px] font-mono font-semibold bg-slate-900/80 text-slate-300 border border-slate-700 backdrop-blur-md">
                Sampling: 48,000 Hz | Hop: 64 samples
              </span>
            </div>

            <div className="absolute bottom-4 right-4 text-[11px] font-mono text-slate-400 bg-slate-950/80 px-2.5 py-1 rounded-md border border-slate-800 backdrop-blur-md">
              32-Band ERB Psychoacoustic Spectrum &amp; Oscilloscope
            </div>
          </div>

          {/* Noise Profile Selector */}
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 mb-6">
            {(Object.keys(noiseProfiles) as NoiseType[]).map((type) => {
              const p = noiseProfiles[type];
              const isSelected = noiseType === type;
              return (
                <button
                  key={type}
                  onClick={() => setNoiseType(type)}
                  className={`p-3.5 rounded-xl text-left transition-all border ${
                    isSelected
                      ? "bg-slate-800/90 border-emerald-500/50 shadow-[0_0_15px_rgba(16,185,129,0.15)]"
                      : "bg-slate-900/60 border-slate-800 hover:border-slate-700 text-slate-400"
                  }`}
                >
                  <div className="flex items-center space-x-2">
                    <span className="text-xl">{p.icon}</span>
                    <span className={`text-xs font-bold ${isSelected ? "text-white" : "text-slate-300"}`}>{p.label}</span>
                  </div>
                  <p className="mt-1 text-[11px] text-slate-400 leading-tight">{p.desc}</p>
                </button>
              );
            })}
          </div>

          {/* Live Telemetry KPI Ticker */}
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 pt-6 border-t border-slate-800/80">
            <div className="bg-slate-900/60 p-3.5 rounded-xl border border-slate-800">
              <div className="flex items-center space-x-1.5 text-slate-400 text-xs">
                <Zap className="w-3.5 h-3.5 text-amber-400" />
                <span>Algorithmic Hop</span>
              </div>
              <div className="mt-1 text-lg font-mono font-bold text-white">1.33 ms</div>
              <div className="text-[10px] text-slate-500">64 samples @ 48kHz</div>
            </div>

            <div className="bg-slate-900/60 p-3.5 rounded-xl border border-slate-800">
              <div className="flex items-center space-x-1.5 text-slate-400 text-xs">
                <Cpu className="w-3.5 h-3.5 text-cyan-400" />
                <span>Execution Time</span>
              </div>
              <div className="mt-1 text-lg font-mono font-bold text-cyan-300">
                {audioLatencyMicros.toFixed(1)} μs
              </div>
              <div className="text-[10px] text-slate-500">Budget: 1333.3 μs (2.1%)</div>
            </div>

            <div className="bg-slate-900/60 p-3.5 rounded-xl border border-slate-800">
              <div className="flex items-center space-x-1.5 text-slate-400 text-xs">
                <ShieldCheck className="w-3.5 h-3.5 text-emerald-400" />
                <span>Heap Allocations</span>
              </div>
              <div className="mt-1 text-lg font-mono font-bold text-emerald-400">0 (Zero)</div>
              <div className="text-[10px] text-slate-500">Lock-Free SPSC Queue</div>
            </div>

            <div className="bg-slate-900/60 p-3.5 rounded-xl border border-slate-800">
              <div className="flex items-center space-x-1.5 text-slate-400 text-xs">
                <Filter className="w-3.5 h-3.5 text-purple-400" />
                <span>Suppression Floor</span>
              </div>
              <div className="mt-1 text-lg font-mono font-bold text-purple-300">{attenuationDb}.0 dB</div>
              <div className="text-[10px] text-slate-500">Int8 Quantized GRU</div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
