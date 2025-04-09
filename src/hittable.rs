pub mod sphere;

use std::rc::Rc;

use crate::math::{Point3, Vector3};

use crate::{
    materials::Material,
    math::{Interval, Ray},
};

pub struct HitRecord {
    p: Point3,
    normal: Vector3,
    t: f32,
    front_face: bool,
    mat: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vector3,
        t: f32,
        front_face: bool,
        mat: Rc<dyn Material>,
    ) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
            mat,
        }
    }

    pub fn p(&self) -> &Point3 {
        &self.p
    }

    pub fn normal(&self) -> &Vector3 {
        &self.normal
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn mat(&self) -> &Rc<dyn Material> {
        &self.mat
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max();
        let mut hit_record = None;

        let mut interval = Interval::new(ray_t.min(), closest_so_far);
        for object in &self.objects {
            interval.set_max(closest_so_far);
            if let Some(hit) = object.hit(ray, &interval) {
                closest_so_far = hit.t();
                hit_record = Some(hit);
            }
        }

        hit_record
    }
}
