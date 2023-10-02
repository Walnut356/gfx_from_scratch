use std::sync::Arc;

use crate::{objects::{Sphere, material::Material}, scene::Intersection, Matrix, Object, Pos3, Vec3, Ray};

#[derive(Debug, Clone)]
pub struct Viewport {
    pub position: Pos3,
    pub width: f32,
    pub height: f32,
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
    pub fn new(position: Pos3, width: f32, height: f32) -> Self {
        Self {
            position,
            width,
            height,
        }
    }

    pub fn ray_from_coord(
        &self,
        x: isize,
        y: isize,
        canvas_width: usize,
        canvas_height: usize,
    ) -> Ray {
        let x = x as f32 * (self.width / (canvas_width as f32));
        let y = y as f32 * (self.height / (canvas_height as f32));

        Ray::new(self.position, Vec3::new(x, y, 1.0))
    }
}
