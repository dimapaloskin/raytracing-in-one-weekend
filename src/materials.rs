pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::{color::Color, hittable::HitRecord, math::Ray};

pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}
