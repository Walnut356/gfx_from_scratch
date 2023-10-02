use std::ops;

use crate::float_eq;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {
    pub const WHITE: Color = Color(1.0, 1.0, 1.0);
    pub const BLACK: Color = Color(0.0, 0.0, 0.0);
    pub const RED: Color = Color(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color(0.0, 0.0, 1.0);
    pub const YELLOW: Color = Color(1.0, 1.0, 0.0);
    pub const CYAN: Color = Color(0.0, 1.0, 1.0);
}

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color(
            self.0 as f32 * rhs,
            self.1 as f32 * rhs,
            self.2 as f32 * rhs,
        )
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color(
            self.0 as f32 * rhs.0,
            self.1 as f32 * rhs.1,
            self.2 as f32 * rhs.2,
        )
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
        )
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
        )
    }
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Color(
            value[0] as f32 / 255.0,
            value[1] as f32 / 255.0,
            value[2] as f32 / 255.0,
        )
    }
}

impl From<Color> for [u8; 3] {
    fn from(value: Color) -> Self {
        [
            (value.0 * 255.0).min(255.0) as u8,
            (value.1 * 255.0).min(255.0) as u8,
            (value.2 * 255.0).min(255.0) as u8,
        ]
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.0, other.0) && float_eq(self.1, other.1) && float_eq(self.2, other.2)
    }
}