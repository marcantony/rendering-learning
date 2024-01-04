use std::f64::consts;

use ray_tracer_challenge::{
    draw::color::Color,
    math::{matrix::InvertibleMatrix, point::Point3d, vector::Vec3d},
    scene::{
        camera::Camera, light::PointLight, material::Material, sphere::Sphere, transformation,
        world::World,
    },
    util,
};

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let floor_material = Material {
        color: Color::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Default::default()
    };
    let floor = Sphere {
        transform: InvertibleMatrix::try_from(transformation::scaling(10.0, 0.01, 10.0)).unwrap(),
        material: floor_material.clone(),
    };

    let left_wall = Sphere {
        transform: InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::scaling(10.0, 0.01, 10.0),
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::rotation_y(-consts::FRAC_PI_4),
            transformation::translation(0.0, 0.0, 5.0),
        ]))
        .unwrap(),
        material: floor_material.clone(),
    };

    let right_wall = Sphere {
        transform: InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::scaling(10.0, 0.01, 10.0),
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::rotation_y(consts::FRAC_PI_4),
            transformation::translation(0.0, 0.0, 5.0),
        ]))
        .unwrap(),
        material: floor_material.clone(),
    };

    let middle = Sphere {
        transform: InvertibleMatrix::try_from(transformation::translation(-0.5, 1.0, 0.5)).unwrap(),
        material: Material {
            color: Color::new(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    };

    let right = Sphere {
        transform: InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::scaling(0.5, 0.5, 0.5),
            transformation::translation(1.5, 0.5, -0.5),
        ]))
        .unwrap(),
        material: Material {
            color: Color::new(0.5, 1.0, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    };

    let left = Sphere {
        transform: InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::scaling(0.33, 0.33, 0.33),
            transformation::translation(-1.5, 0.33, -0.75),
        ]))
        .unwrap(),
        material: Material {
            color: Color::new(1.0, 0.8, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    };

    let light_source = PointLight {
        position: Point3d::new(-10.0, 10.0, -10.0),
        intensity: Color::new(1.0, 1.0, 1.0),
    };

    let world = World {
        objects: vec![floor, left_wall, right_wall, left, middle, right],
        light: Some(light_source),
    };

    let from = Point3d::new(0.0, 1.5, -5.0);
    let to = Point3d::new(0.0, 1.0, 0.0);
    let up = Vec3d::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        1000,
        500,
        consts::FRAC_PI_3,
        InvertibleMatrix::try_from(transformation::view_transform(&from, &to, &up)).unwrap(),
    );

    println!("Rendering scene...");
    let canvas = camera.render(&world);
    println!("Scene rendered.");

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    util::write_to_file(&canvas, "scene");
}
