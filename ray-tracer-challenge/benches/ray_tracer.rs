use std::{
    f64::consts,
    time::{Duration, Instant},
};

use ray_tracer_challenge::{
    draw::color::{self, Color},
    math::{point::Point3d, vector::Vec3d},
    scene::{
        camera::Camera,
        light::PointLight,
        material::{Material, Surface},
        object::{plane::Plane, sphere::Sphere, transformed::Transformed},
        pattern::checker3d::Checker3d,
        transformation,
        world::World,
        Scene,
    },
};

fn main() {
    let iterations = 300;
    let scene = test_mirror_scene();
    let results = (0..iterations)
        .map(|_i| time(|| scene.render(&Default::default())))
        .map(|(_c, d)| d)
        .collect::<Vec<_>>();

    let avg_duration = results.iter().sum::<Duration>() / iterations;

    println!(
        "Ran {} iterations. Average duration: {:?}",
        iterations, avg_duration
    );
}

fn time<F, T>(f: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let now = Instant::now();
    let result = f();
    let elapsed = now.elapsed();

    (std::hint::black_box(result), elapsed)
}

fn test_mirror_scene() -> Scene {
    let floor = Plane {
        material: Material {
            surface: Surface::Pattern(Box::new(Checker3d {
                a: color::white(),
                b: color::black(),
                transform: transformation::translation(0.0, -0.01, 0.0)
                    .try_into()
                    .unwrap(),
            })),
            specular: 0.0,
            reflectivity: 0.02,
            ..Default::default()
        },
    };

    let left_wall = Transformed::new(
        Plane {
            material: Material {
                surface: Surface::Color(color::white()),
                specular: 1.0,
                reflectivity: 0.9,
                shininess: 400.0,
                diffuse: 0.0,
                ..Default::default()
            },
        },
        transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::rotation_y(-consts::FRAC_PI_3),
            transformation::translation(-8.0, 0.0, 0.0),
        ])
        .try_into()
        .unwrap(),
    );

    let right_wall = Transformed::new(
        Plane {
            material: Material {
                surface: Surface::Color(color::white()),
                specular: 1.0,
                reflectivity: 1.0,
                shininess: 400.0,
                diffuse: 0.0,
                ..Default::default()
            },
        },
        transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::rotation_y(consts::FRAC_PI_4),
            transformation::translation(10.0, 0.0, 0.0),
        ])
        .try_into()
        .unwrap(),
    );

    let middle_wall = Transformed::new(
        Plane {
            material: Material {
                surface: Surface::Color(Color::new(0.945, 0.788, 0.647)),
                specular: 0.1,
                shininess: 50.0,
                ..Default::default()
            },
        },
        transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::translation(0.0, 0.0, 7.0),
        ])
        .try_into()
        .unwrap(),
    );

    let ball = Transformed::new(
        Sphere {
            material: Material {
                surface: Surface::Color(Color::new(0.059, 0.322, 0.729)),
                diffuse: 0.3,
                specular: 1.0,
                reflectivity: 0.9,
                transparency: 0.75,
                refractive_index: 1.52,
                ..Default::default()
            },
        },
        transformation::translation(0.0, 2.0, 0.0)
            .try_into()
            .unwrap(),
    );

    let inner_air_pocket = Transformed::new(
        Sphere {
            material: Material {
                surface: Surface::Color(color::white()),
                ambient: 0.0,
                diffuse: 0.0,
                specular: 0.0,
                transparency: 1.0,
                refractive_index: 1.0,
                reflectivity: 1.0,
                ..Default::default()
            },
        },
        transformation::sequence(&vec![
            transformation::scaling(0.5, 0.5, 0.5),
            transformation::translation(0.0, 2.0, 0.0),
        ])
        .try_into()
        .unwrap(),
    );

    let behind_wall = Transformed::new(
        Plane {
            material: Material {
                surface: Surface::Color(Color::new(0.678, 0.847, 0.902)),
                specular: 0.1,
                shininess: 50.0,
                ..Default::default()
            },
        },
        transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::translation(0.0, 0.0, -100.0),
        ])
        .try_into()
        .unwrap(),
    );

    let light_source = PointLight {
        position: Point3d::new(-10.0, 10.0, -10.0),
        intensity: color::white(),
    };

    let world = World {
        objects: vec![
            Box::new(floor),
            Box::new(left_wall),
            Box::new(right_wall),
            Box::new(middle_wall),
            Box::new(ball),
            Box::new(inner_air_pocket),
            Box::new(behind_wall),
        ],
        lights: vec![light_source],
        ..Default::default()
    };

    let from = Point3d::new(0.0, 2.0, -7.0);
    let to = Point3d::new(0.0, 1.5, 0.0);
    let up = Vec3d::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        300,
        200,
        consts::FRAC_PI_3,
        transformation::view_transform(&from, &to, &up)
            .try_into()
            .unwrap(),
    );

    Scene { camera, world }
}
