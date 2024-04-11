use std::io::{self, BufWriter, Result};

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    color::Color,
    material::{Lambertian, Material, Metal},
    sphere::Sphere,
    vec3::Point3,
};

fn main() -> Result<()> {
    let mut master_rng = Xoshiro256PlusPlus::seed_from_u64(1);

    let mut material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
        rng: Xoshiro256PlusPlus::from_rng(&mut master_rng)?,
    };
    let mut material_center = Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
        rng: Xoshiro256PlusPlus::from_rng(&mut master_rng)?,
    };
    let mut material_left = Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
    };
    let mut material_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
    };

    // World
    let mut world: [Sphere<&mut dyn Material>; 4] = [
        Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: &mut material_ground,
        },
        Sphere {
            center: Point3::new(0.0, 0.0, -1.2),
            radius: 0.5,
            material: &mut material_center,
        },
        Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: &mut material_left,
        },
        Sphere {
            center: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: &mut material_right,
        },
    ];

    let mut camera = Camera::new(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        rng: Xoshiro256PlusPlus::from_rng(&mut master_rng)?,
        max_depth: 50,
    });

    let mut out = BufWriter::new(io::stdout().lock());

    camera.render(&mut world.as_mut_slice(), &mut out)
}
