pub mod primitives {
    pub mod algebra_impl;
    pub mod pos;
    pub mod vector;
}
pub use primitives::{
    pos::{Pos2, Pos3},
    vector::{Vec2, Vec3},
};

use image::Rgb;

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
    pub bg_color: [u8; 3],
}

impl Scene {
    pub fn trace_ray(&self, ray_origin: Pos3, d: Vec3, t_min: f64, t_max: f64) -> [u8; 3] {
        let mut closest = f64::MAX;
        let mut closest_sphere: Option<&Sphere> = None;

        for sphere in self.spheres.iter() {
            let (t1, t2) = self.find_intersections(ray_origin, d, sphere);

            if (t_min..t_max).contains(&t1) && t1 < closest {
                closest = t1;
                closest_sphere = Some(sphere);
            }
            if (t_min..t_max).contains(&t2) && t2 < closest {
                closest = t2;
                closest_sphere = Some(sphere);
            }
        }

        match closest_sphere {
            None => self.bg_color,
            Some(sph) => sph.color,
        }
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
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Pos3,
    pub radius: f64,
    pub color: [u8; 3],
}

impl Sphere {
    pub fn new(center: Pos3, radius: f64, color: [u8; 3]) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }
}
