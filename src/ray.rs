use crate::vec3::{Point3d, Vec3, Vec3d};

pub struct Ray {
    orig: Point3d,
    dir: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Point3d, direction: Vec3d) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3d {
        self.orig
    }

    pub fn direction(&self) -> Vec3d {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3d {
        self.orig + self.dir * t
    }
}
