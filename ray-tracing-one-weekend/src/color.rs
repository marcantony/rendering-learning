use std::io::{Result, Write};

use rand::Rng;

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

        fn linear_to_gamma(linear_component: f64) -> f64 {
            if linear_component > 0.0 {
                linear_component.sqrt()
            } else {
                0.0
            }
        }

        let ir = to_ppm(linear_to_gamma(self.r()));
        let ig = to_ppm(linear_to_gamma(self.g()));
        let ib = to_ppm(linear_to_gamma(self.b()));

        writeln!(writer, "{} {} {}", ir, ig, ib)?;

        Ok(())
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Color::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_in_range<R: Rng + ?Sized>(rng: &mut R, min: f64, max: f64) -> Self {
        Color::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
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

        assert_eq!(output, Ok("0 181 255\n".to_string()));
    }

    #[test]
    fn color_to_ppm_clamps_output() {
        let color = Color::new(-1.0, 0.5, 2.0);
        let mut buf = Vec::new();
        color
            .write_ppm(&mut buf)
            .expect("failed to write color data to buffer");

        let output = String::from_utf8(buf);

        assert_eq!(output, Ok("0 181 255\n".to_string()));
    }
}
