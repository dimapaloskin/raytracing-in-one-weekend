use crate::math::{Point3, Vector3};

#[derive(Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vector3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vector3) -> Self {
        Self { orig, dir }
    }

    pub fn orig(&self) -> &Point3 {
        &self.orig
    }

    pub fn dir(&self) -> &Vector3 {
        &self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}
