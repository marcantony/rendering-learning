use std::io::{Result, Write};

use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_ppm(&self, writer: &mut impl Write) -> Result<()> {
        fn to_ppm(val: f64) -> u8 {
            let n = (val * 255.999).round() as i16;
            n.clamp(0, 255) as u8
        }

        let ir = to_ppm(self.r());
        let ig = to_ppm(self.g());
        let ib = to_ppm(self.b());

        writeln!(writer, "{} {} {}", ir, ig, ib)?;

        Ok(())
    }
}
