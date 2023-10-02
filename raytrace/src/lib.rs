pub mod scene;
pub mod viewport;

pub mod primitives {
    pub mod color;
    pub mod matrix;
    pub mod pos;
    pub mod vector;
    pub mod ray;



}

pub mod objects {
    pub mod sphere;
    pub mod material;

    pub use sphere::Sphere;
}

use std::sync::Arc;

pub use primitives::{color::Color, matrix::Matrix, pos::Pos3, vector::Vec3, ray::Ray};
pub use viewport:: Viewport;
pub use scene::Scene;

use image::Rgb;

#[macro_export]
macro_rules! identity_matrix {
    () => {
        Matrix::from_vec(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    };
}

/// Accepts a center-relative coordinate and translates it to a top-left relative coordinate
///
/// e.g. screen size of 1920x1080, (-960, 540) -> (0, 0)
pub fn topleft_rel(width: usize, height: usize, x: isize, y: isize) -> (usize, usize) {
    (
        ((width as isize / 2) + x) as usize,
        ((height as isize / 2) - y - 1) as usize,
    )
}

pub fn float_eq(a: f32, b: f32) -> bool {
    (a - b).abs() <= 0.001
}

#[derive(Debug, Clone)]
pub struct PointLight {
    position: Pos3,
    intensity: Color
}

impl PointLight {
    pub fn new(position: Pos3, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Surface {
    Matte,
    Shiny(f32),
}

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Arc<objects::Sphere>),
}

// #[derive(Debug, Clone, Default)]
// pub struct Canvas {
//     pub width: usize,
//     pub height: usize,
//     pub pixels: Vec<Vec<[u8; 3]>>,
// }

// impl Canvas {
//     pub fn new(width: usize, height: usize) -> Self {
//         Self {
//             width,
//             height,
//             pixels: vec![vec![[0, 0, 0]; height]; width],
//         }
//     }

//     pub fn to_viewport(&self, x: isize, y: isize, vp: &Viewport) -> Vec3 {
//         let x = x as f32 * (vp.width / self.width as f32);
//         let y = y as f32 * (vp.height / self.height as f32);

//         Vec3::new(x, y, 1.0)
//     }

//     pub fn put_pixel(&mut self, x: isize, y: isize, color: [u8; 3]) {
//         let (nx, ny) = self.topleft_rel(x, y);
//         if nx > self.width || ny > self.height {
//             return;
//         }

//         self.pixels[nx][ny] = color
//     }

//     /// Accepts a topleft-relative coordinate and translates it to a center-relative coordinate
//     ///
//     /// e.g. screen size of 1920x1080, (0, 0) -> (-960, 540)
//     pub fn center_rel(&self, x: usize, y: usize) -> (isize, isize) {
//         let nx = x as isize;
//         let ny = y as isize;

//         (
//             -(self.width as isize / 2) + nx,
//             (self.height as isize / 2) - ny,
//         )
//     }

//     /// Accepts a center-relative coordinate and translates it to a top-left relative coordinate
//     ///
//     /// e.g. screen size of 1920x1080, (-960, 540) -> (0, 0)
//     pub fn topleft_rel(&self, x: isize, y: isize) -> (usize, usize) {
//         (
//             ((self.width as isize / 2) + x) as usize,
//             ((self.height as isize / 2) - y - 1) as usize,
//         )
//     }
// }
