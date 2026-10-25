"use client";

import React from "react";
import Link from "next/link";
import { Mic, Heart } from "lucide-react";
import { GithubIcon } from "@/components/Icons";

export default function Footer() {
  return (
    <footer className="border-t border-slate-900 bg-[#04070F] text-slate-400 py-12 text-sm">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex flex-col md:flex-row items-center justify-between gap-6">
          {/* Brand */}
          <div className="flex items-center space-x-3">
            <div className="w-8 h-8 rounded-lg bg-emerald-950/40 border border-emerald-500/30 flex items-center justify-center p-1">
              <Mic className="w-4 h-4 text-emerald-400" />
            </div>
            <div>
              <span className="font-bold text-white tracking-tight">SilentMic-RT</span>
              <p className="text-xs text-slate-500 font-mono">Deterministic Sub-3ms Neural Voice Isolator</p>
            </div>
          </div>

          {/* Quick Nav */}
          <div className="flex items-center space-x-6 text-xs font-medium text-slate-400">
            <a href="#simulator" className="hover:text-emerald-400 transition-colors">Simulator</a>
            <a href="#benchmarks" className="hover:text-emerald-400 transition-colors">Benchmarks</a>
            <a href="#architecture" className="hover:text-emerald-400 transition-colors">DSP Architecture</a>
            <a href="#api" className="hover:text-emerald-400 transition-colors">API Explorer</a>
            <a href="https://github.com/shahmdmahi15/silentmic-rt" target="_blank" rel="noopener noreferrer" className="hover:text-emerald-400 transition-colors flex items-center space-x-1">
              <GithubIcon className="w-3.5 h-3.5" />
              <span>GitHub</span>
            </a>
          </div>

          {/* Copyright & Author */}
          <div className="text-center md:text-right text-xs text-slate-500">
            <p>
              Designed &amp; Engineered by{" "}
              <a href="https://shahmdmahi.dpdns.org" target="_blank" rel="noopener noreferrer" className="text-emerald-400 hover:underline font-semibold">
                Shah Md. Mahi
              </a>
            </p>
            <p className="mt-1 font-mono text-[11px]">MIT License • 100% Offline Real-Time Audio</p>
          </div>
        </div>
      </div>
    </footer>
  );
}
