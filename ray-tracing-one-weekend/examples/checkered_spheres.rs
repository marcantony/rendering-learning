use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    color::Color,
    material::Lambertian,
    sphere::{Center, Sphere},
    texture::{Checker, SolidColor},
    vec3::{Point3, Vec3},
};

mod common;

fn main() {
    let mut world = Vec::new();

    let checker = Checker::new(
        0.32,
        SolidColor {
            albedo: Color::new(0.2, 0.3, 0.1),
        },
        SolidColor {
            albedo: Color::new(0.9, 0.9, 0.9),
        },
    );
    let lambertian = Lambertian { texture: checker };

    world.push(Sphere {
        center: Center::Stationary(Point3::new(0.0, -10.0, 0.0)),
        radius: 10.0,
        material: &lambertian,
    });

    world.push(Sphere {
        center: Center::Stationary(Point3::new(0.0, 10.0, 0.0)),
        radius: 10.0,
        material: &lambertian,
    });

    let camera = Camera::new(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    });

    common::render_to_stdout(&world.as_slice(), &camera);
}
