// MIT License
//
// Copyright (c) 2017 Brook Heisler
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Based on code from https://github.com/bheisler/raytracer and
// https://bheisler.github.io/.

use image::ImageBuffer;

pub struct World;
pub type Vector3 = euclid::Vector3D<f32, World>;

type Float = f32;

pub struct Point {
    pub x: Float,
    pub y: f32,
    pub z: f32,
}

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl From<Color> for image::Bgra<u8> {
    fn from(c: Color) -> Self {
        fn scale(component: Float) -> u8 { (component * 255.0) as u8 }
        Self([scale(c.blue), scale(c.green), scale(c.red), 255])
    }
}

pub type Image<P> = image::ImageBuffer<P, Vec<<P as image::Pixel>::Subpixel>>;

pub struct Renderer<P: image::Pixel>
{
    scene: Scene,
    pub image: Image<P>,
}

impl<P> Renderer<P> where
    P: image::Pixel + From<Color> + From<[u8; 4]> + 'static,
{
    pub fn new(scene: Scene) -> Self {
        Renderer {
            image: ImageBuffer::new(scene.width, scene.height),
            scene,
        }
    }

    pub fn render(&mut self) {
        for (i, pixel) in self.image.pixels_mut().enumerate() {
            let color = i as u32;
            *pixel = P::from(color.to_le_bytes());
        }
    }
}

pub fn make_scene() -> Scene {
    Scene {
        width: 800,
        height: 600,
        fov: 90.,
        sphere: Sphere {
            center: Point { x: 0., y: 0., z: -5. },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
            }
        }
    }
}
