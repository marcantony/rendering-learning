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
        let in_srgb = self.linear_to_srgb();
        let (ir, ig, ib) = in_srgb.to_u8();

        writeln!(writer, "{} {} {}", ir, ig, ib)?;

        Ok(())
    }

    pub fn linear_to_srgb(&self) -> Self {
        Color::new(
            srgb::linear_to_srgb(self.r()),
            srgb::linear_to_srgb(self.g()),
            srgb::linear_to_srgb(self.b()),
        )
    }

    pub fn srgb_to_linear(&self) -> Self {
        Color::new(
            srgb::srgb_to_linear(self.r()),
            srgb::srgb_to_linear(self.g()),
            srgb::srgb_to_linear(self.b()),
        )
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        fn channel_to_u8(val: f64) -> u8 {
            let n = (val * 255.999).floor() as i16;
            n.clamp(0, 255) as u8
        }
        (
            channel_to_u8(self.r()),
            channel_to_u8(self.g()),
            channel_to_u8(self.b()),
        )
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

        assert_eq!(output, Ok("0 188 255\n".to_string()));
    }

    #[test]
    fn color_to_ppm_clamps_output() {
        let color = Color::new(-1.0, 0.5, 2.0);
        let mut buf = Vec::new();
        color
            .write_ppm(&mut buf)
            .expect("failed to write color data to buffer");

        let output = String::from_utf8(buf);

        assert_eq!(output, Ok("0 188 255\n".to_string()));
    }
}

/// Provides functions for encoding and decoding sRGB values
/// https://en.wikipedia.org/wiki/SRGB#Transfer_function_(%22gamma%22)
pub mod srgb {

    const U: f64 = 0.04045;
    const V: f64 = 0.0031308;
    const A: f64 = 12.92;
    const C: f64 = 0.055;
    const GAMMA: f64 = 2.4;

    pub fn srgb_to_linear(u: f64) -> f64 {
        if u <= U {
            u / A
        } else {
            ((u + C) / (1.0 + C)).powf(GAMMA)
        }
    }

    pub fn linear_to_srgb(v: f64) -> f64 {
        if v <= V {
            A * v
        } else {
            (1.0 + C) * v.powf(1.0 / GAMMA) - C
        }
    }
}
