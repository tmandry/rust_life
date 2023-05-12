#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{adc::Adc, interrupt};
use embassy_time::{Duration, Timer};
use futures::join;
use {defmt_rtt as _, panic_probe as _};

mod touch;
use touch::Touch;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");

    #[allow(unused_mut)]
    let mut p = embassy_rp::init(Default::default());

    let irq = interrupt::take!(ADC_IRQ_FIFO);
    let mut adc = Adc::new(p.ADC, irq, Default::default());
    let mut touch = Touch::new(p.PIN_26).await.unwrap();
    loop {
        Timer::after(Duration::from_millis(50)).await;
        let (value, touched) = join!(adc.read(&mut p.PIN_27), touch.read());
        info!(
            "{} {} {}",
            Slider { value },
            value,
            if touched { "*" } else { "" }
        );
    }
}

struct Slider {
    value: u16,
}

impl Format for Slider {
    fn format(&self, fmt: Formatter) {
        const WIDTH: u32 = 90;
        const MAX_VAL: u32 = 4096;
        const HALF_STEP: u32 = MAX_VAL / WIDTH;
        let filled = (self.value as u32 + HALF_STEP) * WIDTH / MAX_VAL;
        for _ in 0..filled {
            defmt::write!(fmt, "=");
        }
        for _ in filled..WIDTH {
            defmt::write!(fmt, "-");
        }
    }
}
