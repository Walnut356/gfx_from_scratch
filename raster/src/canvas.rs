use crate::prelude::*;
use image::{Rgb, RgbImage};

#[derive(Debug, Clone)]
pub struct Canvas {
    image: RgbImage,
    width: u32,
    height: u32,
}

impl Canvas {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            image: RgbImage::new(width, height),
            width,
            height,
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn save(&self, file_name: &str) {
        self.image.save(format!("./{file_name}.png")).unwrap();
    }

    #[inline]
    pub fn draw_pixel(&mut self, point: Point, color: Rgb<u8>) {
        self.image.put_pixel(point.x, point.y, color)
    }

    pub fn draw_line(&mut self, mut p1: Point, mut p2: Point, color: Rgb<u8>) {
        if (p2.x as f32 - p1.x as f32).abs() > (p2.y as f32 - p1.y as f32).abs() {
            // ~horizontal
            if p1.x > p2.x {
                std::mem::swap(&mut p1, &mut p2);
            }
            let y_vals = p1.interp(p2);
            for x in p1.x..p2.x {
                self.draw_pixel(p![x, y_vals[(x - p1.x) as usize] as u32], color);
            }
        } else {
            // ~vertical
            if p1.y > p2.y {
                std::mem::swap(&mut p1, &mut p2);
            }
            let x_vals = p1.invert().interp(p2.invert());
            for y in p1.y..p2.y {
                self.draw_pixel(p![x_vals[(y - p1.y) as usize] as u32, y], color);
            }
        }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        let width: u32 = 1000;
        let height: u32 = 1000;
        Self {
            image: RgbImage::new(width, height),
            width,
            height,
        }
    }
}
