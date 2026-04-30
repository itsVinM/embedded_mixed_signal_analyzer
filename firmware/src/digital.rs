use defmt::*;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::peripherals::{PA6, PA8, PA9, PA10, PA11, TIM1, TIM3};
use embassy_stm32::Peri;
use embassy_stm32::time::khz;
use embassy_stm32::timer::pwm_input::PwmInput;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::{bind_interrupts, peripherals, timer};
use embassy_time::Timer;

// INTERRUPT BINDING 
bind_interrupts!(struct Irqs {
    TIM3 => timer::CaptureCompareInterruptHandler<peripherals::TIM3>;
});

// DATA STRUCTURE
pub struct PwmSignal {
    pub frequency_hz: u32,
    pub duty_cycle_pct: u32,
}

const CLOCK_SPEED: u32 = 10; // Clock speed in kHz

// PWM OUTPUT TASK - All 4 TIM1 channels simultaneously
/// Generates 4 independent PWM signals:
/// - PA8 (Ch1): 25% duty cycle
/// - PA9 (Ch2): 50% duty cycle
/// - PA10 (Ch3): 75% duty cycle
/// - PA11 (Ch4): 10% duty cycle
#[embassy_executor::task]
pub async fn pwm_task(
    tim1: Peri<'static, TIM1>,
    pa8: Peri<'static, PA8>,
    pa9: Peri<'static, PA9>,
    pa10: Peri<'static, PA10>,
    pa11: Peri<'static, PA11>,
) -> ! {
    // Create PWM pins for all 4 channels
    let ch1_pin = PwmPin::new(pa8, OutputType::PushPull);
    let ch2_pin = PwmPin::new(pa9, OutputType::PushPull);
    let ch3_pin = PwmPin::new(pa10, OutputType::PushPull);
    let ch4_pin = PwmPin::new(pa11, OutputType::PushPull);

    // Initialize SimplePwm with all 4 channels
    let mut pwm = SimplePwm::new(
        tim1,
        Some(ch1_pin),  // Ch1 - PA8
        Some(ch2_pin),  // Ch2 - PA9
        Some(ch3_pin),  // Ch3 - PA10
        Some(ch4_pin),  // Ch4 - PA11
        khz(CLOCK_SPEED),
        Default::default(),
    );

    // Get handles to each channel
    let (mut ch1, mut ch2, mut ch3, mut ch4) = pwm.split();  // All at once

    // Enable all channels
    ch1.enable();
    ch2.enable();
    ch3.enable();
    ch4.enable();

    info!("╔════════════════════════════════════════════╗");
    info!("║  PWM Task - 4 Channel Generator           ║");
    info!("╚════════════════════════════════════════════╝");
    info!("✓ PA8  (TIM1 Ch1) @ 25% duty cycle");
    info!("✓ PA9  (TIM1 Ch2) @ 50% duty cycle");
    info!("✓ PA10 (TIM1 Ch3) @ 75% duty cycle");
    info!("✓ PA11 (TIM1 Ch4) @ 10% duty cycle");
    info!("Frequency: {} kHz", CLOCK_SPEED);

    loop {
        // Set different duty cycles for each channel
        ch1.set_duty_cycle_fraction(1, 4);  // 25%
        ch2.set_duty_cycle_fraction(1, 2);  // 50%
        ch3.set_duty_cycle_fraction(3, 4);  // 75%
        ch4.set_duty_cycle_fraction(1, 10); // 10%

        Timer::after_millis(5000).await;
    }
}

// INPUT CAPTURE TASK - Measures signal on PA6 (TIM3 Channel 1)
/// Measures frequency and duty cycle of signal connected to PA6
/// Connect any of the PWM outputs (PA8, PA9, PA10, PA11) to PA6 to measure
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
        khz(CLOCK_SPEED),  // Must match pwm_task frequency!
    );

    pwm_input.enable();

    info!("╔════════════════════════════════════════════╗");
    info!("║  Capture Task - Signal Measurement        ║");
    info!("╚════════════════════════════════════════════╝");
    info!("✓ PA6 (TIM3 Ch1) - Measuring input signal");
    info!("📌 Connect any PWM output (PA8/PA9/PA10/PA11) → PA6");

    loop {
        // Wait between measurements
        Timer::after_millis(500).await;

        // Read measurements
        let period_ticks = pwm_input.get_period_ticks();
        let width_ticks = pwm_input.get_width_ticks();
        let duty_cycle_pct = pwm_input.get_duty_cycle();

        info!(
            "  📊 Period: {} ticks | Width: {} ticks | Duty: {}%",
            period_ticks, width_ticks, duty_cycle_pct
        );
    }
}