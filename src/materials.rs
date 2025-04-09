pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub use dielectric::{Dielectric, DielectricConfig};
pub use lambertian::{Lambertian, LambertianConfig};
pub use metal::{Metal, MetalConfig};

use crate::{color::Color, hittable::HitRecord, math::Ray};

pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}
