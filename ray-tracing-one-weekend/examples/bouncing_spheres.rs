use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use ray_tracing_one_weekend::{
    bvh::Bvh,
    camera::{Camera, CameraParams},
    color::Color,
    hittable::sphere::{Center, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
    texture::{Checker, SolidColor},
    vec3::{Point3, Vec3},
};

mod common;

fn main() {
    let mut master_rng = Xoshiro256PlusPlus::seed_from_u64(1);

    let mut world = Vec::<Sphere<Box<dyn Material>>>::new();

    let checker = Checker::new(
        0.32,
        SolidColor {
            albedo: Color::new(0.2, 0.23, 0.1),
        },
        SolidColor {
            albedo: Color::new(0.9, 0.9, 0.9),
        },
    );
    let ground_material = Lambertian { texture: checker };
    world.push(Sphere {
        center: Center::Stationary(Point3::new(0.0, -1000.0, 0.0)),
        radius: 1000.0,
        material: Box::new(ground_material),
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = master_rng.gen();
            let center_point = Point3::new(
                a as f64 + 0.9 * master_rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * master_rng.gen::<f64>(),
            );

            if (&center_point - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let (sphere_material, center): (Box<dyn Material>, Center) = if choose_mat < 0.8 {
                    // diffuse
                    let center2 =
                        &center_point + Vec3::new(0.0, master_rng.gen_range(0.0..0.5), 0.0);
                    (
                        Box::new(Lambertian {
                            texture: SolidColor {
                                albedo: Color::random(&mut master_rng)
                                    * Color::random(&mut master_rng),
                            },
                        }),
                        Center::Moving(center_point, center2),
                    )
                } else if choose_mat < 0.95 {
                    // metal
                    (
                        Box::new(Metal {
                            albedo: Color::random_in_range(&mut master_rng, 0.5, 1.0),
                            fuzz: master_rng.gen(),
                        }),
                        Center::Stationary(center_point),
                    )
                } else {
                    // glass
                    (
                        Box::new(Dielectric {
                            refraction_index: 1.5,
                        }),
                        Center::Stationary(center_point),
                    )
                };
                world.push(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                })
            }
        }
    }

    let material1 = Dielectric {
        refraction_index: 1.5,
    };
    world.push(Sphere {
        center: Center::Stationary(Point3::new(0.0, 1.0, 0.0)),
        radius: 1.0,
        material: Box::new(material1),
    });

    let material2 = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.4, 0.2, 0.1),
        },
    };
    world.push(Sphere {
        center: Center::Stationary(Point3::new(-4.0, 1.0, 0.0)),
        radius: 1.0,
        material: Box::new(material2),
    });

    let material3 = Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    world.push(Sphere {
        center: Center::Stationary(Point3::new(4.0, 1.0, 0.0)),
        radius: 1.0,
        material: Box::new(material3),
    });

    let world = Bvh::new(world);

    let camera = Camera::new(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 10,
        vfov: 20.0,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.6,
        focus_dist: 10.0,
        ..Default::default()
    });

    common::render_to_stdout(&world, &camera);
}
