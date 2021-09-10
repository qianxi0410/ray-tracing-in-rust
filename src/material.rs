use std::ops::Neg;

use num_traits::pow;

use crate::{
    hittable::HitRecord,
    ray::Ray,
    utils,
    vec3::{Color3d, Vec3d},
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color3d, Ray)>;
}

impl Material for Box<dyn Material> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)> {
        self.as_ref().scatter(ray_in, hit_record)
    }
}

impl<M: Material> Material for Box<M> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Color3d, Ray)> {
        self.as_ref().scatter(ray_in, hit_record)
    }
}

#[derive(Clone)]

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

#[derive(Clone, Copy)]
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
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3d::random_in_unit_sphere(),
        );
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
#[derive(Clone)]
pub struct Dieletric {
    ir: f64,
}

impl Dieletric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;

        r0 + (1.0 - r0) * pow(1.0 - cosine, 5)
    }
}

impl Material for Dieletric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color3d, Ray)> {
        let attenuation = Color3d::only(1.0);
        let refraction_ratio = if rec.font_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction().unit_vector();
        // let refracted = unit_direction.refract(&rec.normal, refraction_ratio);

        let cos_theta = unit_direction.neg().dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || Dieletric::reflectance(cos_theta, refraction_ratio) > utils::random_double()
        {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        Some((attenuation, Ray::new(rec.p, direction)))
    }
}
