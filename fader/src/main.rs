#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

mod touch;
use touch::Touch;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");

    #[allow(unused_mut)]
    let mut p = embassy_rp::init(Default::default());

    // let irq = interrupt::take!(ADC_IRQ_FIFO);
    // let mut adc = Adc::new(p.ADC, irq, Default::default());
    // loop {
    //     let value = adc.read(&mut p.PIN_27).await;
    //     info!("value = {}", value);
    //     Timer::after(Duration::from_millis(500)).await;
    // }

    let mut touch = Touch::new(p.PIN_26).await.unwrap();
    loop {
        Timer::after(Duration::from_millis(50)).await;
        touch.read().await;
    }
}
