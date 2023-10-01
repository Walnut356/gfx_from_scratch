use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub [u8; 3]);

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color([
            (self.0[0] as f32 * rhs) as u8,
            (self.0[1] as f32 * rhs) as u8,
            (self.0[2] as f32 * rhs) as u8,
        ])
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color([
            (self.0[0].saturating_add(rhs.0[0])),
            (self.0[1].saturating_add(rhs.0[1])),
            (self.0[2].saturating_add(rhs.0[2])),
        ])
    }
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Color(value)
    }
}

impl From<Color> for [u8; 3] {
    fn from(value: Color) -> Self {
        value.0
    }
}