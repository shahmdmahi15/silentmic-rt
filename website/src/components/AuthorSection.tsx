"use client";

import React from "react";
import Image from "next/image";
import { Mail, Phone, Globe, MessageSquare, Send, ExternalLink, ShieldCheck, Award, Heart } from "lucide-react";
import { GithubIcon, LinkedinIcon } from "@/components/Icons";

export default function AuthorSection() {
  const socialLinks = [
    {
      name: "Personal Website",
      url: "https://shahmdmahi.dpdns.org",
      label: "shahmdmahi.dpdns.org",
      icon: <Globe className="w-4 h-4 text-emerald-400" />,
    },
    {
      name: "GitHub",
      url: "https://github.com/shahmdmahi15",
      label: "@shahmdmahi15",
      icon: <GithubIcon className="w-4 h-4 text-slate-200" />,
    },
    {
      name: "LinkedIn",
      url: "https://linkedin.com/in/shahmdmahi27",
      label: "shahmdmahi27",
      icon: <LinkedinIcon className="w-4 h-4 text-blue-400" />,
    },
    {
      name: "Telegram",
      url: "https://t.me/shahmdmahi",
      label: "@shahmdmahi",
      icon: <Send className="w-4 h-4 text-cyan-400" />,
    },
    {
      name: "WhatsApp",
      url: "https://wa.me/8801757290258",
      label: "+8801757290258",
      icon: <MessageSquare className="w-4 h-4 text-emerald-400" />,
    },
    {
      name: "Email",
      url: "mailto:shahmdmahi15@gmail.com",
      label: "shahmdmahi15@gmail.com",
      icon: <Mail className="w-4 h-4 text-rose-400" />,
    },
    {
      name: "Discord",
      url: "https://discord.com",
      label: "shahmdmahi27",
      icon: <Award className="w-4 h-4 text-indigo-400" />,
    },
    {
      name: "Phone",
      url: "tel:+8801757290258",
      label: "+8801757290258",
      icon: <Phone className="w-4 h-4 text-teal-400" />,
    },
  ];

  return (
    <section id="author" className="py-20 relative">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="bg-gradient-to-br from-[#090F1E] to-[#04070F] border border-slate-800 rounded-3xl p-8 sm:p-12 shadow-2xl relative overflow-hidden">
          {/* Subtle Glow */}
          <div className="absolute top-0 right-0 w-96 h-96 bg-emerald-500/10 rounded-full blur-3xl pointer-events-none"></div>

          <div className="flex flex-col lg:flex-row items-center gap-10">
            {/* Author Avatar & Status */}
            <div className="flex flex-col items-center text-center">
              <div className="relative w-36 h-36 rounded-full overflow-hidden border-2 border-emerald-500/40 p-1 bg-gradient-to-tr from-emerald-500/20 to-teal-500/20 shadow-[0_0_30px_rgba(16,185,129,0.2)]">
                {/* Fallback avatar with image */}
                <img
                  src="https://github.com/shahmdmahi15.png"
                  alt="Shah Md. Mahi"
                  className="w-full h-full object-cover rounded-full"
                />
              </div>

              <div className="mt-4">
                <h3 className="text-xl font-bold text-white flex items-center justify-center space-x-1.5">
                  <span>Shah Md. Mahi</span>
                  <ShieldCheck className="w-4 h-4 text-emerald-400" />
                </h3>
                <p className="text-xs font-mono text-emerald-400 mt-0.5">Architect &amp; Lead Systems Engineer</p>
                <div className="mt-2 inline-flex items-center space-x-1 px-2.5 py-0.5 rounded-full bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 text-[11px] font-mono">
                  <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                  <span>Available for Engineering Collaboration</span>
                </div>
              </div>
            </div>

            {/* Author Bio & Social Matrix */}
            <div className="flex-1">
              <div className="border-b border-slate-800/80 pb-6 mb-6">
                <h4 className="text-lg font-bold text-white tracking-tight">
                  About the Creator
                </h4>
                <p className="mt-2 text-slate-300 text-sm leading-relaxed">
                  Shah Md. Mahi is a full-spectrum systems and audio DSP engineer specializing in zero-allocation hard real-time architectures, low-level Linux/POSIX subsystems, SIMD acceleration, and offline neural inference. He created SilentMic-RT to eliminate audio latency barriers for content creators, streamers, and developers worldwide.
                </p>
              </div>

              {/* Social Link Badges */}
              <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-3">
                {socialLinks.map((link) => (
                  <a
                    key={link.name}
                    href={link.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="flex items-center space-x-2.5 p-2.5 rounded-xl bg-slate-900/80 hover:bg-slate-850 border border-slate-800 hover:border-emerald-500/40 transition-all text-xs group shadow-sm"
                  >
                    <div className="p-1 rounded-lg bg-slate-800 border border-slate-700/60 group-hover:scale-110 transition-transform">
                      {link.icon}
                    </div>
                    <div className="flex flex-col overflow-hidden">
                      <span className="text-[10px] text-slate-400">{link.name}</span>
                      <span className="font-semibold text-white group-hover:text-emerald-400 truncate">
                        {link.label}
                      </span>
                    </div>
                  </a>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
