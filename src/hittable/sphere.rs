use std::rc::Rc;

use crate::math::Point3;

use super::{HitRecord, Hittable};
use crate::{materials::Material, math::Interval};

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new<M: Material + 'static>(center: Point3, radius: f32, mat: Rc<M>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::math::Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - ray.orig();

        let a = ray.dir().length_squared();
        let h = ray.dir().dot(oc);
        let c = oc.length_squared() - (self.radius * self.radius);

        let d = h * h - a * c;
        if d < 0.0 {
            return None;
        }

        let mut root = (h - d.sqrt()) / a;
        if root <= ray_t.min() || root >= ray_t.max() {
            root = (h + d.sqrt()) / a;

            if root <= ray_t.min() || root >= ray_t.max() {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let normal = (hit_point - self.center).normalize();
        let front_face = ray.dir().dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };
        let hit_record = HitRecord::new(hit_point, normal, root, front_face, Rc::clone(&self.mat));

        Some(hit_record)
    }
}
