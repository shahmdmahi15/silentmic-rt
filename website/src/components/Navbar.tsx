"use client";

import React, { useState } from "react";
import Image from "next/image";
import Link from "next/link";
import { Mic, Terminal, Activity, Layers, User, ExternalLink, Menu, X } from "lucide-react";
import { GithubIcon } from "@/components/Icons";

export default function Navbar() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  return (
    <nav className="sticky top-0 z-50 backdrop-blur-xl bg-[#060913]/80 border-b border-emerald-500/10">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          {/* Brand Logo */}
          <Link href="/" className="flex items-center space-x-3 group">
            <div className="relative w-9 h-9 rounded-xl overflow-hidden bg-emerald-950/40 border border-emerald-500/30 flex items-center justify-center p-1.5 shadow-[0_0_15px_rgba(16,185,129,0.2)]">
              <Mic className="w-5 h-5 text-emerald-400 group-hover:scale-110 transition-transform" />
            </div>
            <div className="flex flex-col">
              <span className="text-lg font-bold tracking-tight text-white flex items-center">
                SilentMic<span className="text-emerald-400 font-extrabold">-RT</span>
                <span className="ml-2 px-1.5 py-0.5 text-[10px] uppercase font-mono font-bold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 rounded-md">
                  v0.1.0
                </span>
              </span>
              <span className="text-[10px] text-slate-400 font-mono tracking-wider">Sub-3ms Real-Time Voice Isolator</span>
            </div>
          </Link>

          {/* Desktop Nav Links */}
          <div className="hidden md:flex items-center space-x-6 text-sm font-medium text-slate-300">
            <a href="#simulator" className="hover:text-emerald-400 transition-colors flex items-center space-x-1.5">
              <Activity className="w-4 h-4 text-emerald-400" />
              <span>Live Simulator</span>
            </a>
            <a href="#benchmarks" className="hover:text-emerald-400 transition-colors flex items-center space-x-1.5">
              <Activity className="w-4 h-4 text-cyan-400" />
              <span>Benchmarks</span>
            </a>
            <a href="#architecture" className="hover:text-emerald-400 transition-colors flex items-center space-x-1.5">
              <Layers className="w-4 h-4 text-purple-400" />
              <span>DSP Architecture</span>
            </a>
            <a href="#api" className="hover:text-emerald-400 transition-colors flex items-center space-x-1.5">
              <Terminal className="w-4 h-4 text-amber-400" />
              <span>API Explorer</span>
            </a>
            <a href="#author" className="hover:text-emerald-400 transition-colors flex items-center space-x-1.5">
              <User className="w-4 h-4 text-pink-400" />
              <span>Author</span>
            </a>
          </div>

          {/* Action CTAs */}
          <div className="hidden md:flex items-center space-x-3">
            <a
              href="https://github.com/shahmdmahi15/silentmic-rt"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center space-x-2 px-3.5 py-1.5 rounded-lg bg-slate-900 border border-slate-700 hover:border-emerald-500/50 hover:bg-slate-800 text-slate-200 text-xs font-semibold transition-all shadow-sm"
            >
              <GithubIcon className="w-4 h-4" />
              <span>GitHub</span>
              <ExternalLink className="w-3 h-3 text-slate-400" />
            </a>

            <a
              href="#simulator"
              className="inline-flex items-center space-x-2 px-3.5 py-1.5 rounded-lg bg-gradient-to-r from-emerald-500 to-teal-500 hover:from-emerald-400 hover:to-teal-400 text-slate-950 text-xs font-bold transition-all shadow-[0_0_15px_rgba(16,185,129,0.3)]"
            >
              <Mic className="w-3.5 h-3.5" />
              <span>Try Live Demo</span>
            </a>
          </div>

          {/* Mobile Menu Button */}
          <div className="md:hidden flex items-center">
            <button
              onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
              className="p-2 rounded-lg text-slate-400 hover:text-white hover:bg-slate-800 focus:outline-none"
            >
              {mobileMenuOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
            </button>
          </div>
        </div>
      </div>

      {/* Mobile dropdown */}
      {mobileMenuOpen && (
        <div className="md:hidden bg-[#090e1a] border-b border-slate-800 px-4 pt-2 pb-4 space-y-3">
          <a
            href="#simulator"
            onClick={() => setMobileMenuOpen(false)}
            className="block px-3 py-2 rounded-md text-base font-medium text-slate-200 hover:bg-slate-800 hover:text-emerald-400"
          >
            Live Simulator
          </a>
          <a
            href="#benchmarks"
            onClick={() => setMobileMenuOpen(false)}
            className="block px-3 py-2 rounded-md text-base font-medium text-slate-200 hover:bg-slate-800 hover:text-emerald-400"
          >
            Benchmarks
          </a>
          <a
            href="#architecture"
            onClick={() => setMobileMenuOpen(false)}
            className="block px-3 py-2 rounded-md text-base font-medium text-slate-200 hover:bg-slate-800 hover:text-emerald-400"
          >
            DSP Architecture
          </a>
          <a
            href="#api"
            onClick={() => setMobileMenuOpen(false)}
            className="block px-3 py-2 rounded-md text-base font-medium text-slate-200 hover:bg-slate-800 hover:text-emerald-400"
          >
            API Explorer
          </a>
          <a
            href="#author"
            onClick={() => setMobileMenuOpen(false)}
            className="block px-3 py-2 rounded-md text-base font-medium text-slate-200 hover:bg-slate-800 hover:text-emerald-400"
          >
            Author Profile
          </a>
        </div>
      )}
    </nav>
  );
}
