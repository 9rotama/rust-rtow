use crate::{point3::Point3, vec3::Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> Point3 {
        Point3::new([self.orig.x(), self.orig.y(), self.orig.z()])
    }

    pub fn direction(&self) -> Vec3 {
        Vec3::new([self.dir.x(), self.dir.y(), self.dir.z()])
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin() + t * self.direction()
    }
}
