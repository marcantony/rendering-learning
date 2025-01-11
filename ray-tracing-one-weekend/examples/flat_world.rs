use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    color::Color,
    hittable::{flat::plane::Plane, quad::Quad, Hittable},
    material::{Lambertian, Material},
    texture::{Checker, SolidColor},
    vec3::{Point3, Vec3},
};

mod common;

fn main() {
    // Materials
    let left_red = Lambertian {
        texture: SolidColor {
            albedo: Color::new(1.0, 0.2, 0.2),
        },
    };
    let back_green = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.2, 1.0, 0.2),
        },
    };
    let right_blue = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.2, 0.2, 1.0),
        },
    };
    let upper_orange = Lambertian {
        texture: SolidColor {
            albedo: Color::new(1.0, 0.5, 0.0),
        },
    };
    let lower_checker = Lambertian {
        texture: Checker::new(
            1.0,
            SolidColor {
                albedo: Color::new(0.2, 0.8, 0.8),
            },
            SolidColor {
                albedo: Color::new(0.8, 0.8, 0.8),
            },
        ),
    };

    // Quads
    let mut world: Vec<Box<dyn Hittable<Material = &dyn Material>>> = Vec::new();

    world.push(Box::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        &left_red as &dyn Material,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        &back_green as &dyn Material,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        &right_blue as &dyn Material,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        &upper_orange as &dyn Material,
    )));
    world.push(Box::new(Plane::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        &lower_checker as &dyn Material,
    )));

    let camera = Camera::new(CameraParams {
        aspect_ratio: 1.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 80.0,
        lookfrom: Point3::new(0.0, 0.0, 9.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Point3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    });

    common::render_to_stdout(&world.as_slice(), &camera);
}
