use std::io::{self, BufWriter, Result};

use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    color::Color,
    material::{Dielectric, Lambertian, Material, Metal},
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

fn main() -> Result<()> {
    let mut master_rng = Xoshiro256PlusPlus::seed_from_u64(1);

    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let material_center = Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let material_left = Dielectric {
        refraction_index: 1.5,
    };
    let material_bubble = Dielectric {
        refraction_index: 1.0 / 1.5,
    };
    let material_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    // World
    let world: [Sphere<&dyn Material<Xoshiro256PlusPlus>, Xoshiro256PlusPlus>; 5] = [
        Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: &material_ground,
            phantom: Default::default(),
        },
        Sphere {
            center: Point3::new(0.0, 0.0, -1.2),
            radius: 0.5,
            material: &material_center,
            phantom: Default::default(),
        },
        Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: &material_left,
            phantom: Default::default(),
        },
        Sphere {
            center: Point3::new(-1.0, 0.0, -1.0),
            radius: 0.4,
            material: &material_bubble,
            phantom: Default::default(),
        },
        Sphere {
            center: Point3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: &material_right,
            phantom: Default::default(),
        },
    ];

    let camera = Camera::new(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20.0,
        lookfrom: Point3::new(-2.0, 2.0, 1.0),
        lookat: Point3::new(0.0, 0.0, -1.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 10.0,
        focus_dist: 3.4,
    });

    let mut out = BufWriter::new(io::stdout().lock());

    camera.render(&mut master_rng, &world.as_slice(), &mut out)
}
