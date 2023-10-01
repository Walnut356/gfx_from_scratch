use std::sync::Arc;

use crate::{objects::Sphere, Color, Light, Object, Pos3, Ray, Surface, Vec3};

#[derive(Debug, Clone)]
pub struct Scene {
    pub spheres: Vec<Arc<Sphere>>,
    pub lights: Vec<Light>,
    pub bg_color: [u8; 3],
}

impl Scene {
    pub fn trace_ray(&self, ray: Ray, t_min: f32, t_max: f32, depth: usize) -> [u8; 3] {
        let intersects = self.get_intersections(&ray, t_min, t_max);
        let closest = self.get_closest(intersects);

        match closest {
            None => self.bg_color,
            Some(hit) => {
                if let Object::Sphere(obj) = hit.obj {
                    obj.color
                } else {
                    todo!()
                }
                // let p = ray.origin + hit.t * ray.dir;
                // let n = (p - sph.center).to_normalized();

                // let local_color: Color =
                //     Color(sph.color) * self.compute_lighting(p, n, -ray.dir, sph.surface);
                // let reflect = sph.reflective;

                // if depth == 0 || reflect == 0.0 {
                //     return local_color.into();
                // }
                // let recurse_ray = -ray.dir.reflect(n);
                // let reflect_color: Color = self
                //     .trace_ray(Ray::new(p, recurse_ray), 0.001, t_max, depth - 1)
                //     .into();

                // ((local_color * (1.0 - reflect)) + (reflect_color * reflect)).into()
            }
        }
    }

    pub fn get_intersections(&self, ray: &Ray, t_min: f32, t_max: f32) -> Vec<Intersection> {
        let mut intersects: Vec<Vec<Intersection>> = Vec::new();

        for sphere in self.spheres.iter() {
            intersects.push(ray.sphere_intersect(sphere));
        }

        intersects.into_iter().flatten().collect()
    }

    pub fn get_closest(&self, intersects: Vec<Intersection>) -> Option<Intersection> {
        intersects
            .into_iter()
            .filter(|x| x.t.is_sign_positive())
            .min()
    }

    // pub fn compute_lighting(&self, point: Pos3, normal: Vec3, v: Vec3, specular: Surface) -> f32 {
    //     let mut i = 0.0;

    //     for light in self.lights.iter() {
    //         match light {
    //             Light::Ambient(val) => i += val,
    //             Light::Point(val, pos) => {
    //                 let l = *pos - point;
    //                 let nl = normal * l;

    //                 let (_, shadow_sphere) =
    //                     self.get_intersections(&Ray::new(point, l), 0.0001, f32::MAX);
    //                 if shadow_sphere.is_some() {
    //                     continue;
    //                 }

    //                 // diffuse
    //                 if nl.is_sign_positive() {
    //                     i += val * nl / (normal.magnitude() * l.magnitude());
    //                 }

    //                 // specular
    //                 if let Surface::Shiny(s) = specular {
    //                     let r = l.reflect(normal);
    //                     let rv = r * v;
    //                     if rv.is_sign_positive() {
    //                         i += val * (rv / (r.magnitude() * v.magnitude())).powf(s);
    //                     }
    //                 }
    //             }
    //             Light::Directional(val, dir) => {
    //                 let nl = normal * *dir;

    //                 let (_, shadow_sphere) =
    //                     self.get_intersections(&Ray::new(point, *dir), 0.001, f32::MAX);
    //                 if shadow_sphere.is_some() {
    //                     continue;
    //                 }

    //                 // diffuse
    //                 if nl.is_sign_positive() {
    //                     i += val * nl / (normal.magnitude() * dir.magnitude());
    //                 }

    //                 // specular
    //                 if let Surface::Shiny(s) = specular {
    //                     let r = dir.reflect(normal);
    //                     let rv = r * v;
    //                     if rv.is_sign_positive() {
    //                         i += val * (rv / (r.magnitude() * v.magnitude())).powf(s);
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     i
    // }
}

/// Used to track rays intersecting with objects. **All comparison operations are done on the
/// distance value `t`**
#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f32,
    pub obj: Object,
}

impl Intersection {
    pub fn new(t: f32, obj: Object) -> Self {
        Self { t, obj }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl Eq for Intersection {}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.t.total_cmp(&other.t)
    }
}
