use defmt::info;
use embassy_stm32::{
    adc::{Adc, AdcChannel, AnyAdcChannel, SampleTime},
    bind_interrupts, dma, peripherals,
    Peri,
};

bind_interrupts!(struct Irqs {
    DMA2_STREAM0 => dma::InterruptHandler<peripherals::DMA2_CH0>;
});

static mut ADC_BUF: [u16; 1024] = [0u16; 1024];

#[embassy_executor::task]
pub async fn adc_task(
    adc: Peri<'static, peripherals::ADC1>,
    dma: Peri<'static, peripherals::DMA2_CH0>,
    mut pin: Peri<'static, peripherals::PA0>,
) {
    let mut adc_driver = Adc::new(adc);
    let channel: AnyAdcChannel<peripherals::ADC1> = pin.degrade_adc();
    let buf = unsafe { &mut ADC_BUF };

    let mut ring = adc_driver.into_ring_buffered(
        dma,
        buf,
        Irqs,
        [(channel, SampleTime::Cycles15)].into_iter(),
        
        None,
    );

    let mut measurements = [0u16; 512];
    loop {
        match ring.read(&mut measurements).await {
            Ok(n) => {
                let count = n.min(measurements.len());
                if count == 0 {
                    continue;
                }
                let avg = measurements[..count]
                    .iter()
                    .map(|&x| x as u32)
                    .sum::<u32>() / count as u32;
                info!("PA0 avg = {} ({} samples)", avg, count);
            }
            Err(_) => {
                //info!("ADC read error: {:?}", e);
                continue;
            }
        }
    }
}