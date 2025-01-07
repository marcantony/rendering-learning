use std::io::Cursor;

use image::ImageReader;
use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    color::srgb,
    material::Lambertian,
    sphere::{Center, Sphere},
    texture::Image,
    vec3::Point3,
};

mod common;

fn main() {
    let earth_image_bytes = include_bytes!("files/earthmap.jpg");
    let mut earth_image = ImageReader::new(Cursor::new(earth_image_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb32f();

    // image crate isn't color-space aware: https://github.com/image-rs/image/issues/2031#issuecomment-1764804355
    // Therefore, manually convert from sRGB to linear.
    earth_image.pixels_mut().for_each(|p| {
        p.0 = p.0.map(|u| srgb::srgb_to_linear(u as f64) as f32);
    });

    let earth_surface = Lambertian {
        texture: Image { image: earth_image },
    };
    let globe = Sphere {
        center: Center::Stationary(Point3::new(0.0, 0.0, 0.0)),
        radius: 2.0,
        material: earth_surface,
    };

    let camera = Camera::new(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: Point3::new(0.0, 0.0, 12.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Point3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    });

    common::render_to_stdout(&globe, &camera);
}
