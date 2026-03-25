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
