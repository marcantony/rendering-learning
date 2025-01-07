// use ray_tracing_one_weekend::material::Lambertian;

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
        make_box(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(165.0, 330.0, 165.0),
            &white as &dyn Material,
        )
        .rotate_y(15.0)
        .translate(Point3::new(265.0, 0.0, 295.0)),
    ));
    world.push(Box::new(
        make_box(
            &Point3::new(0.0, 0.0, 0.0),
            &Point3::new(165.0, 165.0, 165.0),
            &white as &dyn Material,
        )
        .rotate_y(-18.0)
        .translate(Point3::new(130.0, 0.0, 65.0)),
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

/// Returns the 3D box (six sides) that contains the two opposite vertices a & b
fn make_box<'a, M: ?Sized>(a: &Point3, b: &Point3, material: &'a M) -> [Quad<&'a M>; 6] {
    // Construct the opposite vertices with the min and max coordinates
    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    [
        Quad::new(
            // front
            Point3::new(min.x(), min.y(), max.z()),
            dx.clone(),
            dy.clone(),
            material,
        ),
        Quad::new(
            // right
            Point3::new(max.x(), min.y(), max.z()),
            -&dz,
            dy.clone(),
            material,
        ),
        Quad::new(
            // back
            Point3::new(max.x(), min.y(), min.z()),
            -&dx,
            dy.clone(),
            material,
        ),
        Quad::new(
            // left
            Point3::new(min.x(), min.y(), min.z()),
            dz.clone(),
            dy.clone(),
            material,
        ),
        Quad::new(
            // top
            Point3::new(min.x(), max.y(), max.z()),
            dx.clone(),
            -&dy,
            material,
        ),
        Quad::new(
            // bottom
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            material,
        ),
    ]
}
