#![no_std]

#[derive(Debug, PartialEq)]
pub enum HealthStatus{
    Ready, 
    Fail(HealthError),
}

#[derive(Debug, PartialEq)]
pub enum HealthError{
    AdcCalibration, 
    TimerNotTicking,
    ClockOutOfRange,
}


// Real-time transfer check 
impl HealthStatus{
    pub fn as_str(&self) -> &'static str {
        match self{
            HealthStatus::Ready => "READY: adc, tim, clk\n",
            HealthStatus::Fail(HealthError::AdcCalibration) => "FAIL:adc\n",
            HealthStatus::Fail(HealthError::TimerNotTicking) => "FAIL:tim\n",
            HealthStatus::Fail(HealthError::ClockOutOfRange) => "FAIL:clk\n",
        }
    }
}