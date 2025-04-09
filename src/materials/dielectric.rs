use crate::color::Color;
use crate::math::{Ray, Vector3, rand, vec3_reflect, vec3_refract_with_cos};
use std::ops::Neg;

use super::Material;

pub struct Dielectric {
    refraction_index: f32,
}

pub struct DielectricConfig {
    pub refraction_index: f32,
}

impl Dielectric {
    pub fn new(config: DielectricConfig) -> Self {
        Self {
            refraction_index: config.refraction_index,
        }
    }
}

impl Dielectric {
    fn reflectance(cos: f32, ref_idx: f32) -> f32 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &crate::hittable::HitRecord) -> Option<(Color, Ray)> {
        let ri = if hit.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = ray.dir().normalize();
        let cos = unit_dir.neg().dot(*hit.normal()).min(1.0);
        let sin = (1.0 - cos * cos).sqrt();

        let cannot_refract = ri * sin > 1.0;
        let dir: Vector3;

        if cannot_refract || Dielectric::reflectance(cos, ri) > rand() {
            dir = vec3_reflect(&unit_dir, &hit.normal());
        } else {
            dir = vec3_refract_with_cos(&unit_dir, &hit.normal(), ri, cos);
        }

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(*hit.p(), dir)))
    }
}
