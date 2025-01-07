use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    color::Color,
    hittable::quad::Quad,
    hittable::sphere::{Center, Sphere},
    hittable::Hittable,
    material::{DiffuseLight, Lambertian, Material},
    perlin::Perlin,
    texture::{Noise, SolidColor},
    vec3::{Point3, Vec3},
};

mod common;

fn main() {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(1);

    let perlin_texture = Noise {
        noise: Perlin::new(&mut rng),
        scale: 4.0,
    };

    let material = Lambertian {
        texture: perlin_texture,
    };

    let sphere1 = Box::new(Sphere {
        center: Center::Stationary(Point3::new(0.0, -1000.0, 0.0)),
        radius: 1000.0,
        material: &material as &dyn Material,
    });

    let sphere2 = Box::new(Sphere {
        center: Center::Stationary(Point3::new(0.0, 2.0, 0.0)),
        radius: 2.0,
        material: &material as &dyn Material,
    });

    let light_material = DiffuseLight {
        texture: SolidColor {
            albedo: Color::new(4.0, 4.0, 4.0),
        },
    };
    let quad_light = Box::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        &light_material as &dyn Material,
    ));
    let sphere_light = Box::new(Sphere {
        center: Center::Stationary(Point3::new(0.0, 7.0, 0.0)),
        radius: 2.0,
        material: &light_material as &dyn Material,
    });

    let world: Vec<Box<dyn Hittable<Material = &dyn Material>>> =
        vec![sphere1, sphere2, quad_light, sphere_light];

    let camera = Camera::new(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.0, 0.0, 0.0),
        vfov: 20.0,
        lookfrom: Point3::new(26.0, 3.0, 6.0),
        lookat: Point3::new(0.0, 2.0, 0.0),
        vup: Point3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    });

    common::render_to_stdout(&world.as_slice(), &camera);
}
