use crate::color::Color;
use crate::math::{Ray, Vector3Ext, rand_unit_vec3};

use super::Material;

pub struct Lambertian {
    albedo: Color,
}

pub struct LambertianConfig {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(config: LambertianConfig) -> Self {
        Self {
            albedo: config.albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &crate::hittable::HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit.normal() + rand_unit_vec3();
        if scatter_direction.near_zero() {
            scatter_direction = *hit.normal();
        }

        Some((self.albedo, Ray::new(*hit.p(), scatter_direction)))
    }
}
