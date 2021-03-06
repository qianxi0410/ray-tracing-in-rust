use std::borrow::Borrow;
use std::f64::consts::PI;
use std::fs::OpenOptions;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use hittable::Hittable;
use material::Material;
use vec3::{Vec3, Vec3d};

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::material::{Dieletric, Diffuse, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::random_double;
use crate::vec3::Color3d;
use crate::{color::write_color, vec3::Point3d};
use std::io::{self, Write};

fn ray_color(r: &Ray) -> Color3d {
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color3d::only(1.0) + t * Color3d::new(0.5, 0.7, 1.0)
}

fn ray_color_sphere(r: &Ray) -> Color3d {
    if hit_sphere(&Point3d::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color3d::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color3d::only(1.0) + t * Color3d::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3d, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = r.direction().dot(&r.direction());
    let b = 2.0 * oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn hit_sphere_color(center: &Point3d, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(&r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color_sphere_color(r: &Ray) -> Color3d {
    let mut t = hit_sphere_color(&Point3d::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = (r.at(t) - Vec3d::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color3d::new(N.x + 1.0, N.y + 1.0, N.z + 1.0);
    }
    let unit_direction = r.direction().unit_vector();
    t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color3d::only(1.0) + t * Color3d::new(0.5, 0.7, 1.0)
}

fn ray_color_5(r: &Ray, word: &Hittable) -> Color3d {
    if let Some(result) = word.hit(r, 0.0, f64::INFINITY) {
        return 0.5 * (result.normal + Color3d::only(1.0));
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color3d::only(1.0) + t * Color3d::new(0.5, 0.7, 1.0)
}

fn ray_color_6(r: &Ray, word: &Hittable, depth: i32) -> Color3d {
    if depth <= 0 {
        return Color3d::only(0.0);
    }

    if let Some(result) = word.hit(r, 0.001, f64::INFINITY) {
        // let target = result.p + result.normal + Vec3::random_unit_vector();
        let target = result.p + result.normal + Vec3::random_in_hemisphere(&result.normal);

        return 0.5 * ray_color_6(&Ray::new(result.p, target - result.p), word, depth - 1);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color3d::only(1.0) + t * Color3d::new(0.5, 0.7, 1.0)
}

fn ray_color_7<H: Hittable>(r: &Ray, word: &H, depth: i32) -> Color3d {
    if depth <= 0 {
        return Color3d::only(0.0);
    }

    if let Some(result) = word.hit(&r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = result.material.scatter(&r, &result) {
            return attenuation * ray_color_7(&scattered, word, depth - 1);
        } else {
            return Color3d::only(0.0);
        }
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t) * Color3d::only(1.0) + t * Color3d::new(0.5, 0.7, 1.0)
}

fn main() {
    scene14();
}

// fn scene1() -> io::Result<()> {
//     const IMAGE_WIDTH: i32 = 256;
//     const IMAGE_HEIGHT: i32 = 256;

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/scene1.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let pixel_color = vec3::Color3d::new(
//                 i as f64 / (IMAGE_WIDTH - 1) as f64,
//                 j as f64 / (IMAGE_HEIGHT - 1) as f64,
//                 0.25,
//             );

//             // write_color(&mut fp, pixel_color);
//         }
//     }
//     Ok(())
// }

// fn scene2() -> io::Result<()> {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const IMAGE_WIDTH: i32 = 400;
//     const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

//     const VIEWPORT_HEIGHT: f64 = 2.0;
//     const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
//     const FOCAL_LENGTH: f64 = 1.0;

//     let origin = Point3d::zero();
//     let horizontal = Vec3d::new(VIEWPORT_WIDTH, 0.0, 0.0);
//     let vertical = Vec3d::new(0.0, VIEWPORT_HEIGHT, 0.0);
//     let lower_left_corner =
//         origin - horizontal / 2.0 - vertical / 2.0 - Vec3d::new(0.0, 0.0, FOCAL_LENGTH);

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/blue-to-white.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
//             let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

//             let r = Ray::new(
//                 origin,
//                 lower_left_corner + u * horizontal + v * vertical - origin,
//             );
//             let pixel_color = ray_color(&r);
//             // write_color(&mut fp, pixel_color);
//         }
//     }
//     Ok(())
// }

// fn scene3() -> io::Result<()> {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const IMAGE_WIDTH: i32 = 400;
//     const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

//     const VIEWPORT_HEIGHT: f64 = 2.0;
//     const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
//     const FOCAL_LENGTH: f64 = 1.0;

//     let origin = Point3d::zero();
//     let horizontal = Vec3d::new(VIEWPORT_WIDTH, 0.0, 0.0);
//     let vertical = Vec3d::new(0.0, VIEWPORT_HEIGHT, 0.0);
//     let lower_left_corner =
//         origin - horizontal / 2.0 - vertical / 2.0 - Vec3d::new(0.0, 0.0, FOCAL_LENGTH);

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/circle.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
//             let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

//             let r = Ray::new(
//                 origin,
//                 lower_left_corner + u * horizontal + v * vertical - origin,
//             );
//             let pixel_color = ray_color_sphere(&r);
//             // write_color(&mut fp, pixel_color);
//         }
//     }
//     Ok(())
// }

// fn scene4() -> io::Result<()> {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const IMAGE_WIDTH: i32 = 400;
//     const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

//     const VIEWPORT_HEIGHT: f64 = 2.0;
//     const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
//     const FOCAL_LENGTH: f64 = 1.0;

//     let origin = Point3d::zero();
//     let horizontal = Vec3d::new(VIEWPORT_WIDTH, 0.0, 0.0);
//     let vertical = Vec3d::new(0.0, VIEWPORT_HEIGHT, 0.0);
//     let lower_left_corner =
//         origin - horizontal / 2.0 - vertical / 2.0 - Vec3d::new(0.0, 0.0, FOCAL_LENGTH);

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/circle_color.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
//             let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

//             let r = Ray::new(
//                 origin,
//                 lower_left_corner + u * horizontal + v * vertical - origin,
//             );
//             let pixel_color = ray_color_sphere_color(&r);
//             // write_color(&mut fp, pixel_color);
//         }
//     }
//     Ok(())
// }

// fn scene5() -> io::Result<()> {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const IMAGE_WIDTH: i32 = 400;
//     const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

//     let mut world = HittableList::new();
//     world.push(Sphere::new(Point3d::new(0.0, 0.0, -1.0), 0.5));
//     world.push(Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0));

//     const VIEWPORT_HEIGHT: f64 = 2.0;
//     const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
//     const FOCAL_LENGTH: f64 = 1.0;

//     let origin = Point3d::zero();
//     let horizontal = Vec3d::new(VIEWPORT_WIDTH, 0.0, 0.0);
//     let vertical = Vec3d::new(0.0, VIEWPORT_HEIGHT, 0.0);
//     let lower_left_corner =
//         origin - horizontal / 2.0 - vertical / 2.0 - Vec3d::new(0.0, 0.0, FOCAL_LENGTH);

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/circle_color_with_hittable.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
//             let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;

//             let r = Ray::new(
//                 origin,
//                 lower_left_corner + u * horizontal + v * vertical - origin,
//             );
//             let pixel_color = ray_color_5(&r, &world);
//             // write_color(&mut fp, pixel_color);
//         }
//     }
//     Ok(())
// }

// fn scene6() -> io::Result<()> {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const IMAGE_WIDTH: i32 = 400;
//     const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
//     const SAMPLES_PER_PIXEL: i32 = 100;

//     let mut world = HittableList::new();
//     world.push(Sphere::new(Point3d::new(0.0, 0.0, -1.0), 0.5));
//     world.push(Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0));

//     let cam = Camera::new();

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/multi_sampled.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let mut color = Color3d::only(0.0);
//             for s in 0..SAMPLES_PER_PIXEL {
//                 let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
//                 let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

//                 let r = cam.get_ray(u, v);
//                 color += ray_color_5(&r, &world);
//             }
//             write_color(&mut fp, color, SAMPLES_PER_PIXEL);
//         }
//     }

//     Ok(())
// }

// fn scene7() -> io::Result<()> {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const IMAGE_WIDTH: i32 = 400;
//     const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
//     const SAMPLES_PER_PIXEL: i32 = 100;
//     const MAX_DEPTH: i32 = 50;

//     let mut world = HittableList::new();
//     world.push(Sphere::new(Point3d::new(0.0, 0.0, -1.0), 0.5));
//     world.push(Sphere::new(Point3d::new(0.0, -100.5, -1.0), 100.0));

//     let cam = Camera::new();

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/diffuse_sphere.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let mut color = Color3d::only(0.0);
//             for _ in 0..SAMPLES_PER_PIXEL {
//                 let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
//                 let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

//                 let r = cam.get_ray(u, v);
//                 color += ray_color_6(&r, &world, MAX_DEPTH);
//             }
//             write_color(&mut fp, color, SAMPLES_PER_PIXEL);
//         }
//     }

//     Ok(())
// }

// fn scene8() -> io::Result<()> {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const IMAGE_WIDTH: i32 = 400;
//     const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
//     const SAMPLES_PER_PIXEL: i32 = 100;
//     const MAX_DEPTH: i32 = 50;

//     let mut world = HittableList::new();
//     let material_ground = Diffuse::new(Color3d::new(0.8, 0.8, 0.0));
//     let material_center = Diffuse::new(Color3d::new(0.7, 0.3, 0.3));
//     let material_left = Metal::new(Color3d::new(0.8, 0.8, 0.8), 0.3);

//     let material_right = Metal::new(Color3d::new(0.8, 0.6, 0.2), 1.0);

//     world.push(Box::new(Sphere::new(
//         Point3d::new(0.0, -100.5, -1.0),
//         100.0,
//         material_ground,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(0.0, 0.0, -1.0),
//         0.5,
//         material_center,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(-1.0, 0.0, -1.0),
//         0.5,
//         material_left,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(1.0, 0.0, -1.0),
//         0.5,
//         material_right,
//     )));

//     let cam = Camera::new();

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/metal_spheres.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let mut color = Color3d::only(0.0);
//             for _ in 0..SAMPLES_PER_PIXEL {
//                 let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
//                 let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

//                 let r = cam.get_ray(u, v);
//                 color += ray_color_7(&r, &world, MAX_DEPTH);
//             }
//             write_color(&mut fp, color, SAMPLES_PER_PIXEL);
//         }
//     }

//     Ok(())
// }

// fn scene9() -> io::Result<()> {
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

//     let mut world = HittableList::new();
//     let material_ground = Diffuse::new(Color3d::new(0.8, 0.8, 0.0));
//     let material_center = Dieletric::new(1.5);
//     let material_left = Dieletric::new(1.5);

//     let material_right = Metal::new(Color3d::new(0.8, 0.6, 0.2), 1.0);

//     world.push(Box::new(Sphere::new(
//         Point3d::new(0.0, -100.5, -1.0),
//         100.0,
//         material_ground,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(0.0, 0.0, -1.0),
//         0.5,
//         material_center,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(-1.0, 0.0, -1.0),
//         0.5,
//         material_left,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(1.0, 0.0, -1.0),
//         0.5,
//         material_right,
//     )));

//     let cam = Camera::new();

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/galss_spheres.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let mut color = Color3d::only(0.0);
//             for _ in 0..SAMPLES_PER_PIXEL {
//                 let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
//                 let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

//                 let r = cam.get_ray(u, v);
//                 color += ray_color_7(&r, &world, MAX_DEPTH);
//             }
//             write_color(&mut fp, color, SAMPLES_PER_PIXEL);
//         }
//     }

//     Ok(())
// }

// fn scene10() -> io::Result<()> {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const IMAGE_WIDTH: i32 = 400;
//     const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
//     const SAMPLES_PER_PIXEL: i32 = 100;
//     const MAX_DEPTH: i32 = 50;

//     let mut world = HittableList::new();
//     let material_ground = Diffuse::new(Color3d::new(0.8, 0.8, 0.0));
//     let material_center = Diffuse::new(Color3d::new(0.1, 0.2, 0.5));
//     let material_left = Dieletric::new(1.5);

//     let material_right = Metal::new(Color3d::new(0.8, 0.6, 0.2), 0.0);

//     world.push(Box::new(Sphere::new(
//         Point3d::new(0.0, -100.5, -1.0),
//         100.0,
//         material_ground,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(0.0, 0.0, -1.0),
//         0.5,
//         material_center,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(-1.0, 0.0, -1.0),
//         0.5,
//         material_left,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(-1.0, 0.0, -1.0),
//         -0.4,
//         Dieletric::new(1.5),
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(1.0, 0.0, -1.0),
//         0.5,
//         material_right,
//     )));

//     let cam = Camera::new();

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/dielectric_and_shiny_sphere.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let mut color = Color3d::only(0.0);
//             for _ in 0..SAMPLES_PER_PIXEL {
//                 let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
//                 let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

//                 let r = cam.get_ray(u, v);
//                 color += ray_color_7(&r, &world, MAX_DEPTH);
//             }
//             write_color(&mut fp, color, SAMPLES_PER_PIXEL);
//         }
//     }

//     Ok(())
// }

// fn scene11() -> io::Result<()> {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const IMAGE_WIDTH: i32 = 400;
//     const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
//     const SAMPLES_PER_PIXEL: i32 = 100;
//     const MAX_DEPTH: i32 = 50;

//     let mut world = HittableList::new();
//     let material_left = Diffuse::new(Point3d::new(0.0, 0.0, 1.0));
//     let material_right = Diffuse::new(Point3d::new(1.0, 0.0, 0.0));

//     let r = (PI / 4.0).cos();

//     world.push(Box::new(Sphere::new(
//         Point3d::new(-r, 0.0, -1.0),
//         r,
//         material_left,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(r, 0.0, -1.0),
//         r,
//         material_right,
//     )));

//     let cam = Camera::new(90.0, ASPECT_RATIO);

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/wide_angle_view.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let mut color = Color3d::only(0.0);
//             for _ in 0..SAMPLES_PER_PIXEL {
//                 let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
//                 let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

//                 let r = cam.get_ray(u, v);
//                 color += ray_color_7(&r, &world, MAX_DEPTH);
//             }
//             write_color(&mut fp, color, SAMPLES_PER_PIXEL);
//         }
//     }

//     Ok(())
// }

// fn scene12() -> io::Result<()> {
//     const ASPECT_RATIO: f64 = 16.0 / 9.0;
//     const IMAGE_WIDTH: i32 = 400;
//     const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
//     const SAMPLES_PER_PIXEL: i32 = 100;
//     const MAX_DEPTH: i32 = 50;

//     let mut world = HittableList::new();
//     let material_ground = Diffuse::new(Point3d::new(0.8, 0.8, 0.0));
//     let material_center = Diffuse::new(Point3d::new(0.1, 0.2, 0.5));
//     let material_left = Dieletric::new(1.5);
//     let material_right = Metal::new(Color3d::new(0.6, 0.8, 0.2), 0.0);

//     let r = (PI / 4.0).cos();

//     world.push(Box::new(Sphere::new(
//         Point3d::new(0.0, -100.5, -1.0),
//         100.0,
//         material_ground,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(0.0, 0.0, -1.0),
//         0.5,
//         material_center,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(-1.0, 0.0, -1.0),
//         0.5,
//         material_left,
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(-1.0, 0.0, -1.0),
//         -0.45,
//         Dieletric::new(1.5),
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3d::new(1.0, 0.0, -1.0),
//         0.5,
//         material_right,
//     )));

//     let cam = Camera::new(
//         Point3d::new(-2.0, 2.0, 1.0),
//         Point3d::new(0.0, 0.0, -1.0),
//         Vec3d::new(0.0, 1.0, 0.0),
//         90.0,
//         ASPECT_RATIO,
//     );

//     let mut fp = OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open("./result/zoom_view.ppm")
//         .expect("cannot open file");

//     fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

//     for j in (0..IMAGE_HEIGHT).rev() {
//         for i in 0..IMAGE_WIDTH {
//             let mut color = Color3d::only(0.0);
//             for _ in 0..SAMPLES_PER_PIXEL {
//                 let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
//                 let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

//                 let r = cam.get_ray(u, v);
//                 color += ray_color_7(&r, &world, MAX_DEPTH);
//             }
//             write_color(&mut fp, color, SAMPLES_PER_PIXEL);
//         }
//     }

//     Ok(())
// }

fn scene13() -> io::Result<()> {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    let mut world = HittableList::new();
    let material_ground = Diffuse::new(Point3d::new(0.8, 0.8, 0.0));
    let material_center = Diffuse::new(Point3d::new(0.1, 0.2, 0.5));
    let material_left = Dieletric::new(1.5);
    let material_right = Metal::new(Color3d::new(0.8, 0.6, 0.2), 0.0);

    world.push(Box::new(Sphere::new(
        Point3d::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.push(Box::new(Sphere::new(
        Point3d::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.push(Box::new(Sphere::new(
        Point3d::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(material_left),
    )));
    world.push(Box::new(Sphere::new(
        Point3d::new(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dieletric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Point3d::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let lookfrom = Point3d::new(3.0, 3.0, 2.0);
    let lookat = Point3d::new(0.0, 0.0, -1.0);
    let vup = Vec3d::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let mut fp = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./result/Spheres_with_depth_of_field.ppm")
        .expect("cannot open file");

    fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut color = Color3d::only(0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

                let r = cam.get_ray(u, v);
                color += ray_color_7(&r, &world, MAX_DEPTH);
            }
            write_color(&mut fp, color, SAMPLES_PER_PIXEL);
        }
    }

    Ok(())
}

fn scene14() -> io::Result<()> {
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    let world = HittableList::random_scene();

    let lookfrom = Point3d::new(13.0, 2.0, 3.0);
    let lookat = Point3d::new(0.0, 0.0, 0.0);
    let vup = Vec3d::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let mut fp = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./result/final_scene.ppm")
        .expect("cannot open file");

    fp.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes());

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut color = Color3d::only(0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64;

                let r = cam.get_ray(u, v);
                color += ray_color_7(&r, &world, MAX_DEPTH);
            }
            write_color(&mut fp, color, SAMPLES_PER_PIXEL);
        }
    }

    Ok(())
}
