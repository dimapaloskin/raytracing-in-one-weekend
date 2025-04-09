pub mod ray;

use rand::Rng;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;
use std::f32::EPSILON;
use std::ops::Neg;

use glam::Vec3A;
pub use ray::Ray;

pub type Vector3 = Vec3A;
pub type Point3 = Vec3A;

thread_local! {
    static RNG: std::cell::RefCell<Xoshiro256Plus> =
        std::cell::RefCell::new(Xoshiro256Plus::seed_from_u64(9));
}

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    pub fn default() -> Self {
        Self {
            min: std::f32::NEG_INFINITY,
            max: std::f32::INFINITY,
        }
    }

    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        match x {
            x if x < self.min => self.min,
            x if x > self.max => self.max,
            _ => x,
        }
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn set_max(&mut self, val: f32) {
        self.max = val;
    }
}

pub static EMPTY: Interval = Interval {
    min: std::f32::INFINITY,
    max: std::f32::NEG_INFINITY,
};

pub static UNIVERSE: Interval = Interval {
    min: std::f32::NEG_INFINITY,
    max: std::f32::INFINITY,
};

#[inline]
pub fn rand() -> f32 {
    RNG.with(|rng| rng.borrow_mut().random::<f32>())
}

#[inline]
pub fn rand_rng(min: f32, max: f32) -> f32 {
    RNG.with(|rng| rng.borrow_mut().random_range(min..max))
}

#[inline]
pub fn rand_vec3() -> Vector3 {
    Vector3::new(rand(), rand(), rand())
}

#[inline]
pub fn rand_rng_vec3(min: f32, max: f32) -> Vector3 {
    Vector3::new(rand_rng(min, max), rand_rng(min, max), rand_rng(min, max))
}

#[inline]
pub fn rand_unit_vec3() -> Vector3 {
    loop {
        let p = rand_rng_vec3(-1.0, 1.0);
        let len_sq = p.length_squared();
        if EPSILON < len_sq && len_sq < 1.0 {
            return p.normalize();
        }
    }
}

#[inline]
pub fn rand_hemnisphere_vec3(normal: &Vector3) -> Vector3 {
    let on_unit_sphere = rand_unit_vec3();
    if on_unit_sphere.dot(*normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub trait Vector3Ext {
    fn near_zero(&self) -> bool;
}

impl Vector3Ext for Vector3 {
    fn near_zero(&self) -> bool {
        let s = 1e-8;

        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

#[inline]
pub fn vec3_near_zero(vec: Vector3) -> bool {
    let s = 1e-8;

    (vec.x.abs() < s) && (vec.y.abs() < s) && (vec.z.abs() < s)
}

#[inline]
pub fn vec3_reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    v - 2.0 * v.dot(*n) * n
}

#[inline]
pub fn vec3_refract(v: &Vector3, normal: &Vector3, etai_over_etat: f32) -> Vector3 {
    let cos = v.neg().dot(*normal).min(1.0);
    let r_out_perp = etai_over_etat * (v + cos * normal);

    let r_out_par = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * normal;
    r_out_perp + r_out_par
}

#[inline]
pub fn vec3_refract_with_cos(
    v: &Vector3,
    normal: &Vector3,
    etai_over_etat: f32,
    cos: f32,
) -> Vector3 {
    let r_out_perp = etai_over_etat * (v + cos * normal);
    let r_out_par = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * normal;
    r_out_perp + r_out_par
}

#[inline]
pub fn vec3_rand_in_unit_disk() -> Vector3 {
    loop {
        let p = Vector3::new(rand_rng(-1.0, 1.0), rand_rng(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
