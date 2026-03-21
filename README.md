# embedded_oscilloscope

Bare-metal oscilloscope, waveform generator, and data logger on the 
STM32F401RE NUCLEO board. Firmware in Rust + embassy. Host tooling in Go.

> Scoppy does this for the Pi Pico. This does it for the STM32.

## Status

`init` — Week 1/26. Workspace scaffolded, container environment running.
Follow the build log in [DEVLOG.md](DEVLOG.md).

## Hardware

| | |
|---|---|
| Board | NUCLEO-F401RE · Cortex-M4F @ 84 MHz |
| ADC | 12-bit · ~1 MSPS via DMA · PA0 |
| DAC | 12-bit · PA4 · DMA + TIM6 |
| Loopback | PA4 → PA0 · 1kΩ series resistor |
| Interface | USART2 · onboard USB-UART |
| Button | PC13 · autosetting + mode control |
| LED | PA5 · status + health check |

## Architecture
```
stm32_fw          →   instrument_core   →   host_processor   →   tui
Rust · embassy        no_std · shared       Rust · std            Go · bubbletea
thumbv7em             thumbv7em+x86         FFT · HDF5            braille waveform
```

## Running
```bash
# flash firmware (requires probe-rs + board connected)
cargo build --release --target thumbv7em-none-eabihf -p stm32_fw

# start host stack
podman-compose up -d --build
```

