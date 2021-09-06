use crate::{utils::clamp, vec3::Color3d};
use std::io;

// pub fn write_color(fp: &mut impl io::Write, color: Color3d) -> io::Result<()> {
//     fp.write(
//         format!(
//             "{} {} {}\n",
//             (255.999 * color.x) as u8,
//             (255.999 * color.y) as u8,
//             (255.999 * color.z) as u8,
//         )
//         .as_bytes(),
//     )?;

//     Ok(())
// }

pub fn write_color(
    fp: &mut impl io::Write,
    color: Color3d,
    samples_per_pixel: i32,
) -> io::Result<()> {
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    fp.write(
        format!(
            "{} {} {}\n",
            256.0 * clamp(r, 0.0, 0.999),
            256.0 * clamp(g, 0.0, 0.999),
            256.0 * clamp(b, 0.0, 0.999),
        )
        .as_bytes(),
    )?;

    Ok(())
}
