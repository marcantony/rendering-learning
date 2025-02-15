use std::io::{BufReader, Cursor};

use image::ImageReader;
use ray_tracing_one_weekend::{
    bvh::Bvh,
    camera::{Camera, CameraParams},
    color::{srgb, Color},
    hittable::{flat::quad::Quad, Hittable},
    io::wavefront_obj::WavefrontObj,
    material::{DiffuseLight, Lambertian, Material},
    texture::{Image, SolidColor},
    vec3::{Point3, Vec3},
};

mod common;

fn main() {
    let cow_image_bytes = include_bytes!("../../objs/spot_texture.png");
    let mut cow_image = ImageReader::new(Cursor::new(cow_image_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb32f();

    // image crate isn't color-space aware: https://github.com/image-rs/image/issues/2031#issuecomment-1764804355
    // Therefore, manually convert from sRGB to linear.
    cow_image.pixels_mut().for_each(|p| {
        p.0 = p.0.map(|u| srgb::srgb_to_linear(u as f64) as f32);
    });

    let cow_surface = Lambertian {
        texture: Image { image: cow_image },
    };

    // let cow_mat = Dielectric {
    //     refraction_index: 2.42,
    // };
    // let cow_inside_mat = Isotropic {
    //     texture: SolidColor {
    //         albedo: Color::new(0.95, 0.95, 0.95), // Cow internal color
    //     },
    // };

    let obj_bytes = include_bytes!("../../objs/spot_triangulated.obj");
    let reader = BufReader::new(obj_bytes.as_slice());
    let cow = WavefrontObj::parse(reader)
        .to_mesh()
        .to_hittable(&cow_surface as &(dyn Material + Sync));
    let transformed_cow = cow
        .scale(200.0)
        .rotate_y(45.0)
        .translate(Vec3::new(240.0, 165.0, 240.0));

    // let cow_inside = ConstantMedium::new(
    //     transformed_cow.clone(),
    //     0.008, // Cow internal color density
    //     &cow_inside_mat as &dyn Material,
    // );

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
            albedo: Color::new(5.0, 5.0, 5.0),
        },
    };

    let mut world: Vec<Box<dyn Hittable<Material = &(dyn Material + Sync)> + Sync>> = Vec::new();

    world.push(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        &green as &(dyn Material + Sync),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        &red as &(dyn Material + Sync),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        &light as &(dyn Material + Sync),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        &white as &(dyn Material + Sync),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        &white as &(dyn Material + Sync),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        &white as &(dyn Material + Sync),
    )));

    world.push(Box::new(transformed_cow));
    // world.push(Box::new(cow_inside));

    let world = Bvh::new(world);

    let camera = Camera::new(CameraParams {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 200,
        max_depth: 40,
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
