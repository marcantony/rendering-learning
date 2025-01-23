use std::io::BufReader;

use ray_tracing_one_weekend::{
    bvh::Bvh,
    camera::{Camera, CameraParams},
    color::Color,
    hittable::{
        sphere::{Center, Sphere},
        Hittable,
    },
    io::wavefront_obj::WavefrontObj,
    material::{DiffuseLight, Lambertian, Material},
    texture::SolidColor,
    vec3::{Point3, Vec3},
};

mod common;

fn main() {
    let diffuse = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.53, 0.32, 0.75),
        },
    };
    let light = DiffuseLight {
        texture: SolidColor {
            albedo: Color::new(5.0, 5.0, 5.0),
        },
    };

    let obj_bytes = include_bytes!("../../objs/teapot-low.obj");
    let reader = BufReader::new(obj_bytes.as_slice());
    let teapot = WavefrontObj::parse(reader).to_object(&diffuse as &(dyn Material + Sync));
    let transformed_teapot = teapot.scale(2.0).rotate_x(-90.0);

    let mut world: Vec<Box<dyn Hittable<Material = &(dyn Material + Sync)> + Sync>> = Vec::new();

    world.push(Box::new(Sphere {
        center: Center::Stationary(Point3::new(40.0, 20.0, -40.0)),
        radius: 10.0,
        material: &light as &(dyn Material + Sync),
    }));
    world.push(Box::new(transformed_teapot));

    let world = Bvh::new(world);

    let camera = Camera::new(CameraParams {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 200,
        max_depth: 40,
        vfov: 40.0,
        lookfrom: Point3::new(0.0, 0.0, -100.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        background: Color::new(0.1, 0.1, 0.1),
        ..Default::default()
    });

    common::render_to_stdout(&world, &camera);
}
