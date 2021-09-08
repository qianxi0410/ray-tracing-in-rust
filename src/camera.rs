use crate::{
    ray::Ray,
    utils::degrees_to_radians,
    vec3::{Point3d, Vec3d},
};

pub struct Camera {
    origin: Point3d,
    lower_left_corner: Point3d,
    horizontal: Vec3d,
    vertical: Vec3d,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }

    pub fn new(
        lookfrom: Point3d,
        lookat: Point3d,
        vup: Vec3d,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        // let aspect_ratio = 16.0 / 9.0;
        // let viewport_height = 2.0;
        // let viewport_width = aspect_ratio * viewport_height;
        // let focal_length = 1.0;

        let thea = degrees_to_radians(vfov);
        let h = (thea / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3d::unit_vector(&(lookfrom - lookat));
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        Self {
            origin: lookfrom,
            horizontal: viewport_width * u,
            vertical: viewport_height * v,
            lower_left_corner: lookfrom - viewport_width * u / 2.0 - viewport_height * v / 2.0 - w,
        }
    }
}
