use crate::{
    ray::Ray,
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

    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        Self {
            origin: Point3d::only(0.0),
            horizontal: Vec3d::new(viewport_width, 0.0, 0.0),
            vertical: Vec3d::new(0.0, viewport_height, 0.0),
            lower_left_corner: Point3d::only(0.0)
                - Vec3d::new(viewport_width, 0.0, 0.0) / 2.0
                - Vec3d::new(0.0, viewport_height, 0.0) / 2.0
                - Vec3d::new(0.0, 0.0, focal_length),
        }
    }
}
