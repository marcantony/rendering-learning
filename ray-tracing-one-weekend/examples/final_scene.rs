use std::io::Cursor;

use common::make_box;
use image::ImageReader;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use ray_tracing_one_weekend::{
    bvh::Bvh,
    camera::{Camera, CameraParams},
    color::{srgb, Color},
    hittable::{
        constant_medium::ConstantMedium,
        quad::Quad,
        sphere::{Center, Sphere},
        Hittable,
    },
    material::{Dielectric, DiffuseLight, Isotropic, Lambertian, Material, Metal},
    perlin::Perlin,
    texture::{Image, Noise, SolidColor},
    vec3::{Point3, Vec3},
};

mod common;

// "Dev" parameters
const IMAGE_WIDTH: usize = 400;
const SAMPLES_PER_PIXEL: usize = 250;
const MAX_DEPTH: usize = 4;

// "High quality" parameters
// const IMAGE_WIDTH: usize = 800;
// const SAMPLES_PER_PIXEL: usize = 10000;
// const MAX_DEPTH: usize = 40;

fn main() {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);

    // Define all materials up front so that they live longer than the world
    let ground_material = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.48, 0.83, 0.53),
        },
    };

    let light_material = DiffuseLight {
        texture: SolidColor {
            albedo: Color::new(7.0, 7.0, 7.0),
        },
    };

    let sphere_material = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.7, 0.3, 0.1),
        },
    };

    let glass_material = Dielectric {
        refraction_index: 1.5,
    };

    let metal_material = Metal {
        albedo: Color::new(0.8, 0.8, 0.9),
        fuzz: 1.0,
    };

    let subsurface_material = Isotropic {
        texture: SolidColor {
            albedo: Color::new(0.2, 0.4, 0.9),
        },
    };

    let fog_material = Isotropic {
        texture: SolidColor {
            albedo: Color::new(1.0, 1.0, 1.0),
        },
    };

    let earth_image_bytes = include_bytes!("files/earthmap.jpg");
    let mut earth_image = ImageReader::new(Cursor::new(earth_image_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb32f();
    earth_image.pixels_mut().for_each(|p| {
        p.0 = p.0.map(|u| srgb::srgb_to_linear(u as f64) as f32);
    });
    let earth_material = Lambertian {
        texture: Image { image: earth_image },
    };

    let perlin_material = Lambertian {
        texture: Noise {
            noise: Perlin::new(&mut rng),
            scale: 0.2,
        },
    };

    let white_material = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.73, 0.73, 0.73),
        },
    };

    // Start defining world
    let mut world: Vec<Box<dyn Hittable<Material = &dyn Material>>> = Vec::new();

    // Floor boxes
    let mut boxes = Vec::new();
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let i_f = i as f64;
            let j_f = j as f64;

            let w = 100.0;
            let x0 = -1000.0 + i_f * w;
            let z0 = -1000.0 + j_f * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            boxes.push(make_box(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                &ground_material as &dyn Material,
            ));
        }
    }
    world.push(Box::new(Bvh::new(boxes)));

    // Light
    let light = Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        &light_material as &dyn Material,
    );
    world.push(Box::new(light));

    // Moving sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = &center1 + Vec3::new(30.0, 0.0, 0.0);
    world.push(Box::new(Sphere {
        center: Center::Moving(center1, center2),
        radius: 50.0,
        material: &sphere_material as &dyn Material,
    }));

    // Glass sphere
    world.push(Box::new(Sphere {
        center: Center::Stationary(Point3::new(260.0, 150.0, 45.0)),
        radius: 50.0,
        material: &glass_material as &dyn Material,
    }));

    // Metal sphere
    world.push(Box::new(Sphere {
        center: Center::Stationary(Point3::new(0.0, 150.0, 145.0)),
        radius: 50.0,
        material: &metal_material as &dyn Material,
    }));

    // Subsurface reflection sphere
    let subsurface_boundary = Sphere {
        center: Center::Stationary(Point3::new(360.0, 150.0, 145.0)),
        radius: 70.0,
        material: &glass_material as &dyn Material,
    };
    world.push(Box::new(subsurface_boundary.clone()));
    world.push(Box::new(ConstantMedium::new(
        subsurface_boundary,
        0.2,
        &subsurface_material as &dyn Material,
    )));
    // Fog
    let fog_boundary = Sphere {
        center: Center::Stationary(Point3::new(0.0, 0.0, 0.0)),
        radius: 5000.0,
        material: &glass_material as &dyn Material,
    };
    world.push(Box::new(ConstantMedium::new(
        fog_boundary,
        0.0001,
        &fog_material as &dyn Material,
    )));

    // Earth texture sphere
    world.push(Box::new(Sphere {
        center: Center::Stationary(Point3::new(400.0, 200.0, 400.0)),
        radius: 100.0,
        material: &earth_material as &dyn Material,
    }));

    // Perlin noise sphere
    world.push(Box::new(Sphere {
        center: Center::Stationary(Point3::new(220.0, 280.0, 300.0)),
        radius: 80.0,
        material: &perlin_material as &dyn Material,
    }));

    // Box of small spheres
    let mut boxes2 = Vec::new();
    let ns = 1000;
    for _ in 0..ns {
        boxes2.push(Sphere {
            center: Center::Stationary(Point3::random_in_range(&mut rng, 0.0, 165.0)),
            radius: 10.0,
            material: &white_material as &dyn Material,
        });
    }
    world.push(Box::new(
        Bvh::new(boxes2)
            .rotate_y(15.0)
            .translate(Vec3::new(-100.0, 270.0, 395.0)),
    ));

    let camera = Camera::new(CameraParams {
        aspect_ratio: 1.0,
        image_width: IMAGE_WIDTH,
        samples_per_pixel: SAMPLES_PER_PIXEL,
        max_depth: MAX_DEPTH,
        background: Color::new(0.0, 0.0, 0.0),
        vfov: 40.0,
        lookfrom: Point3::new(478.0, 278.0, -600.0),
        lookat: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 0.0,
        ..Default::default()
    });

    common::render_to_stdout(&world.as_slice(), &camera);
    // common::render_save_checkpoint(&world.as_slice(), &camera, "output/final-scene");
    // common::render_from_checkpoint(
    //     &world.as_slice(),
    //     &camera,
    //     "output/final-scene-1736561842.chkpt",
    // );
}
