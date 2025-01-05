use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    material::Lambertian,
    perlin::Perlin,
    sphere::{Center, Sphere},
    texture::Noise,
    vec3::Point3,
};

mod common;

fn main() {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(1);

    let mut world = Vec::new();

    let perlin_texture = Noise {
        noise: Perlin::new(&mut rng),
    };

    let material = Lambertian {
        tex: perlin_texture,
    };

    world.push(Sphere {
        center: Center::Stationary(Point3::new(0.0, -1000.0, 0.0)),
        radius: 1000.0,
        material: &material,
    });

    world.push(Sphere {
        center: Center::Stationary(Point3::new(0.0, 2.0, 0.0)),
        radius: 2.0,
        material: &material,
    });

    let camera = Camera::new(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Point3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    });

    common::render_to_stdout(&world.as_slice(), &camera);
}
