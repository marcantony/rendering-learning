use image::Rgb32FImage;

use crate::{color::Color, perlin::Perlin, vec3::Point3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

impl<T: Texture> Texture for &T {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        (*self).value(u, v, p)
    }
}

pub struct SolidColor {
    pub albedo: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo.clone()
    }
}

pub struct Checker<A, B> {
    pub even: A,
    pub odd: B,
    inv_scale: f64,
}

impl<A, B> Checker<A, B> {
    pub fn new(scale: f64, even: A, odd: B) -> Self {
        Checker {
            even,
            odd,
            inv_scale: 1.0 / scale,
        }
    }
}

impl<A: Texture, B: Texture> Texture for Checker<A, B> {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = (p.x() * self.inv_scale).floor() as i64;
        let y_integer = (p.y() * self.inv_scale).floor() as i64;
        let z_integer = (p.z() * self.inv_scale).floor() as i64;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

/// A texture based on an image. The image data should be in linear color space.
pub struct Image {
    pub image: Rgb32FImage,
}

impl Texture for Image {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        assert!(
            self.image.width() > 0 && self.image.height() > 0,
            "Image has no data"
        );

        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip v to image coordinates (top to bottom)

        let i = (u * (self.image.width() - 1) as f64) as u32;
        let j = (v * (self.image.height() - 1) as f64) as u32;

        let pixel = self.image.get_pixel(i, j);
        let r = pixel.0[0] as f64;
        let g = pixel.0[1] as f64;
        let b = pixel.0[2] as f64;

        Color::new(r, g, b)
    }
}

pub struct Noise {
    pub noise: Perlin,
    pub scale: f64,
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1.0 + f64::sin(self.scale * p.z() + 10.0 * self.noise.turb(p, 7)))
    }
}
