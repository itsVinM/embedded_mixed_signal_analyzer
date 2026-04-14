// self-test + RTT handshake 

ude defmt::info;
use embassy_stm32::{
    adc::{Adc, SampleTime},
    peripherals,
    rcc,
};
use shared::{HealthError, HealthStatus};

pub fn run_self_test(
    adc: &mut Adc<'_, peripherals::ADC1>
)->HealthStatus{
    
    // CLOCK check - F401RE at full speed should be 84 MHz
    let sysclk = rcc::clocks().sysclk.0;
    info!("sysclk = {} Hz", sysclk);

    if sysclk < 80_000_000 || sysclk > 88_000_000 {
        return HealthStatus::Fail(HealthError::ClockOutOfRange);
    }

    //
}