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

use crate::color::Color;
use std::f32::consts::PI;
use image::ImageBuffer;

/// The floating point type used for coordinate space.
pub type Float = f64;

pub type Image<P> = image::ImageBuffer<P, Vec<<P as image::Pixel>::Subpixel>>;

pub struct World;
pub type Vector3 = euclid::Vector3D<Float, World>;
pub type Point = euclid::Point3D<Float, World>;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: Float,
    pub shapes: Vec<Primitive>,
    pub lights: Vec<Box<dyn Light>>,
    // Tiny fudge factor for making sure our shadow rays don't accidentally
    // intersect the object we're tracing from.
    pub shadow_bias: Float,
    pub background: Color,
}

pub struct Primitive {
    pub shape: Box<dyn Intersectable>,
    pub surface: Surface,
}

pub struct Surface {
    pub color: Color,
    pub albedo: f32,
}

pub struct Sphere {
    pub center: Point,
    pub radius: Float,
}

pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
}

pub struct DirectionalLight {
    pub direction: Vector3,
    //pub color: Color,
    pub intensity: f32,
}

pub struct SphericalLight {
    pub point: Point,
    //pub color: Color,
    pub intensity: f32,
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Primitive,
}

pub struct Interaction {
    pub point: Point,
    pub surface_norm: Vector3,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, point: &Point) -> Vector3;
}

pub trait Light {
    /// Returns the intensity of the light at the point of the intersection,
    /// assuming no occlusion.
    fn light_power(&self, interaction: &Interaction, scene: &Scene) -> f32;
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

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.shapes
            .iter()
            .filter_map(|s| s.shape.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, object: &'a Primitive) -> Intersection<'a> {
        Intersection { distance, object }
    }

    pub fn as_interaction(&self, ray: &Ray) -> Interaction {
        let point = ray.origin + ray.direction * self.distance;
        Interaction {
            point,
            // TODO: How to make sure this is facing the right direction?
            surface_norm: self.object.shape.surface_normal(&point),
        }
    }

    fn light_reflected(&self, ray: &Ray, scene: &Scene) -> Color {
        let interaction = self.as_interaction(ray);
        let mut color = Color::black();
        for light in &scene.lights {
            let light_power = light.light_power(&interaction, scene);
            let light_reflected = self.object.surface.albedo / PI;
            color = color + self.object.surface.color.clone() *
                (light_power * light_reflected);
        }
        color
    }
}

/// Check if we are occluded by another object (shadow).
fn occlusion<'a>(
    direction_to_light: Vector3,
    int: &Interaction,
    scene: &'a Scene
) -> Option<Intersection<'a>> {
    let shadow_ray = Ray {
        origin: int.point + int.surface_norm * scene.shadow_bias,
        direction: direction_to_light,
    };
    scene.trace(&shadow_ray)
}

impl Light for DirectionalLight {
    fn light_power(&self, int: &Interaction, scene: &Scene) -> f32 {
        let direction_to_light = -self.direction;
        if occlusion(direction_to_light, int, scene).is_some() {
            return 0.0;
        }
        int.surface_norm.dot(direction_to_light).max(0.0) as f32 * self.intensity
    }
}

impl Light for SphericalLight {
    fn light_power(&self, int: &Interaction, scene: &Scene) -> f32 {
        let to_light = self.point - int.point;
        let distance_sq = to_light.square_length();
        let direction_to_light = to_light.normalize();
        match occlusion(direction_to_light, int, scene) {
            Some(int) if int.distance * int.distance < distance_sq => return 0.0,
            _ => (),
        }
        let intensity = self.intensity / (4.0 * PI * distance_sq as f32);
        int.surface_norm.dot(direction_to_light).max(0.0) as f32 * intensity
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // First triange:
        // H: Distance between ray and sphere origin
        // A: Segment of ray ending at point X nearest sphere origin (forming a right angle)
        // Solve for opposite side = distance between point X and sphere origin
        let l: Vector3 = self.center - ray.origin;
        let adj = l.dot(ray.direction);
        let center_dist_sq = l.square_length() - (adj * adj);
        let radius_sq = self.radius * self.radius;
        if center_dist_sq > radius_sq {
            return None
        }
        // Second triangle:
        // O: Same as before
        // H: Radius (distance from origin to intersection)
        // Solve for adjacent side = "thickness", distance between X and intersection points
        let thickness = (radius_sq - center_dist_sq).sqrt();
        // We may need to consider i2 if the camera is inside the sphere.
        let i1 = adj - thickness;
        let i2 = adj + thickness;
        // Return the nearest intersection that is in front of the camera.
        [i1, i2].iter().copied()
            .filter(|d| d >= &0.)
            .min_by(|i1, i2| i1.partial_cmp(i2).unwrap())
    }

    fn surface_normal(&self, point: &Point) -> Vector3 {
        (*point - self.center).normalize()
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // Check if the ray and plane are parallel. If so, they won't intersect.
        let ray_dot_n = ray.direction.dot(self.normal);
        if ray_dot_n.abs() < 1e-6 {
            return None;
        }
        let distance = (self.origin - ray.origin).dot(self.normal) / ray_dot_n;
        if distance >= 0.0 {
            Some(distance)
        } else {
            None
        }
    }

    fn surface_normal(&self, _point: &Point) -> Vector3 {
        self.normal
    }
}

pub struct Renderer<P: image::Pixel> {
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
                match scene.trace(&ray) {
                    None => image.put_pixel(x, y, P::from(&scene.background)),
                    Some(intersection) => {
                        let color = intersection.light_reflected(&ray, &scene);
                        image.put_pixel(x, y, P::from(&color))
                    }
                }
            }
        }
    }
}

pub fn make_scene() -> Scene {
    trait Boxed where Self: Sized {
        fn boxed(self) -> Box<Self> {
            Box::new(self)
        }
    }
    impl<T> Boxed for T {}

    Scene {
        width: 800,
        height: 600,
        fov: 90.,
        background: Color::rgb(0.0, 0.4, 0.8),
        lights: vec![
            DirectionalLight {
                direction: Vector3::new(-0.2, -0.9, -0.8).normalize(),
                intensity: 1.0,
            }.boxed(),
            DirectionalLight {
                direction: Vector3::new(0.2, -0.9, -0.8).normalize(),
                intensity: 1.0,
            }.boxed(),
            SphericalLight {
                point: Point::new(-0.1, -1.0, -0.0),
                intensity: 200.0,
            }.boxed(),
        ],
        shadow_bias: 1e-6,
        shapes: vec![
            Primitive {
                shape: Sphere {
                    center: Point::new(-1.7, -0.7, -7.),
                    radius: 1.0,
                }.boxed(),
                surface: Surface {
                    color: Color::rgb(1.0, 0.4, 0.4),
                    albedo: 1.0,
                },
            },
            Primitive {
                shape: Sphere {
                    center: Point::new(0., 0., -5.),
                    radius: 1.0,
                }.boxed(),
                surface: Surface {
                    color: Color::rgb(0.4, 1.0, 0.4),
                    albedo: 1.0,
                },
            },
            Primitive {
                shape: Sphere {
                    center: Point::new(1.0, 1.0, -4.),
                    radius: 1.2,
                }.boxed(),
                surface: Surface {
                    color: Color::rgb(0.4, 0.4, 1.0),
                    albedo: 0.9,
                },
            },
            Primitive {
                shape: Plane {
                    origin: Point::new(0., -6.0, 0.),
                    normal: Vector3::new(0., 1., 0.),
                }.boxed(),
                surface: Surface {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    albedo: 1.0,
                },
            },
        ],
    }
}
