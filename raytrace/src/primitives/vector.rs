use std::ops;

use crate::{float_eq, Pos3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn iter(&self) -> impl Iterator<Item = f32> {
        [self.x, self.y, self.z].into_iter()
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn to_normalized(&self) -> Vec3 {
        *self / self.magnitude()
    }

    /// Returns a vector that is perpendicular to both given vectors. Order matters.
    pub fn cross_product(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.x * rhs.z) - (self.z * rhs.x),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    /// reflects self via a given Normal vector
    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - (2.0 * normal * (normal * *self))
    }
}

impl FromIterator<f32> for Vec3 {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        Self {
            x: i.next().unwrap(),
            y: i.next().unwrap(),
            z: i.next().unwrap(),
        }
    }
}

impl From<Pos3> for Vec3 {
    fn from(value: Pos3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    /// Scalar multiplication
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    /// Scalar multiplication
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = f32;

    /// Dot product. For cross product, see `Vec3::cross_product`
    fn mul(self, rhs: Vec3) -> Self::Output {
        let temp = [self.x * rhs.x, self.y * rhs.y, self.z * rhs.z];

        temp.into_iter().sum()
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
    }
}
