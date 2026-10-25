import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "SilentMic-RT | Sub-3ms Hard Real-Time Neural Voice Isolator & Virtual Microphone Driver",
  description:
    "Deterministic, zero-allocation sub-3ms hard real-time neural voice isolator and virtual microphone driver running entirely offline on bare-metal CPU SIMD.",
  authors: [{ name: "Shah Md. Mahi", url: "https://shahmdmahi.dpdns.org" }],
  creator: "Shah Md. Mahi",
  publisher: "Shah Md. Mahi",
  keywords: [
    "audio dsp",
    "noise suppression",
    "voice isolation",
    "real-time audio",
    "virtual microphone",
    "pipewire",
    "wasapi",
    "coreaudio",
    "erb filterbank",
    "rust audio",
    "zero allocation",
  ],
  metadataBase: new URL("https://silentmic-rt.shahmdmahi.dpdns.org"),
  alternates: {
    canonical: "https://silentmic-rt.shahmdmahi.dpdns.org",
  },
  openGraph: {
    title: "SilentMic-RT | Sub-3ms Hard Real-Time Neural Voice Isolator",
    description:
      "Eliminate room noise, crying babies, barking dogs, typing clicks, and room echo in sub-3ms latency with zero cloud transmission.",
    url: "https://silentmic-rt.shahmdmahi.dpdns.org",
    siteName: "SilentMic-RT",
    images: [
      {
        url: "/og-banner.svg",
        width: 1200,
        height: 630,
        alt: "SilentMic-RT Architecture Banner",
      },
    ],
    locale: "en_US",
    type: "website",
  },
  twitter: {
    card: "summary_large_image",
    title: "SilentMic-RT | Sub-3ms Hard Real-Time Neural Voice Isolator",
    description: "Zero-allocation, deterministic offline neural voice isolator and virtual microphone driver.",
    images: ["/og-banner.svg"],
    creator: "@shahmdmahi",
  },
  icons: {
    icon: "/favicon.svg",
    apple: "/logo.svg",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const jsonLd = {
    "@context": "https://schema.org",
    "@graph": [
      {
        "@type": "SoftwareApplication",
        "name": "SilentMic-RT",
        "url": "https://silentmic-rt.shahmdmahi.dpdns.org",
        "operatingSystem": "Linux, Windows, macOS",
        "applicationCategory": "MultimediaApplication",
        "offers": {
          "@type": "Offer",
          "price": "0",
          "priceCurrency": "USD"
        },
        "description": "Deterministic, Zero-Allocation Sub-3ms Hard Real-Time Neural Voice Isolator & Virtual Microphone Driver",
        "author": {
          "@type": "Person",
          "name": "Shah Md. Mahi",
          "url": "https://shahmdmahi.dpdns.org"
        }
      },
      {
        "@type": "Person",
        "name": "Shah Md. Mahi",
        "url": "https://shahmdmahi.dpdns.org",
        "jobTitle": "Lead Systems & DSP Engineer",
        "sameAs": [
          "https://github.com/shahmdmahi15",
          "https://linkedin.com/in/shahmdmahi27",
          "https://t.me/shahmdmahi"
        ]
      }
    ]
  };

  return (
    <html lang="en" className="dark scroll-smooth">
      <head>
        <link rel="icon" type="image/x-icon" href="/favicon.ico?v=3" />
        <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png?v=3" />
        <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png?v=3" />
        <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png?v=3" />
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
        />
      </head>
      <body className="antialiased bg-[#060913] text-slate-100 min-h-screen">
        {children}
      </body>
    </html>
  );
}
