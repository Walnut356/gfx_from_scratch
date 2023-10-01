use std::ops;

use crate::{Vec3, float_eq};

#[derive(Debug, Clone, Default, Copy)]
pub struct Pos3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Pos3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn iter(&self) -> impl Iterator<Item=f32> {
        [self.x, self.y, self.z].into_iter()
    }

    /// Naively subtracts 2 points from eachother, returning a Pos3. For a Vec3 result, try
    /// subtraction
    pub fn sub_naive(&self, rhs: Pos3) -> Pos3 {
        Pos3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl FromIterator<f32> for Pos3 {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        let mut i = iter.into_iter();
        Self {
            x: i.next().unwrap(),
            y: i.next().unwrap(),
            z: i.next().unwrap(),
        }
    }
}

impl From<Vec3> for Pos3 {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl ops::Sub for Pos3 {
    type Output = Vec3;

    /// Returns the difference between 2 points as a, returning a Vec3
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Pos3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Pos3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }

}

impl ops::Add<Vec3> for Pos3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl PartialEq for Pos3 {
    fn eq(&self, other: &Self) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
    }
}
