# Embedded oscilloscope on a stm32
Status:     init
Core:       Cortex-M4F at 84 MHz
ADC:        12-bit · ~1 MSPS via DMA · PA0
DAC:        12-bit · PA4 · DMA + TIM6
Loopback:   PA4 → PA0
Interface:  USART2 · onboard USB-UART
User button:PC13 · autosetting + mode control
User LED:   PA5 · status + health check
Firmware:   Rust · embassy-stm32



# Set up
```bash
# locally to development and flashign the stm32 
podman-compose up -d --build 
```