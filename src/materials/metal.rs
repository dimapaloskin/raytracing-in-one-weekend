use super::Material;
use crate::color::Color;
use crate::math::{Ray, rand_unit_vec3, vec3_reflect};

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

pub struct MetalConfig {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(config: MetalConfig) -> Self {
        Self {
            albedo: config.albedo,
            fuzz: config.fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &crate::hittable::HitRecord) -> Option<(Color, Ray)> {
        let reflected = vec3_reflect(ray.dir(), hit.normal());
        let reflected = reflected.normalize() + (self.fuzz * rand_unit_vec3());
        let scattered = Ray::new(*hit.p(), reflected);
        if scattered.dir().dot(*hit.normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
