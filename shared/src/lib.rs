#![no_std]

#[derive(Debug, PartialEq)]
pub enum HealthStatus{
    Ready, 
    Fail(HealthError),
}

#[derive(Debug, PartialEq)]
pub enum HealthError{
    // Memory
    StackCanary,
    RamTest,
    FlashCRC,
    
    // Clocks
    TimerNotTicking,
    ClockOutOfRange,

    // Peripherals
    AdcCalibration, 
    I2cInitFailed,
    SpiInitFailed,
    UartInitFailed,
    DmaInitFailed,

}


// Real-time transfer check 
impl HealthStatus{
    pub fn as_str(&self) -> &'static str {
        match self{
            HealthStatus::Ready => "READY\n",
            HealthStatus::Fail(HealthError::AdcCalibration) => "FAIL:adc\n",
            HealthStatus::Fail(HealthError::TimerNotTicking) => "FAIL:tim\n",
            HealthStatus::Fail(HealthError::ClockOutOfRange) => "FAIL:clk\n",
            HealthStatus::Fail(HealthError::StackCanary)    => "FAIL:stack\n",
            HealthStatus::Fail(HealthError::RamTest)        => "FAIL:ram\n",
            HealthStatus::Fail(HealthError::FlashCRC)       => "FAIL:flash\n",
            HealthStatus::Fail(HealthError::I2cInitFailed)  => "FAIL:i2c\n",
            HealthStatus::Fail(HealthError::SpiInitFailed)  => "FAIL:spi\n",
            HealthStatus::Fail(HealthError::UartInitFailed) => "FAIL:uart\n",
            HealthStatus::Fail(HealthError::DmaInitFailed)     => "FAIL:dma\n",
        }
    }
}