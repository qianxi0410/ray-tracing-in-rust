use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color3d, Vec3d},
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color3d, Ray)>;
}

pub struct Diffuse {
    albedo: Color3d,
}

impl Diffuse {
    pub fn new(a: Color3d) -> Self {
        Self { albedo: a }
    }
}

impl Material for Diffuse {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color3d, Ray)> {
        let mut scatter_direction = rec.normal + Vec3d::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some((self.albedo, Ray::new(rec.p, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Color3d,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color3d, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color3d, Ray)> {
        let reflected = r_in.direction().unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(rec.p, self.fuzz * Vec3d::random_in_unit_sphere());
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
