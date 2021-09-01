use std::fs::OpenOptions;

mod color;
mod ray;
mod vec3;

use vec3::Vec3d;

use crate::ray::Ray;
use crate::vec3::Color3d;
use crate::{color::write_color, vec3::Point3d};
use std::io::{self, Write};

fn ray_color(r: &Ray) -> Color3d {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color3d::only(1.0) + t * Color3d::new(0.5, 0.7, 1.0)
}

fn main() {
    scene2();
}

fn scene1() -> io::Result<()> {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    let mut fp = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./result/scene1.ppm")
        .expect("cannot open file");

    fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel_color = vec3::Color3d::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );

            write_color(&mut fp, pixel_color);
        }
    }
    Ok(())
}

fn scene2() -> io::Result<()> {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin = Point3d::zero();
    let horizontal = Vec3d::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3d::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3d::new(0.0, 0.0, FOCAL_LENGTH);

    let mut fp = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./result/blue-to-white.ppm")
        .expect("cannot open file");

    fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&r);
            write_color(&mut fp, pixel_color);
        }
    }
    Ok(())
}
