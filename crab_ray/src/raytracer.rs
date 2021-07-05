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

/// The floating point type used for coordinate space.
pub type Float = f64;

pub struct World;
pub type Vector3 = euclid::Vector3D<Float, World>;
pub type Point = euclid::Point3D<Float, World>;

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub const fn black() -> Self {
        Color { red: 0.0, green: 0.0, blue: 0.0 }
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: Float,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: Float,
    pub sphere: Sphere,
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn new_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        // Make the rays pass through the center of each pixel.
        let (x, y) = (x as Float + 0.5, y as Float + 0.5);
        // Rescale the coordinates to [-1, 1]. Invert y.
        let sensor_x = (x / scene.width as Float) * 2.0 - 1.0;
        let sensor_y = 1.0 - (y / scene.height as Float) * 2.0;
        // Adjust for aspect ratio to ensure pixels are the same distance apart
        // on both axes.
        assert!(scene.width >= scene.height);
        let aspect_ratio = (scene.width as Float) / (scene.height as Float);
        let sensor_x = sensor_x * aspect_ratio;
        // Apply field of view. TODO cache adjustment.
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let sensor_x = sensor_x * fov_adjustment;
        let sensor_y = sensor_y * fov_adjustment;
        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                // Put the sensor one unit in front of the camera.
                z: -1.0,
                ..Vector3::default()
            }.normalize(),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        let l: Vector3 = self.center - ray.origin;
        let adj = l.dot(ray.direction);
        let dist_sq = l.square_length() - (adj * adj);
        dist_sq < (self.radius * self.radius)
    }
}

impl From<&Color> for image::Bgra<u8> {
    fn from(c: &Color) -> Self {
        fn scale(component: f32) -> u8 { (component * 255.0) as u8 }
        Self([scale(c.blue), scale(c.green), scale(c.red), 255])
    }
}
impl From<Color> for image::Bgra<u8> {
    fn from(c: Color) -> Self {
        (&c).into()
    }
}

pub type Image<P> = image::ImageBuffer<P, Vec<<P as image::Pixel>::Subpixel>>;

pub struct Renderer<P: image::Pixel>
{
    scene: Scene,
    pub image: Image<P>,
}

impl<P> Renderer<P> where
    P: image::Pixel + for<'a> From<&'a Color> + From<[u8; 4]> + 'static,
{
    pub fn new(scene: Scene) -> Self {
        Renderer {
            image: ImageBuffer::new(scene.width, scene.height),
            scene,
        }
    }

    pub fn render(&mut self) {
        let Renderer { scene, image } = self;
        for x in 0..scene.width {
            for y in 0..scene.height {
                let ray = Ray::new_prime(x, y, &scene);
                if scene.sphere.intersect(&ray) {
                    image.put_pixel(x, y, P::from(&scene.sphere.color));
                } else {
                    image.put_pixel(x, y, P::from(&Color::black()));
                }
            }
        }
    }
}

pub fn make_scene() -> Scene {
    Scene {
        width: 800,
        height: 600,
        fov: 90.,
        sphere: Sphere {
            center: Point::new(0., 0., -5.),
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
            }
        }
    }
}
