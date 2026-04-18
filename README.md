# Mixed Signal Analyser

A full-stack, register-level mixed signal analyser that captures and processes both **analogue** and **digital** signals using a three-layer architecture.

Built as a real engineering instrument — not a toy — spanning embedded Rust, backend systems, and a terminal UI.

---

## ✨ Overview

This project implements a mixed signal analyser similar to a logic analyser + oscilloscope hybrid.

It supports:

- **Analogue signal capture** (via ADC)
- **Digital signal decoding** (GPIO, UART, SPI, I2C)
- **Real-time streaming** via gRPC
- **Terminal-based UI** for visualization

The system is composed of **three independent programs**, written in **two languages**, connected via **gRPC**, and fully **containerised using Podman**.

---
```bash
system_profiler SPUSBDataType | grep -A8 "STM\|STLINK\|ST-Link\|0483"
```

FLASHING 
```bash
# build inside OrbStack
orb run -p -w /Users/vincentiumocanu/Documents/Rust/embedded_mixed_signal_analyzer msa \
  cargo build --release --target thumbv7em-none-eabihf

# flash from Mac
probe-rs run --chip STM32F401RETx \
  target/thumbv7em-none-eabihf/release/embedded_oscilloscope

# inspect binary size (inside container)
orb run -w /Users/vincentiumocanu/Documents/Rust/embedded_mixed_signal_analyzer msa \
  cargo size --release --target thumbv7em-none-eabihf
```

```bash
probe-rs run --chip STM32F401RETx \
  --connect-under-reset \
  --speed 100 \
  target/thumbv7em-none-eabihf/release/embedded_oscilloscope
```

![alt text](<Screenshot 2026-04-17 at 20.36.45.png>)