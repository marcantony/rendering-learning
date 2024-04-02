use std::io::{Result, Write};

use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn r(&self) -> f64 {
        self.x()
    }

    pub fn g(&self) -> f64 {
        self.y()
    }

    pub fn b(&self) -> f64 {
        self.z()
    }

    pub fn write_ppm(&self, writer: &mut impl Write) -> Result<()> {
        fn to_ppm(val: f64) -> u8 {
            let n = (val * 255.999).floor() as i16;
            n.clamp(0, 255) as u8
        }

        let ir = to_ppm(self.r());
        let ig = to_ppm(self.g());
        let ib = to_ppm(self.b());

        writeln!(writer, "{} {} {}", ir, ig, ib)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_accessors() {
        let c = Color::new(0.0, 1.0, 2.0);

        assert_eq!(c.r(), 0.0);
        assert_eq!(c.g(), 1.0);
        assert_eq!(c.b(), 2.0);
    }

    #[test]
    fn write_color_to_ppm() {
        let color = Color::new(0.0, 0.5, 1.0);
        let mut buf = Vec::new();
        color
            .write_ppm(&mut buf)
            .expect("failed to write color data to buffer");

        let output = String::from_utf8(buf);

        assert_eq!(output, Ok("0 127 255\n".to_string()));
    }

    #[test]
    fn color_to_ppm_clamps_output() {
        let color = Color::new(-1.0, 0.5, 2.0);
        let mut buf = Vec::new();
        color
            .write_ppm(&mut buf)
            .expect("failed to write color data to buffer");

        let output = String::from_utf8(buf);

        assert_eq!(output, Ok("0 127 255\n".to_string()));
    }
}
