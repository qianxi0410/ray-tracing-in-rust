use crate::{
    ray::Ray,
    vec3::{Point3d, Vec3d},
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3d,
    pub normal: Vec3d,
    pub t: f64,
}

pub trait Hittable {
    fn hit(r: &Ray, t_mint: f64, t_max: f64, rec: &HitRecord) -> bool;
}
