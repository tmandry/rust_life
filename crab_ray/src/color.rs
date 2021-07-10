use std::ops::{Add, Mul};

#[derive(Clone)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    /// Creates a `Color` from standard RGB values scaled from 0.0-1.0.
    pub fn rgb(red: f32, green: f32, blue: f32) -> Color {
        Color {
            red: gamma_decode(red),
            green: gamma_decode(green),
            blue: gamma_decode(blue),
        }
    }

    pub const fn black() -> Self {
        Color { red: 0.0, green: 0.0, blue: 0.0 }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(value: f32) -> f32 {
    value.powf(GAMMA)
}

impl From<&Color> for image::Bgra<u8> {
    fn from(c: &Color) -> Self {
        fn scale(component: f32) -> u8 {
            (gamma_encode(component) * 255.0).clamp(0.0, 255.0) as u8
        }
        Self([scale(c.blue), scale(c.green), scale(c.red), 255])
    }
}

impl From<Color> for image::Bgra<u8> {
    fn from(c: Color) -> Self {
        (&c).into()
    }
}
