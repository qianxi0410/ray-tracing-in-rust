use crate::{
    hittable::{HitRecord, Hittable},
    material::{Dieletric, Diffuse, Metal},
    sphere::Sphere,
    utils,
    vec3::{Color3d, Point3d, Vec3d},
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn push(&mut self, p: Box<dyn Hittable>) {
        self.objects.push(p);
    }

    pub fn random_scene() -> Self {
        let mut world = Self::new();

        let ground_material = Box::new(Diffuse::new(Color3d::only(0.5)));
        world.push(Box::new(Sphere::new(
            Point3d::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = utils::random_double();
                let center = Point3d::new(
                    a as f64 + 0.9 * utils::random_double(),
                    0.2,
                    b as f64 + 0.9 * utils::random_double(),
                );

                if (center - Point3d::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let albedo = Color3d::random() * Color3d::random();
                        let sphere = Sphere::new(center, 0.2, Box::new(Diffuse::new(albedo)));
                        world.push(Box::new(sphere));
                    } else if choose_mat < 0.95 {
                        let albedo = Color3d::random_range(0.5, 1.0);
                        let fuzz = utils::random_range(0.0, 0.5);
                        let sphere = Sphere::new(center, 0.2, Box::new(Metal::new(albedo, fuzz)));
                        world.push(Box::new(sphere));
                    } else {
                        let sphere = Sphere::new(center, 0.2, Box::new(Dieletric::new(1.5)));
                        world.push(Box::new(sphere));
                    }
                }
            }
        }
        world.push(Box::new(Sphere::new(
            Point3d::new(0.0, 1.0, 0.0),
            1.0,
            Box::new(Dieletric::new(1.5)),
        )));

        world.push(Box::new(Sphere::new(
            Point3d::new(-4.0, 1.0, 0.0),
            1.0,
            Box::new(Diffuse::new(Color3d::new(0.4, 0.2, 0.1))),
        )));

        world.push(Box::new(Sphere::new(
            Point3d::new(4.0, 1.0, 0.0),
            1.0,
            Box::new(Metal::new(Color3d::new(0.7, 0.6, 0.5), 0.0)),
        )));
        world
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(result) = object.hit(&r, t_min, closest_so_far) {
                closest_so_far = result.t;
                temp_rec = Some(result);
            }
        }
        temp_rec
    }
}
