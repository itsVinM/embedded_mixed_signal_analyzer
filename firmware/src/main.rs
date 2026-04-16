#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use panic_probe as _;
use shared::HealthStatus;

mod health;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_stm32::init(Default::default());
    info!("booting...");

    let status = health::run_self_test(peripherals.RCC);

    match status {
        HealthStatus::Ready => {
            info!("health check passed — ready for commands");
        }
        HealthStatus::Fail(_) => {
            info!("health check FAILED: {}", status.as_str());
            loop {}
        }
    }

    let mut led = Output::new(peripherals.PA5, Level::Low, Speed::Low);

    loop {
        led.set_high();
        embassy_time::Timer::after_millis(500).await;
        led.set_low();
        embassy_time::Timer::after_millis(500).await;
    }
}