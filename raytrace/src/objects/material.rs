use crate::Color;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shine: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shine: 200.0,
        }
    }
}

impl Material {
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shine: f32) -> Self {
        Self { color, ambient, diffuse, specular, shine }
    }
}
