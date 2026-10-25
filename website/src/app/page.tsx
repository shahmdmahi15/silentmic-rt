"use client";

import React, { useState } from "react";
import Navbar from "@/components/Navbar";
import AudioSimulator from "@/components/AudioSimulator";
import BenchmarkSection from "@/components/BenchmarkSection";
import ArchitectureSection from "@/components/ArchitectureSection";
import ApiExplorer from "@/components/ApiExplorer";
import AuthorSection from "@/components/AuthorSection";
import Footer from "@/components/Footer";
import { Mic, Zap, Shield, Sparkles, Terminal, Copy, Check, ArrowRight, ExternalLink, Cpu } from "lucide-react";
import { GithubIcon } from "@/components/Icons";

export default function Home() {
  const [copiedCmd, setCopiedCmd] = useState(false);

  const copyCommand = () => {
    navigator.clipboard.writeText("cargo install silentmic-rt && silentmic-rt run");
    setCopiedCmd(true);
    setTimeout(() => setCopiedCmd(false), 2000);
  };

  return (
    <div className="min-h-screen bg-[#060913] text-slate-100 selection:bg-emerald-500/30 selection:text-emerald-300">
      <Navbar />

      <main>
        {/* Hero Section */}
        <section className="relative pt-24 pb-20 overflow-hidden">
          {/* Subtle Ambient Neon Background Glow */}
          <div className="absolute top-1/4 left-1/2 -translate-x-1/2 w-[600px] h-[350px] bg-gradient-to-r from-emerald-500/15 via-teal-500/15 to-cyan-500/15 rounded-full blur-[120px] pointer-events-none"></div>

          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10 text-center">
            {/* Eyebrow badge */}
            <div className="inline-flex items-center space-x-2 px-3.5 py-1.5 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-emerald-300 text-xs font-mono font-semibold uppercase mb-8 shadow-[0_0_20px_rgba(16,185,129,0.15)]">
              <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
              <span>Sub-3ms Deterministic Hard Real-Time Voice Isolator</span>
            </div>

            {/* Main Headline */}
            <h1 className="text-4xl sm:text-6xl lg:text-7xl font-extrabold tracking-tight text-white max-w-5xl mx-auto leading-tight">
              Eliminate Room Noise in{" "}
              <span className="text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 via-teal-300 to-cyan-400">
                Sub-3 Milliseconds
              </span>
              .
            </h1>

            {/* Sub-headline */}
            <p className="mt-6 text-lg sm:text-xl text-slate-400 max-w-3xl mx-auto leading-relaxed">
              Zero cloud transmission. Zero heap allocation. Intercepts physical microphone audio directly at the kernel driver layer to eradicate screaming babies, keyboard clicks, barking dogs, and room echo offline on bare-metal CPU SIMD.
            </p>

            {/* Quick CTAs */}
            <div className="mt-10 flex flex-wrap items-center justify-center gap-4">
              <a
                href="#simulator"
                className="inline-flex items-center space-x-2 px-6 py-3.5 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-500 hover:from-emerald-400 hover:to-teal-400 text-slate-950 font-bold text-sm transition-all shadow-[0_0_30px_rgba(16,185,129,0.3)] hover:scale-105"
              >
                <Sparkles className="w-4 h-4" />
                <span>Launch Audio Simulator</span>
                <ArrowRight className="w-4 h-4 ml-1" />
              </a>

              <a
                href="https://github.com/shahmdmahi15/silentmic-rt"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center space-x-2 px-6 py-3.5 rounded-xl bg-slate-900/90 hover:bg-slate-800 text-slate-200 border border-slate-700 hover:border-slate-600 font-semibold text-sm transition-all shadow-md"
              >
                <GithubIcon className="w-4 h-4" />
                <span>Explore Source Code</span>
                <ExternalLink className="w-3.5 h-3.5 text-slate-400 ml-1" />
              </a>
            </div>

            {/* Copyable Quick Install CLI Pill */}
            <div className="mt-8 inline-flex items-center space-x-3 px-4 py-2 rounded-xl bg-slate-900/80 border border-slate-800 text-xs font-mono text-slate-300 backdrop-blur-md">
              <Terminal className="w-4 h-4 text-emerald-400" />
              <span>cargo install silentmic-rt &amp;&amp; silentmic-rt run</span>
              <button
                onClick={copyCommand}
                className="p-1 rounded hover:bg-slate-800 text-slate-400 hover:text-white transition-colors"
                title="Copy Command"
              >
                {copiedCmd ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
              </button>
            </div>

            {/* Core Feature Metric Badges */}
            <div className="mt-16 grid grid-cols-2 md:grid-cols-4 gap-4 max-w-4xl mx-auto text-left">
              <div className="p-4 rounded-2xl bg-[#090E1B] border border-slate-800/80">
                <div className="text-xs font-mono text-slate-400 flex items-center space-x-1.5">
                  <Zap className="w-3.5 h-3.5 text-amber-400" />
                  <span>Algorithmic Hop</span>
                </div>
                <div className="mt-1 text-2xl font-mono font-bold text-white">1.33 ms</div>
                <div className="text-[11px] text-slate-500">64 samples @ 48,000 Hz</div>
              </div>

              <div className="p-4 rounded-2xl bg-[#090E1B] border border-slate-800/80">
                <div className="text-xs font-mono text-slate-400 flex items-center space-x-1.5">
                  <Shield className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Audio Allocations</span>
                </div>
                <div className="mt-1 text-2xl font-mono font-bold text-emerald-400">0 (Zero)</div>
                <div className="text-[11px] text-slate-500">Lock-Free SPSC Ring Buffer</div>
              </div>

              <div className="p-4 rounded-2xl bg-[#090E1B] border border-slate-800/80">
                <div className="text-xs font-mono text-slate-400 flex items-center space-x-1.5">
                  <Cpu className="w-3.5 h-3.5 text-cyan-400" />
                  <span>Compute Budget</span>
                </div>
                <div className="mt-1 text-2xl font-mono font-bold text-cyan-300">28.4 μs</div>
                <div className="text-[11px] text-slate-500">2.1% of 1333.3 μs budget</div>
              </div>

              <div className="p-4 rounded-2xl bg-[#090E1B] border border-slate-800/80">
                <div className="text-xs font-mono text-slate-400 flex items-center space-x-1.5">
                  <Sparkles className="w-3.5 h-3.5 text-purple-400" />
                  <span>Auditory Filterbank</span>
                </div>
                <div className="mt-1 text-2xl font-mono font-bold text-purple-300">32-Band</div>
                <div className="text-[11px] text-slate-500">Glasberg &amp; Moore ERB Scale</div>
              </div>
            </div>
          </div>
        </section>

        {/* Interactive Audio Simulator Component */}
        <AudioSimulator />

        {/* Industrial Benchmark Section */}
        <BenchmarkSection />

        {/* DSP & Neural Architecture Section */}
        <ArchitectureSection />

        {/* API & Code Integration Explorer */}
        <ApiExplorer />

        {/* Prominent Author Profile Section */}
        <AuthorSection />
      </main>

      <Footer />
    </div>
  );
}
