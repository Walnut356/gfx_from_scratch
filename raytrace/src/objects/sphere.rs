use crate::{Pos3, Surface, Matrix};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub transform: Matrix,
    pub color: [u8; 3],
    pub surface: Surface,
    pub reflective: f32,
}

impl Sphere {
    pub fn new(
        transform: Matrix,
        color: [u8; 3],
        surface: Surface,
        reflective: f32,
    ) -> Self {
        assert!((0.0..=1.0).contains(&reflective));
        Self {
            transform,
            color,
            surface,
            reflective,
        }
    }

    pub fn set_transform(mut self, transform: Matrix) -> Self {
        self.transform = transform;
        self
    }
}