use std::fs::OpenOptions;

mod color;
mod ray;
mod vec3;

use std::io::{self, Write};

use crate::color::write_color;

fn main() -> io::Result<()> {
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
