use crate::vec3::Color;
use std::io;

pub fn write_color(fp: &mut impl io::Write, color: Color) -> io::Result<()> {
    fp.write(&[
        (255.999 * color.x) as u8,
        (255.999 * color.y) as u8,
        (255.999 * color.z) as u8,
    ])?;
    Ok(())
}
