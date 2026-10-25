# pysilentmic: Python Real-Time Voice Isolator

Python C-FFI bindings for **SilentMic-RT** — the deterministic, zero-allocation sub-3ms hard real-time neural voice isolator and virtual microphone driver.

## Features
- **Ultra-Low Latency**: 1.33 ms frame hop (64 samples at 48,000 Hz).
- **Sub-3ms Total Latency**: Complete input-to-output algorithmic delay under 2.67 ms.
- **Zero Allocation**: Lock-free, zero heap memory allocations on audio worker threads.
- **Pure Offline SIMD**: Operates entirely on CPU SIMD instructions with zero network or cloud latency.
- **NumPy Integration**: Seamlessly denoises NumPy arrays and streaming audio generators.

## Installation

```bash
pip install pysilentmic
```

## Quickstart

```python
import numpy as np
from pysilentmic import SilentMic

# Initialize SilentMic engine at 48kHz
mic = SilentMic(sample_rate=48000, attenuation_db=-48.0)

# Process 64-sample audio frame in real time (< 30 us)
raw_frame = np.random.randn(64).astype(np.float32)
clean_frame = mic.process_frame(raw_frame)

# Denoise complete audio file / NumPy array
noisy_audio = np.random.randn(48000 * 5).astype(np.float32)
isolated_voice = mic.process(noisy_audio)

# Access hardware telemetry
print(mic.get_stats())
```

## Author & Maintainer
- **Author**: Shah Md. Mahi
- **Email**: shahmdmahi15@gmail.com
- **Website**: [https://shahmdmahi.dpdns.org](https://shahmdmahi.dpdns.org)
- **License**: MIT
