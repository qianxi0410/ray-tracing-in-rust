use std::borrow::Borrow;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{Point3d, Vec3d},
};

#[derive(Clone)]
pub struct Sphere<M>
where
    M: Material,
{
    pub center: Point3d,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(cen: Point3d, r: f64, m: M) -> Self {
        Self {
            center: cen,
            radius: r,
            material: m,
        }
    }
}

fn solve_sphere_equation(
    ray: &Ray,
    center: Point3d,
    radius: f64,
    t_min: f64,
    t_max: f64,
) -> Option<(f64, Point3d, Vec3d)> {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(&ray.direction());
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let mut root = (-half_b - sqrt_d) / a;
    if root < t_min || root > t_max {
        root = (-half_b + sqrt_d) / a;
        if root < t_min || root > t_max {
            return None;
        }
    }

    let point = ray.at(root);
    Some((root, point, (point - center) / radius))
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        solve_sphere_equation(ray, self.center, self.radius, t_min, t_max).map(
            |(root, point, outward_normal)| {
                HitRecord::new_with_face_normal(
                    root,
                    point,
                    outward_normal,
                    self.material.borrow(),
                    ray,
                )
            },
        )
    }
}
