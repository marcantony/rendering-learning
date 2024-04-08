use std::io::{self, BufWriter, Result};

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    sphere::Sphere,
    vec3::Point3,
};

fn main() -> Result<()> {
    // World
    let world = [
        Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        },
        Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        },
    ];

    let mut camera = Camera::new(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        rng: Xoshiro256PlusPlus::seed_from_u64(1),
    });

    let mut out = BufWriter::new(io::stdout().lock());

    camera.render(&world.as_slice(), &mut out)
}
