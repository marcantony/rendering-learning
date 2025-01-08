use ray_tracing_one_weekend::{
    bvh::Bvh,
    camera::{Camera, CameraParams},
    color::Color,
    hittable::quad::Quad,
    hittable::Hittable,
    material::{DiffuseLight, Lambertian, Material},
    texture::SolidColor,
    vec3::{Point3, Vec3},
};

mod common;

fn main() {
    let red = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.65, 0.05, 0.05),
        },
    };
    let white = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.73, 0.73, 0.73),
        },
    };
    let green = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.12, 0.45, 0.15),
        },
    };
    let light = DiffuseLight {
        texture: SolidColor {
            albedo: Color::new(15.0, 15.0, 15.0),
        },
    };

    let mut world: Vec<Box<dyn Hittable<Material = &dyn Material>>> = Vec::new();

    world.push(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        &green as &dyn Material,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        &red as &dyn Material,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        &light as &dyn Material,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        &white as &dyn Material,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        &white as &dyn Material,
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        &white as &dyn Material,
    )));

    world.push(Box::new(
        common::make_box(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(165.0, 330.0, 165.0),
            &white as &dyn Material,
        )
        .rotate_y(15.0)
        .translate(Vec3::new(265.0, 0.0, 295.0)),
    ));
    world.push(Box::new(
        common::make_box(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(165.0, 165.0, 165.0),
            &white as &dyn Material,
        )
        .rotate_y(-18.0)
        .translate(Vec3::new(130.0, 0.0, 65.0)),
    ));

    let world = Bvh::new(world);

    let camera = Camera::new(CameraParams {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 200,
        max_depth: 50,
        background: Color::new(0.0, 0.0, 0.0),
        vfov: 40.0,
        lookfrom: Point3::new(278.0, 278.0, -800.0),
        lookat: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    });

    common::render_to_stdout(&world, &camera);
}
