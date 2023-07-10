#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]

use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::Spawner;
use embassy_net as net;
use embassy_rp::gpio::{self, Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0};
use embassy_rp::pio::{self, Pio};
use embassy_rp::pwm::{self, Pwm};
use embassy_rp::{
    adc::{self, Adc, Async},
    bind_interrupts,
};
use embassy_time::{Duration, Timer};
use futures::join;
use {defmt_rtt as _, panic_probe as _};

mod touch;
use static_cell::make_static;
use touch::Touch;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => adc::InterruptHandler;
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
});

trait ReadPin {
    async fn read(&mut self) -> Result<u16, adc::Error>;
}

async fn move_to(val: u16, pwm: &mut Pwm<'_, impl pwm::Channel>, mut inp: impl ReadPin) {
    let mut cfg: pwm::Config = Default::default();
    cfg.phase_correct = true;
    cfg.divider = 10.into(); // 100 Hz

    let mut integral = 0.0;
    let mut last = None;
    loop {
        let reading = match inp.read().await {
            Ok(r) => r,
            Err(e) => {
                warn!("Got error while reading from adc: {}", e);
                Timer::after(Duration::from_millis(2)).await;
                continue;
            }
        };
        let error = (val as i32) - (reading as i32);
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
async fn main(spawner: Spawner) {
    info!("Hello World!");

    #[allow(unused_mut)]
    let mut p = embassy_rp::init(Default::default());

    // let fw = include_bytes!("../firmware/43439A0.bin");
    // let clm = include_bytes!("../firmware/43439A0_clm.bin");

    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs-cli download firmware/43439A0.bin --format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs-cli download firmware/43439A0_clm.bin --format bin --chip RP2040 --base-address 0x10140000
    let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 224190) };
    let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    wifi_connect(&spawner, pwr, spi, fw, clm).await;

    let adc = Adc::new(p.ADC, Irqs, Default::default());
    let touch = Touch::new(p.PIN_26).await.unwrap();
    let slider = adc::Pin::new(p.PIN_27, embassy_rp::gpio::Pull::None);
    let motor_pwm = Pwm::new_output_ab(p.PWM_CH0, p.PIN_16, p.PIN_17, Default::default());

    unwrap!(spawner.spawn(fader_task(motor_pwm, adc, slider, touch)));
}

type PwmChannel = impl pwm::Channel;
type TouchPin = impl gpio::Pin;

#[embassy_executor::task]
async fn fader_task(
    mut motor_pwm: Pwm<'static, PwmChannel>,
    mut adc: Adc<'static, Async>,
    mut slider: adc::Pin<'static>,
    mut touch: Touch<'static, TouchPin>,
) -> ! {
    struct Reader<'a, 'd> {
        adc: &'a mut Adc<'d, Async>,
        pin: &'a mut adc::Pin<'d>,
    }
    impl<'a, 'd> ReadPin for Reader<'a, 'd> {
        async fn read(&mut self) -> Result<u16, adc::Error> {
            self.adc.read(self.pin).await
        }
    }

    for target in [0.0, 0.25, 0.75, 0.5, 1.0] {
        Timer::after(Duration::from_millis(1000)).await;
        info!("Moving to {}", target);
        if embassy_time::with_timeout(
            Duration::from_millis(2000),
            move_to(
                (target * 4096.0) as u16,
                &mut motor_pwm,
                Reader {
                    adc: &mut adc,
                    pin: &mut slider,
                },
            ),
        )
        .await
        .is_err()
        {
            debug!("Timed out moving servo");
        }
        motor_pwm.set_config(&Default::default());
    }

    let mut cfg: pwm::Config = Default::default();
    cfg.phase_correct = true;
    cfg.divider = 10.into();
    // 100 Hz
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
        let (value, touched) = join!(adc.read(&mut slider), touch.read());
        let Ok(value) = value else { continue };
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

include!("../config.rs");

async fn wifi_connect(
    spawner: &Spawner,
    pwr: Output<'static, PIN_23>,
    spi: PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
    fw: &[u8],
    clm: &[u8],
) -> &'static net::Stack<cyw43::NetDriver<'static>> {
    let state = make_static!(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(wifi_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let config = net::Config::dhcpv4(Default::default());
    let seed = 0x0123_4567_89ab_cdef; // TODO

    let stack = &*make_static!(net::Stack::new(
        net_device,
        config,
        make_static!(net::StackResources::<2>::new()),
        seed
    ));

    unwrap!(spawner.spawn(net_task(stack)));

    loop {
        match control.join_wpa2(WIFI_NETWORK, WIFI_PASSWORD).await {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status={}", err.status);
            }
        }
    }

    stack
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<
        'static,
        Output<'static, PIN_23>,
        PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>,
    >,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static net::Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}
