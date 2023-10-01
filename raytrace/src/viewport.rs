use std::sync::Arc;

use crate::{Pos3, objects::Sphere, Vec3, Object, scene::Intersection, Matrix};

#[derive(Debug, Clone)]
pub struct Viewport {
    pub position: Pos3,
    pub width: f32,
    pub height: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            position: Pos3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Viewport {
    pub fn new(position: Pos3, width: f32, height: f32) -> Self {
        Self {
            position,
            width,
            height,
        }
    }

    pub fn ray_from_coord(&self, x: isize, y: isize, canvas_width: usize, canvas_height: usize) -> Ray {
        let x = x as f32 * (self.width / (canvas_width as f32));
        let y = y as f32 * (self.height / (canvas_height as f32));

        Ray::new(
            self.position,
        Vec3::new(x, y, 1.0),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    pub origin: Pos3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Pos3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn position(&self, time: f32) -> Pos3 {
        self.origin + (self.dir * time)
    }

    pub fn transform(&self, matrix: &Matrix) -> Self {
        Self {
            origin: matrix * self.origin,
            dir: matrix * self.dir,
        }
    }

    pub fn sphere_intersect(&self, sphere: &Arc<Sphere>) -> Vec<Intersection> {
        let ray_tf = self.transform(&sphere.transform.invert().unwrap());
        let sphr_to_ray = ray_tf.origin - Pos3::new(0.0, 0.0, 0.0);
        let a = ray_tf.dir * ray_tf.dir;
        let b = 2.0 * (sphr_to_ray * ray_tf.dir);
        let c = (sphr_to_ray * sphr_to_ray) - 1.0;

        let discr = b.powi(2) - (4.0 * a * c);

        let t1 = (-b + discr.sqrt()) / (2.0 * a);
        let t2 = (-b - discr.sqrt()) / (2.0 * a);


        let mut result = vec![Intersection::new(t1, Object::Sphere(sphere.clone())), Intersection::new(t2, Object::Sphere(sphere.clone()))];

        result.sort();

        result
    }
}

#[test]
pub fn test_ray_transform() {
    let ray_1 = Ray::new(Pos3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 1.0, 0.0));
    let test_1 = Matrix::translation(3.0, 4.0, 5.0);

    assert_eq!(ray_1.transform(&test_1), Ray::new(Pos3::new(4.0, 6.0, 8.0), Vec3::new(0.0, 1.0, 0.0)));

    let test_2 = Matrix::scaling(2.0, 3.0, 4.0);

    assert_eq!(ray_1.transform(&test_2), Ray::new(Pos3::new(2.0, 6.0, 12.0), Vec3::new(0.0, 3.0, 0.0)));
}

#[test]
pub fn test_ray_intersect() {
    let ray_1 = Ray::new(Pos3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
    let sphere = Arc::new(Sphere::new(Matrix::scaling(2.0, 2.0, 2.0), [255, 0, 0], crate::Surface::Matte, 0.0));

    let intersects = ray_1.sphere_intersect(&sphere);

    assert_eq!(intersects[0].t, 3.0);
    assert_eq!(intersects[1].t, 7.0);
}