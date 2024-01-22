pub mod point;
pub mod canvas;
pub use canvas::Canvas;

pub mod prelude {
    pub use crate::{p, point::Point, canvas::Canvas, BLACK, RED, GREEN, BLUE, WHITE, GRAY};
}

use image::Rgb;

pub const BLACK: Rgb<u8> = Rgb([0, 0, 0,]);
pub const RED: Rgb<u8> = Rgb([255, 0, 0]);
pub const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
pub const BLUE: Rgb<u8> = Rgb([0, 0, 255]);
pub const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
pub const GRAY: Rgb<u8> = Rgb([128, 128, 128]);