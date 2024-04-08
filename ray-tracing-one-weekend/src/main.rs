use std::io::{self, BufWriter, Result};

use ray_tracing_one_weekend::{camera::Camera, sphere::Sphere, vec3::Point3};

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

    let camera = Camera {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
    };

    let mut out = BufWriter::new(io::stdout().lock());

    camera.render(&world.as_slice(), &mut out)
}
