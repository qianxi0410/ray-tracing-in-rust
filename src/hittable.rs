use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3d, Vec3d},
};

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3d,
    pub normal: Vec3d,
    pub t: f64,
    pub font_face: bool,
    pub material: &'a (dyn Material),
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3d) {
        self.font_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.font_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }

    pub fn new_with_face_normal(
        t: f64,
        point: Point3d,
        outward_normal: Vec3d,
        material: &'a (dyn Material),
        ray: &Ray,
    ) -> Self {
        let font_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if font_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            p: point,
            normal,
            t,
            font_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for Box<dyn Hittable> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.as_ref().hit(r, t_min, t_max)
    }
}
