#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::rcc::{
    APBPrescaler, Pll, PllMul, PllPDiv, PllPreDiv, PllSource, Sysclk,
};
use embassy_stm32::Peri;
use embassy_stm32::peripherals::RCC;
use embassy_stm32::Config;
use panic_probe as _;
use shared::HealthStatus;

mod health;
mod analog;

#[embassy_executor::main]
async fn main(spawner: Spawner) {

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
    health_check(peripherals.RCC).await;

    spawner.spawn(analog::adc_task(
    peripherals.ADC1,
    peripherals.DMA2_CH0,
    peripherals.PA0,
    ).unwrap())
}


async fn health_check(rcc: Peri<'_, RCC>){
    // Peripherals will be checked when initialized 
    // due to Rust ownership rules.
    //
    let clock_status = health::check_clock(rcc);
    if clock_status != HealthStatus::Ready {
        info!("clock FAILED: {}", clock_status.as_str());
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
}