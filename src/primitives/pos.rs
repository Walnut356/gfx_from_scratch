use std::ops;

use crate::Vec3;

#[derive(Debug, Clone, Default, Copy)]
pub struct Pos3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Pos3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
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

#[derive(Debug, Clone, Default, Copy)]
pub struct Pos2 {
    pub x: f64,
    pub y: f64,
}

impl Pos2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}