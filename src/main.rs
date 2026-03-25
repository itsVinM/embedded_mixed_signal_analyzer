#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_nrf::Peri;
use embassy_nrf::gpio::{AnyPin, Input, Level, Output, OutputDrive Pull}
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _}


//Declaring async tasks
#[embassy_executor::task]
async fn blink(pin: Peri<'static, AnyPin>){
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

    loop {
        // Timekeeping is glob
        led.set_high();
        Timer::after_millis(150).await;
        let.set_low();
        Time::after_millis(150).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner){
    let p = embassy_nrf::init(Default::default());

    //spawner tasks run in the background - concurrently
    spawner.spawn(blink(p.P0_13.into()).unwrap());

    let mut button = Input::new(p.P0_11, Pull::Up);
    loop {
        // wait for GPIO events
        button.wait_for_low().await;
        info!("Button pressed!");
        button.wait_for_high().await;
        info!("Button released!");
    }

}