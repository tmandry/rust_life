//! Capacitive touch detection on a GPIO pin.
//!
//! This method works on a GPIO pin connected to a capacitive touch surface
//! AND a pulldown resistor to ground (typically 1M-ohm is recommended).
//!
//! It works by pulling the pin high for a few microseconds, which charges
//! our "capacitor". The pin is then set to float and we measure the amount of
//! time until it discharges via our pulldown resistor.

/// Capacitive touch detector.
pub struct Touch<'pin, P: gpio::Pin> {
    pin: gpio::Flex<'pin, P>,
    threshold: Duration,
}

/// How many samples to take per reading.
const SAMPLES: usize = 10;
/// How many samples need to be above the threshold for the reading to be
/// positive. This controls the sensitivity.
const SAMPLE_THRESHOLD: usize = 2;
/// Timeout for an individual sample, which usually takes around 50 microseconds.
const SAMPLE_TIMEOUT: Duration = Duration::from_micros(150);
/// How many samples to take during calibration to establish a threshold.
const INIT_SAMPLES: usize = 100;

impl<'pin, P: gpio::Pin> Touch<'pin, P> {
    /// Creates a capacitive touch reader at the given pin.
    ///
    /// This method performs calibration, which can take up to several
    /// milliseconds. Currently we assume there is no touch when `new` is
    /// called, or future touches will not be detected.
    pub async fn new(
        pin: impl Peripheral<P = P> + 'pin,
    ) -> Result<Touch<'pin, P>, NoPulldownError> {
        let mut pin = gpio::Flex::new(pin);

        // Perform one reading and throw it away, since the initial values tend to be unpredictable.
        for _ in 0..SAMPLES {
            let _ = Self::read_one(&mut pin).await;
        }

        let mut min = Duration::MAX;
        let mut max = Duration::MIN;
        let mut sum = Duration::from_ticks(0);
        for _ in 0..INIT_SAMPLES {
            let sample = Self::read_one(&mut pin).await;
            if sample >= SAMPLE_TIMEOUT {
                return Err(NoPulldownError);
            }
            min = min.min(sample);
            max = max.max(sample);
            sum += sample;
        }
        debug!(
            "touch init min={} max={} avg={}",
            min.as_ticks(),
            max.as_ticks(),
            (sum / (INIT_SAMPLES as u32)).as_ticks(),
        );

        // TODO: save min and detect need to recalibrate if finger was touching at the beginning
        Ok(Self {
            pin,
            threshold: max,
        })
    }

    /// Detect if a touch is currently present.
    ///
    /// This typically takes half a millisecond, but can take up to two milliseconds.
    pub async fn read(&mut self) -> bool {
        let mut above = 0;
        let mut samples = [0; SAMPLES];
        for i in 0..SAMPLES {
            let sample = Self::read_one(&mut self.pin).await;
            samples[i] = sample.as_ticks();
            if sample > self.threshold {
                above += 1;
            }
        }
        let result = above >= SAMPLE_THRESHOLD;
        debug!("touch = {}; {:?}", result, samples);
        result
    }

    async fn read_one(pin: &mut gpio::Flex<'pin, P>) -> Duration {
        pin.set_high();
        pin.set_as_output();
        Timer::after(Duration::from_micros(10)).await;

        let start = Instant::now();
        pin.set_as_input();
        let _ = embassy_time::with_timeout(SAMPLE_TIMEOUT, pin.wait_for_low()).await;

        let elapsed = Instant::now() - start;
        elapsed
    }
}

/// No pulldown resistor was detected on the pin.
#[derive(Debug, Clone, PartialEq, Eq, defmt::Format)]
pub struct NoPulldownError;

impl From<TimeoutError> for NoPulldownError {
    fn from(_: TimeoutError) -> Self {
        Self
    }
}

use defmt::*;
use embassy_rp::{gpio, Peripheral};
use embassy_time::{Duration, Instant, TimeoutError, Timer};
use {defmt_rtt as _, panic_probe as _};
