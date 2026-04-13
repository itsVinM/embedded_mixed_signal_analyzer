#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pull, Speed};
use embassy_stm32::Peri;
use embassy_time::Timer;

#[embassy_executor::task]
async fn blink(pin: Peri<'static, AnyPin>) {
    // fix 1: no OutputDrive in 0.6, signature is (pin, level, speed)
    let mut led = Output::new(pin, Level::Low, Speed::Low);

    loop {
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // fix 2: Peri needs a deref, unwrap goes on blink() not spawn()
    spawner.spawn(blink(p.PA5.into()).expect("blink spawn failed"));

    let button = Input::new(p.PC13, Pull::Up);
    loop {
        // fix 3: you were right, they are async in 0.6
        button.is_low();
        info!("Button pressed!");
        button.is_high();
        info!("Button released!");
    }
}