#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::rcc::{
    APBPrescaler, Pll, PllMul, PllPDiv, PllPreDiv, PllSource, Sysclk,
};
use embassy_stm32::Config;
use panic_probe as _;
use shared::HealthStatus;

mod health;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {

    // ==== CLOCK TREE CONFIG ======
    let mut config = Config::default();
    config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::Div8,   // M=8
        mul: PllMul::Mul168,       // N=168
        divp: Some(PllPDiv::Div4), // P=4 → 84 MHz
        divq: None,
        divr: None,
    });
    config.rcc.pll_src = PllSource::Hsi;
    config.rcc.sys = Sysclk::Pll1P;
    config.rcc.apb1_pre = APBPrescaler::Div2;  // 84/2 = 42 MHz — within limit
    config.rcc.apb2_pre = APBPrescaler::Div1;  // 84/1 = 84 MHz 

    let peripherals = embassy_stm32::init(config);
    info!("booting...");

    health_check(peripherals).await;
}


async fn health_check(peripherals: embassy_stm32::Peripherals){
    let clock_status = health::check_clock(peripherals.RCC);
    if clock_status != HealthStatus::Ready {
        info!("clock FAILED: {}", clock_status.as_str());
        loop {}
    }

    let i2c_status = health::check_i2c(peripherals.I2C1, peripherals.PB8, peripherals.PB9);
    if i2c_status != HealthStatus::Ready {
        info!("i2c FAILED: {}", i2c_status.as_str());
        loop {}
    }

    let spi_status = health::check_spi(peripherals.SPI2, peripherals.PB13, peripherals.PB14, peripherals.PB15);
    if spi_status != HealthStatus::Ready {
        info!("spi FAILED: {}", spi_status.as_str());
        loop {}
    }

    let uart_status = health::check_uart(peripherals.USART1, peripherals.PA9);
    if uart_status != HealthStatus::Ready {
        info!("uart FAILED: {}", uart_status.as_str());
        loop {}
    }

    let adc_status = health::check_adc(peripherals.ADC1);
    if adc_status != HealthStatus::Ready {
        info!("adc FAILED: {}", adc_status.as_str());
        loop {}
    }

    let canary_status = health::check_stack_canary();
    if canary_status != HealthStatus::Ready {
        info!("adc FAILED: {}", canary_status.as_str());
        loop {}
    }

    let ram_status = health::check_ram();
    if ram_status != HealthStatus::Ready {
        info!("adc FAILED: {}", ram_status.as_str());
        loop {}
    }

    info!("all health checks passed — ready for commands");

    let mut led = Output::new(peripherals.PA5, Level::Low, Speed::Low);

    loop {
        led.set_high();
        embassy_time::Timer::after_millis(500).await;
        led.set_low();
        embassy_time::Timer::after_millis(500).await;
    }
}