pub mod primitives {
    pub mod algebra_impl;
    pub mod color;
    pub mod pos;
    pub mod vector;
}
pub use primitives::{
    pos::{Pos2, Pos3},
    vector::{Vec2, Vec3},
};

use image::Rgb;

use crate::primitives::color::Color;

#[derive(Debug, Clone)]
pub struct Viewport {
    pub position: Pos3,
    pub width: f64,
    pub height: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            position: Pos3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Viewport {
    pub fn new(position: Pos3, width: f64, height: f64) -> Self {
        Self {
            position,
            width,
            height,
        }
    }

    pub fn trace_ray(&self, point: Pos3, scalar: f64) -> Pos3 {
        point + scalar * (self.position.difference(point))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<[u8; 3]>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![[0, 0, 0]; height]; width],
        }
    }

    pub fn to_viewport(&self, x: isize, y: isize, vp: &Viewport) -> Vec3 {
        let x = x as f64 * (vp.width / self.width as f64);
        let y = y as f64 * (vp.height / self.height as f64);

        Vec3::new(x, y, 1.0)
    }

    pub fn put_pixel(&mut self, x: isize, y: isize, color: [u8; 3]) {
        let (nx, ny) = self.topleft_rel(x, y);
        if nx > self.width || ny > self.height {
            return;
        }

        self.pixels[nx][ny] = color
    }

    /// Accepts a topleft-relative coordinate and translates it to a center-relative coordinate
    ///
    /// e.g. screen size of 1920x1080, (0, 0) -> (-960, 540)
    pub fn center_rel(&self, x: usize, y: usize) -> (isize, isize) {
        let nx = x as isize;
        let ny = y as isize;

        (
            -(self.width as isize / 2) + nx,
            (self.height as isize / 2) - ny,
        )
    }

    /// Accepts a center-relative coordinate and translates it to a top-left relative coordinate
    ///
    /// e.g. screen size of 1920x1080, (-960, 540) -> (0, 0)
    pub fn topleft_rel(&self, x: isize, y: isize) -> (usize, usize) {
        (
            ((self.width as isize / 2) + x) as usize,
            ((self.height as isize / 2) - y - 1) as usize,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub bg_color: [u8; 3],
}

impl Scene {
    pub fn trace_ray(
        &self,
        origin: Pos3,
        d: Vec3,
        t_min: f64,
        t_max: f64,
        depth: usize,
    ) -> [u8; 3] {
        let (closest, closest_sphere) = self.get_closest_intersection(origin, d, t_min, t_max);

        match closest_sphere {
            None => self.bg_color,
            Some(sph) => {
                let p = origin + closest * d;
                let n = p.difference(sph.center).to_normalized();

                let local_color: Color =
                    Color(sph.color) * self.compute_lighting(p, n, -d, sph.surface);
                let reflect = sph.reflective;

                if depth == 0 || reflect == 0.0 {
                    return local_color.into();
                }
                let recurse_ray = -d.reflect(n);
                let reflect_color: Color = self
                    .trace_ray(p, recurse_ray, 0.001, t_max, depth - 1)
                    .into();

                ((local_color * (1.0 - reflect)) + (reflect_color * reflect)).into()
            }
        }
    }

    pub fn get_closest_intersection(
        &self,
        origin: Pos3,
        d: Vec3,
        t_min: f64,
        t_max: f64,
    ) -> (f64, Option<&Sphere>) {
        let mut closest = f64::MAX;
        let mut closest_sphere: Option<&Sphere> = None;

        for sphere in self.spheres.iter() {
            let (t1, t2) = self.find_intersections(origin, d, sphere);

            if (t_min..t_max).contains(&t1) && t1 < closest {
                closest = t1;
                closest_sphere = Some(sphere);
            }
            if (t_min..t_max).contains(&t2) && t2 < closest {
                closest = t2;
                closest_sphere = Some(sphere);
            }
        }

        (closest, closest_sphere)
    }

    pub fn find_intersections(&self, origin: Pos3, d: Vec3, sphere: &Sphere) -> (f64, f64) {
        let co = origin.difference(sphere.center);
        let a = d * d;
        let b = 2.0 * (co * d);
        let c = (co * co) - sphere.radius.powi(2);

        let discr = b.powi(2) - (4.0 * a * c);

        if discr.is_sign_negative() {
            return (f64::MAX, f64::MAX);
        }

        let t1 = (-b + discr.sqrt()) / (2.0 * a);
        let t2 = (-b - discr.sqrt()) / (2.0 * a);

        (t1, t2)
    }

    pub fn compute_lighting(&self, point: Pos3, normal: Vec3, v: Vec3, specular: Surface) -> f64 {
        let mut i = 0.0;

        for light in self.lights.iter() {
            match light {
                Light::Ambient(val) => i += val,
                Light::Point(val, pos) => {
                    let l = pos.difference(point);
                    let nl = normal * l;

                    let (_, shadow_sphere) =
                        self.get_closest_intersection(point, l, 0.0001, f64::MAX);
                    if shadow_sphere.is_some() {
                        continue;
                    }

                    // diffuse
                    if nl.is_sign_positive() {
                        i += val * nl / (normal.magnitude() * l.magnitude());
                    }

                    // specular
                    if let Surface::Shiny(s) = specular {
                        let r = l.reflect(normal);
                        let rv = r * v;
                        if rv.is_sign_positive() {
                            i += val * (rv / (r.magnitude() * v.magnitude())).powf(s);
                        }
                    }
                }
                Light::Directional(val, dir) => {
                    let nl = normal * *dir;

                    let (_, shadow_sphere) =
                        self.get_closest_intersection(point, *dir, 0.001, f64::MAX);
                    if shadow_sphere.is_some() {
                        continue;
                    }

                    // diffuse
                    if nl.is_sign_positive() {
                        i += val * nl / (normal.magnitude() * dir.magnitude());
                    }

                    // specular
                    if let Surface::Shiny(s) = specular {
                        let r = dir.reflect(normal);
                        let rv = r * v;
                        if rv.is_sign_positive() {
                            i += val * (rv / (r.magnitude() * v.magnitude())).powf(s);
                        }
                    }
                }
            }
        }

        i
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Pos3,
    pub radius: f64,
    pub color: [u8; 3],
    pub surface: Surface,
    pub reflective: f64,
}

impl Sphere {
    pub fn new(
        center: Pos3,
        radius: f64,
        color: [u8; 3],
        surface: Surface,
        reflective: f64,
    ) -> Self {
        assert!(reflective >= 0.0 && reflective <= 1.0);
        Self {
            center,
            radius,
            color,
            surface,
            reflective,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Light {
    Ambient(f64),
    Point(f64, Pos3),
    Directional(f64, Vec3),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Surface {
    Matte,
    Shiny(f64),
}
