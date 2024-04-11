use std::io::{self, BufWriter, Result};

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    color::Color,
    material::{Lambertian, Material},
    sphere::Sphere,
    vec3::Point3,
};

fn main() -> Result<()> {
    let mut master_rng = Xoshiro256PlusPlus::seed_from_u64(1);

    let mut material_left = Lambertian {
        albedo: Color::new(0.0, 0.0, 1.0),
        rng: Xoshiro256PlusPlus::from_rng(&mut master_rng)?,
    };
    let mut material_right = Lambertian {
        albedo: Color::new(1.0, 0.0, 0.0),
        rng: Xoshiro256PlusPlus::from_rng(&mut master_rng)?,
    };

    let radius = f64::cos(std::f64::consts::FRAC_PI_4);

    // World
    let mut world: [Sphere<&mut dyn Material>; 2] = [
        Sphere {
            center: Point3::new(-radius, 0.0, -1.0),
            radius,
            material: &mut material_left,
        },
        Sphere {
            center: Point3::new(radius, 0.0, -1.0),
            radius,
            material: &mut material_right,
        },
    ];

    let mut camera = Camera::new(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        rng: Xoshiro256PlusPlus::from_rng(&mut master_rng)?,
        max_depth: 50,
        vfov: 90.0,
    });

    let mut out = BufWriter::new(io::stdout().lock());

    camera.render(&mut world.as_mut_slice(), &mut out)
}
