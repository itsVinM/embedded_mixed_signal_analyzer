use defmt::info;
use shared::{HealthError, HealthStatus};

// Embassy imports
use embassy_stm32::{i2c, rcc, Peri, spi};
use embassy_stm32::peripherals::{
    I2C1, PB8, PB9, RCC, 
    ADC1, 
    SPI2, PB13, PB14, PB15, 
    USART1, PA9};
use embassy_stm32::adc::Adc;
use embassy_stm32::usart::UartTx;


pub fn check_clock(
    rcc_peripheral: Peri<'_, RCC>) 
-> HealthStatus {

    let clocks = rcc::clocks(&rcc_peripheral);
    let sys_hz = clocks.sys.to_hertz().map(|h| h.0).unwrap_or(0);
    let pclk1_tim_hz = clocks.pclk1_tim.to_hertz().map(|h| h.0).unwrap_or(0);
    let pclk1_hz = clocks.pclk1.to_hertz().map(|h| h.0).unwrap_or(0);
    let hclk1_hz = clocks.hclk1.to_hertz().map(|h| h.0).unwrap_or(0);
    info!("sys clock = {} Hz", sys_hz);
    info!("pclk1_tim = {} Hz", pclk1_tim_hz);
    info!("pclk1 = {} Hz", pclk1_hz);
    info!("hclk1 (DMA bus) = {} Hz", hclk1_hz);

    if sys_hz == 0 {
        return HealthStatus::Fail(HealthError::ClockOutOfRange);
    }

    if pclk1_tim_hz == 0 {
        return HealthStatus::Fail(HealthError::TimerNotTicking);
    }
    if pclk1_hz == 0 {
    return HealthStatus::Fail(HealthError::I2cInitFailed);
    }
    if hclk1_hz == 0 {
    return HealthStatus::Fail(HealthError::DmaInitFailed);
    }


    info!("clock init ok");
    HealthStatus::Ready
}


pub fn check_i2c(
    i2c: Peri<'_, I2C1>,
    scl: Peri<'_, PB8>,
    sda: Peri<'_, PB9>,
)-> HealthStatus {
    let _i2c_driver = i2c::I2c::new_blocking(
        i2c,                                // I2C1 peripheral
        scl,                                // clock pin
        sda,                                // data pin
        Default::default(),                 // config
    );
    info!("I2C init succeeded");
    HealthStatus::Ready
}

pub fn check_spi(
    spi:  Peri<'_, SPI2>,
    sck:  Peri<'_, PB13>,
    miso: Peri<'_, PB14>,
    mosi: Peri<'_, PB15>,
) -> HealthStatus {
    let _spi_driver = spi::Spi::new_blocking(
        spi, 
        sck,
        mosi,
        miso,
        Default::default(),
    );
    info!("Spi init succeeded");
    HealthStatus::Ready
}

pub fn check_uart(
    uart:  Peri<'_, USART1>,
    tx:  Peri<'_, PA9>,
) -> HealthStatus {
    match UartTx::new_blocking(uart, tx, Default::default()) {
        Ok(_)  => {
            info!("UART init succeeded");
            HealthStatus::Ready
        }
        Err(_) => {
            info!("UART init FAILED");
            HealthStatus::Fail(HealthError::UartInitFailed)
        }
    }
}

pub fn check_adc(
    adc: Peri<'_, ADC1>
 ) -> HealthStatus {
    let _adc_driver = Adc::new(
        adc,
    );
    info!("Adc init succedded");
    HealthStatus::Ready
}


const CANARY_ADDRESS: *mut u32 = 0x2000_1000 as *mut u32;
const CANARY_VALUE: u32 = 0xDEAD_BEEF;

pub fn check_stack_canary() -> HealthStatus {
    unsafe {
        // write known pattern
        core::ptr::write_volatile(CANARY_ADDRESS, CANARY_VALUE);
        
        // read it back
        let read_back = core::ptr::read_volatile(CANARY_ADDRESS);
        
        // verify it matches
        if read_back != CANARY_VALUE {
            return HealthStatus::Fail(HealthError::StackCanary);
        }
    }
    info!("stack canary ok — read back 0x{:X}", CANARY_VALUE);
    HealthStatus::Ready
}

const RAM_TEST_BASE: *mut u32 = 0x2000_4000 as *mut u32;
const RAM_TEST_SIZE: usize= 16;

pub fn check_ram() -> HealthStatus {
    unsafe {
        // write pattern to each address
        for i in 0..RAM_TEST_SIZE {
            core::ptr::write_volatile(RAM_TEST_BASE.add(i), 0xFEED_FACE);
        }
        // read back each address
        for i in 0..RAM_TEST_SIZE {
            let read_back = core::ptr::read_volatile(RAM_TEST_BASE.add(i));
            if read_back != 0xFEED_FACE {
                return HealthStatus::Fail(HealthError::RamTest);
            }
        }
    }
    info!("ram ok — {} words verified", RAM_TEST_SIZE);
    HealthStatus::Ready
}