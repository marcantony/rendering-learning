use std::io::BufReader;

use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    color::Color,
    io::wavefront_obj::WavefrontObj,
    material::Lambertian,
    texture::SolidColor,
    vec3::{Point3, Vec3},
};

mod common;

fn main() {
    let lambertian = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.8, 0.8, 0.8),
        },
    };

    let obj_bytes = include_bytes!("../../objs/spot_triangulated.obj");
    let reader = BufReader::new(obj_bytes.as_slice());

    let obj = WavefrontObj::parse(reader).to_object(&lambertian);

    let mut world = Vec::new();

    world.push(obj);

    let camera = Camera::new(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: Point3::new(4.5, 3.0, 2.5),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    });

    common::render_to_stdout(&world.as_slice(), &camera);
}
