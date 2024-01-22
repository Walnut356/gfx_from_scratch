#![allow(clippy::approx_constant)]

use crate::{Pos3, Matrix, Vec3, objects::material::Material};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub transform: Matrix,
    pub t_inverted: Matrix,
    pub t_transposed: Matrix,
    pub t_invert_transp: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new(
        transform: Matrix,
        material: Material
    ) -> Self {
        let t_inverted = transform.inverted().unwrap();
        let t_transposed = transform.transposed();
        let t_invert_transp = t_inverted.transposed();
        Self {
            transform,
            t_inverted,
            t_transposed,
            t_invert_transp,
            material,
        }
    }

    pub fn set_transform(mut self, transform: Matrix) -> Self {
        self.transform = transform;
        self
    }

    pub fn normal_at(&self, point: Pos3) -> Vec3 {
        let object_point = &self.t_inverted * point;
        let dist = object_point - Pos3::new(0.0, 0.0, 0.0);

        (&self.t_invert_transp * dist).to_normalized()
    }
}

#[test]
pub fn test_normal() {
    let sphere = Sphere::new(Matrix::translation(0.0, 1.0, 0.0), Material::default());
    let normal = sphere.normal_at(Pos3::new(0.0, 1.70711, -0.70711));

    assert_eq!(normal, Vec3::new(0.0, 0.70711, -0.70711));
}