use crate::{
    hittable::{HitRecord, Hittable},
    sphere::Sphere,
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
