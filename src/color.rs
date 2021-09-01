use crate::vec3::Color3d;
use std::io;

pub fn write_color(fp: &mut impl io::Write, color: Color3d) -> io::Result<()> {
    fp.write(
        format!(
            "{} {} {}\n",
            (255.999 * color.x) as u8,
            (255.999 * color.y) as u8,
            (255.999 * color.z) as u8,
        )
        .as_bytes(),
    )?;

    Ok(())
}
