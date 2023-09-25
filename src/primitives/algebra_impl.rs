use std::ops::*;
use crate::primitives::{pos::*, vector::*};

// ---------------------------------------------------------------------------------------------- //
//                                           3D Position                                          //
// ---------------------------------------------------------------------------------------------- //

impl Pos3 {
    /// Returns a Vector3D representing the difference between 2 points.
    pub fn difference(&self, rhs: Pos3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for Pos3 {
    type Output = Self;

    /// Naively subtracts 2 points from eachother, returning a Pos3. For a Vec3 result, try
    /// `Pos3::difference()`
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<Vec3> for Pos3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// ---------------------------------------------------------------------------------------------- //
//                                            3D Vector                                           //
// ---------------------------------------------------------------------------------------------- //

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}   

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul for Vec3 {
    type Output = f64;

    /// Dot product. For cross product, see `Vec3::cross_product`
    fn mul(self, rhs: Vec3) -> Self::Output {
        let temp = Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };

        temp.x + temp.y + temp.z
    }
}


// ---------------------------------------------------------------------------------------------- //
//                                               2D                                               //
// ---------------------------------------------------------------------------------------------- //