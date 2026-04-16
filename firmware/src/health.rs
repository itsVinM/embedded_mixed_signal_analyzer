use defmt::info;
use embassy_stm32::{peripherals::RCC, rcc, Peri};
use shared::{HealthError, HealthStatus};

pub fn run_self_test(rcc_peripheral: Peri<'_, RCC>) -> HealthStatus {
    let clocks = rcc::clocks(&rcc_peripheral);

    let sys_hz = clocks.sys.to_hertz().map(|h| h.0).unwrap_or(0);
    let pclk1_tim_hz = clocks.pclk1_tim.to_hertz().map(|h| h.0).unwrap_or(0);

    info!("sys clock = {} Hz", sys_hz);
    info!("pclk1_tim = {} Hz", pclk1_tim_hz);

    if sys_hz == 0 {
        return HealthStatus::Fail(HealthError::ClockOutOfRange);
    }

    if pclk1_tim_hz == 0 {
        return HealthStatus::Fail(HealthError::TimerNotTicking);
    }

    info!("self test passed");
    HealthStatus::Ready
}