#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::khz;
use embassy_stm32::timer::pwm_input::PwmInput;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::{bind_interrupts, peripherals, timer};
use embassy_time::Timer;

bind_interrupts!(struct Irqs {
    TIM3 => timer::CaptureCompareInterruptHandler<peripherals::TIM3>;
});

pub struct PwmSignal {
    pub frequency_hz: u32,
    pub duty_cycle_pct: u32,
}

pub async fn pwm_task(
    p: embassy_stm32::Peripherals,
    signal: &PwmSignal,
) -> ! {
    // Create PWM pin on PA7
    let ch1_pin = PwmPin::new(p.PA7, OutputType::PushPull);

    // Initialize SimplePwm on TIM1
    let mut pwm = SimplePwm::new(
        p.TIM1,
        Some(ch1_pin),
        None,
        None,
        None,
        khz(10),  // Change to khz(100) for 100 kHz testing
        Default::default(),
    );

    let mut ch1 = pwm.ch1();
    ch1.enable();

    loop {
        // Set duty cycle to 50%
        ch1.set_duty_cycle_fraction(1, 2);
        Timer::after_millis(1000).await;
    }
}

//  Measures signal on PA6 (TIM3)
pub async fn capture_task(
    p: embassy_stm32::Peripherals,
) -> PwmSignal {
    // Create PWM input on TIM3, channel 1, listening on PA6
    let mut pwm_input = PwmInput::new_ch1(
        p.TIM3,
        p.PA6,
        Irqs,
        embassy_stm32::gpio::Pull::None,
        khz(10),  // Must match pwm_task frequency!
    );

    pwm_input.enable();

    // Wait for first measurement
    Timer::after_millis(100).await;

    // Read measurements
    let period_ticks = pwm_input.get_period_ticks();
    let width_ticks = pwm_input.get_width_ticks();
    let duty_cycle_pct = pwm_input.get_duty_cycle();

    info!(
        "Period: {} ticks, Width: {} ticks, Duty: {}%",
        period_ticks, width_ticks, duty_cycle_pct
    );

    // Return the measurement
    PwmSignal {
        frequency_hz: 10000,  // TODO: Calculate from period_ticks
        duty_cycle_pct,
    }
}