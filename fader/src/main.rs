#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::dma;
use embassy_rp::gpio::Pin;
use embassy_rp::pwm::{self, Pwm};
use embassy_rp::{
    adc::{self, Adc},
    bind_interrupts, interrupt,
};
use embassy_time::{Duration, Timer};
use futures::{join, Future};
use {defmt_rtt as _, panic_probe as _};

mod touch;
use touch::Touch;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => adc::InterruptHandler;
});

trait ReadPin {
    async fn read(&mut self) -> u16;
}

async fn move_to(val: u16, pwm: &mut Pwm<'_, impl pwm::Channel>, mut inp: impl ReadPin) {
    let mut cfg: pwm::Config = Default::default();
    cfg.phase_correct = true;
    cfg.divider = 10.into(); // 100 Hz

    let mut integral = 0.0;
    let mut last = None;
    loop {
        let error = (val as i32) - (inp.read().await as i32);
        if error.abs() < 64 {
            break;
        }

        const P: f32 = 1.0;
        const I: f32 = 0.02;
        const D: f32 = 0.0;
        let error: f32 = error as f32 / 4069.0;
        let computed: f32 =
            P * error + I * integral + last.map(|l| (error - l) as f32 * D).unwrap_or(0.0);
        debug!(
            "{} {} {} => {}",
            error,
            last.unwrap_or(0.0),
            integral,
            computed
        );

        let scaled = computed * 0xffff as f32;
        if computed.is_sign_negative() {
            cfg.compare_a = 0;
            cfg.compare_b = -scaled as u16;
        } else {
            cfg.compare_a = scaled as u16;
            cfg.compare_b = 0;
        };

        pwm.set_config(&cfg);
        Timer::after(Duration::from_millis(2)).await;

        integral += error;
        last = Some(error);
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");

    #[allow(unused_mut)]
    let mut p = embassy_rp::init(Default::default());

    let mut touch = Touch::new(p.PIN_26).await.unwrap();
    let mut adc = Adc::new(p.ADC, Irqs, Default::default());

    struct Reader<'a, 'd, PIN: embedded_hal::adc::Channel<Adc<'d>, ID = u8> + Pin> {
        adc: &'a mut Adc<'d>,
        pin: &'a mut PIN,
    }
    impl<'a, 'd, PIN: embedded_hal::adc::Channel<Adc<'d>, ID = u8> + Pin> ReadPin
        for Reader<'a, 'd, PIN>
    {
        async fn read(&mut self) -> u16 {
            self.adc.read(self.pin).await
        }
    }

    for target in [0.0, 0.25, 0.75, 0.5, 1.0] {
        Timer::after(Duration::from_millis(1000)).await;
        info!("Moving to {}", target);
        let mut pwm = Pwm::new_output_ab(
            &mut p.PWM_CH0,
            &mut p.PIN_16,
            &mut p.PIN_17,
            Default::default(),
        );
        if embassy_time::with_timeout(
            Duration::from_millis(2000),
            move_to(
                (target * 4096.0) as u16,
                &mut pwm,
                Reader {
                    adc: &mut adc,
                    pin: &mut p.PIN_27,
                },
            ),
        )
        .await
        .is_err()
        {
            debug!("Timed out moving servo");
        }
    }

    let mut cfg: pwm::Config = Default::default();
    cfg.phase_correct = true;
    cfg.divider = 10.into(); // 100 Hz
    cfg.compare_a = 0xbbbb;
    cfg.compare_b = 0x8888;
    // {
    //     let _pwm = Pwm::new_output_a(&mut p.PWM_CH0, p.PIN_16, cfg.clone());
    //     Timer::after(Duration::from_millis(200)).await;
    // }
    // {
    //     let _pwm = Pwm::new_output_b(&mut p.PWM_CH0, p.PIN_17, cfg.clone());
    //     Timer::after(Duration::from_millis(300)).await;
    // }

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
