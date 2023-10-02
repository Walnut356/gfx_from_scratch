use std::sync::Arc;

use crate::{
    identity_matrix,
    objects::{material::Material, Sphere},
    primitives::color,
    Color, Matrix, Object, PointLight, Pos3, Ray, Surface, Vec3,
};

#[derive(Debug)]
pub struct Scene {
    pub spheres: Vec<Arc<Sphere>>,
    pub lights: Vec<PointLight>,
    pub bg_color: [u8; 3],
}

impl Clone for Scene {
    fn clone(&self) -> Self {
        Self {
            spheres: self.spheres.clone(),
            lights: self.lights.clone(),
            bg_color: self.bg_color.clone(),
        }
    }
}

impl Scene {
    pub fn trace_ray(&self, ray: Ray, t_min: f32, t_max: f32, depth: usize) -> [u8; 3] {
        let intersects = self.get_intersections(&ray, t_min, t_max);
        let closest = self.get_closest(intersects);

        match closest {
            None => self.bg_color,
            Some(hit) => {
                if let Object::Sphere(obj) = hit.obj {
                    let point = ray.position(hit.t);
                    self.compute_lighting(point, obj.normal_at(point), ray.dir, &obj.material)
                        .into()
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

    pub fn compute_lighting(
        &self,
        point: Pos3,
        normal_vec: Vec3,
        cam_vec: Vec3,
        material: &Material,
    ) -> Color {
        let mut result = Color::BLACK;

        for light in &self.lights {
            let effective_color = material.color * light.intensity;
            let light_vec = (light.position - point).to_normalized();

            let ambient = effective_color * material.ambient;

            let l_dot_n = light_vec * normal_vec;

            let (diffuse, specular): (Color, Color) = if l_dot_n < 0.0 {
                (Color::BLACK, Color::BLACK)
            } else {
                let reflect_vec = -light_vec.reflect(normal_vec);
                let r_dot_c = reflect_vec * cam_vec;

                let specular = if r_dot_c <= 0.0 {
                    Color::BLACK
                } else {
                    let factor = r_dot_c.powf(material.shine);
                    light.intensity * material.specular * factor
                };

                (effective_color * material.diffuse * l_dot_n, specular)
            };

            result = result + ambient + diffuse + specular;
        }

        result
    }
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

#[test]
pub fn test_lighting_behindcam() {
    let spheres = vec![Arc::new(Sphere::new(
        identity_matrix!(),
        Material::new(Color(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0),
    ))];

    let scene = Scene {
        spheres,
        lights: vec![PointLight::new(
            Pos3::new(0.0, 0.0, -10.0),
            Color(1.0, 1.0, 1.0),
        )],
        bg_color: [0, 0, 0],
    };

    let result = scene.compute_lighting(
        Pos3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
        &scene.spheres[0].material,
    );

    assert_eq!(result, Color(1.9, 1.9, 1.9));
}

#[test]
pub fn test_lighting_eyeoffset() {
    let spheres = vec![Arc::new(Sphere::new(
        identity_matrix!(),
        Material::new(Color(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0),
    ))];

    let scene = Scene {
        spheres,
        lights: vec![PointLight::new(
            Pos3::new(0.0, 0.0, -10.0),
            Color(1.0, 1.0, 1.0),
        )],
        bg_color: [0, 0, 0],
    };

    let result = scene.compute_lighting(
        Pos3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 2.0_f32.sqrt() / 2.0, -(2.0_f32.sqrt() / 2.0)),
        &scene.spheres[0].material,
    );

    assert_eq!(result, Color(1.0, 1.0, 1.0));
}

#[test]
pub fn test_lighting_lightoffset() {
    let spheres = vec![Arc::new(Sphere::new(
        identity_matrix!(),
        Material::new(Color(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0),
    ))];

    let scene = Scene {
        spheres,
        lights: vec![PointLight::new(
            Pos3::new(0.0, 10.0, -10.0),
            Color(1.0, 1.0, 1.0),
        )],
        bg_color: [0, 0, 0],
    };

    let result = scene.compute_lighting(
        Pos3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
        &scene.spheres[0].material,
    );

    assert_eq!(result, Color(0.7364, 0.7364, 0.7364));
}

#[test]
pub fn test_lighting_bothoffset() {
    let spheres = vec![Arc::new(Sphere::new(
        identity_matrix!(),
        Material::new(Color(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0),
    ))];

    let scene = Scene {
        spheres,
        lights: vec![PointLight::new(
            Pos3::new(0.0, 10.0, -10.0),
            Color(1.0, 1.0, 1.0),
        )],
        bg_color: [0, 0, 0],
    };

    let result = scene.compute_lighting(
        Pos3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, -(2.0_f32.sqrt() / 2.0), -(2.0_f32.sqrt() / 2.0)),
        &scene.spheres[0].material,
    );

    assert_eq!(result, Color(1.6364, 1.6364, 1.6364));
}

#[test]
pub fn test_lighting_behindobj() {
    let spheres = vec![Arc::new(Sphere::new(
        identity_matrix!(),
        Material::new(Color(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0),
    ))];

    let scene = Scene {
        spheres,
        lights: vec![PointLight::new(
            Pos3::new(0.0, 0.0, 10.0),
            Color(1.0, 1.0, 1.0),
        )],
        bg_color: [0, 0, 0],
    };

    let result = scene.compute_lighting(
        Pos3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
        &scene.spheres[0].material,
    );

    assert_eq!(result, Color(0.1, 0.1, 0.1));
}
