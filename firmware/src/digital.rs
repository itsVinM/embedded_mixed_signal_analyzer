use defmt::*;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::peripherals::{PA0, PA1, PA6, PA7, TIM1, TIM3, Peri};
use embassy_stm32::time::khz;
use embassy_stm32::timer::pwm_input::PwmInput;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::{bind_interrupts, peripherals, timer};
use embassy_time::Timer;

// INTERRUPT BINDING 
bind_interrupts!(struct Irqs {
    TIM3 => timer::CaptureCompareInterruptHandler<peripherals::TIM3>;
});

pub struct PwmSignal {
    pub frequency_hz: u32,
    pub duty_cycle_pct: u32,
}

// PWM OUTPUT TASK - Generates test signal on PA7 (TIM1)
#[embassy_executor::task]
pub async fn pwm_task(
    tim1: Peri<'static, TIM1>,
    pa7: Peri<'static, PA7>,
) -> ! {
    // Create PWM pin on PA7
    let ch1_pin = PwmPin::new(pa7, OutputType::PushPull);

    // Initialize SimplePwm on TIM1
    let mut pwm = SimplePwm::new(
        tim1,
        Some(ch1_pin),
        None,
        None,
        None,
        khz(10),  // Change to khz(100) for 100 kHz testing
        Default::default(),
    );

    let mut ch1 = pwm.ch1();
    ch1.enable();

    info!("PWM task started on PA7 @ 10 kHz");

    loop {
        // Set duty cycle to 50%
        ch1.set_duty_cycle_fraction(1, 2);
        Timer::after_millis(1000).await;
    }
}

// INPUT CAPTURE TASK - Measures signal on PA6 (TIM3)
#[embassy_executor::task]
pub async fn capture_task(
    tim3: Peri<'static, TIM3>,
    pa6: Peri<'static, PA6>,
) -> ! {
    // Create PWM input on TIM3, channel 1, listening on PA6
    let mut pwm_input = PwmInput::new_ch1(
        tim3,
        pa6,
        Irqs,
        embassy_stm32::gpio::Pull::None,
        khz(10),  // Must match pwm_task frequency!
    );

    pwm_input.enable();

    info!("Capture task started on PA6");

    loop {
        // Wait between measurements
        Timer::after_millis(500).await;

        // Read measurements
        let period_ticks = pwm_input.get_period_ticks();
        let width_ticks = pwm_input.get_width_ticks();
        let duty_cycle_pct = pwm_input.get_duty_cycle();

        info!(
            "Period: {} ticks, Width: {} ticks, Duty: {}%",
            period_ticks, width_ticks, duty_cycle_pct
        );
    }
}