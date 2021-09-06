use crate::{
    ray::Ray,
    vec3::{Point3d, Vec3d},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3d,
    pub normal: Vec3d,
    pub t: f64,
    pub font_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3d::only(0.0),
            normal: Vec3d::only(0.0),
            t: 0.0,
            font_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3d) {
        self.font_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.font_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
