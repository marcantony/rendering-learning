#![allow(dead_code)]

use std::f64::consts;

use ray_tracer_challenge::{
    draw::color::{self, Color},
    math::{matrix::InvertibleMatrix, point::Point3d, vector::Vec3d},
    scene::{
        camera::Camera,
        light::PointLight,
        material::{Material, Surface},
        object::{
            bounded::Bounded, cube::Cube, cylinder::Cylinder, group::Group, plane::Plane,
            sphere::Sphere, transformed::Transformed, Object,
        },
        pattern::{checker3d::Checker3d, stripe::Stripe},
        transformation,
        world::World,
    },
    util,
};

const RES_X: usize = 600;
const RES_Y: usize = 400;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let world = test_hexagon_world();

    let from = Point3d::new(0.0, 2.0, -7.0);
    let to = Point3d::new(0.0, 1.5, 0.0);
    let up = Vec3d::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        RES_X,
        RES_Y,
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

fn hexagon_corner() -> impl Object {
    let corner = Sphere::unit();
    let transform = transformation::sequence(&vec![
        transformation::scaling(0.25, 0.25, 0.25),
        transformation::translation(0.0, 0.0, -1.0),
    ]);
    Transformed::new(corner, InvertibleMatrix::try_from(transform).unwrap())
}

fn hexagon_edge() -> impl Object {
    let edge = Cylinder {
        minimum: Some(0.0),
        maximum: Some(1.0),
        ..Default::default()
    };
    let transform = transformation::sequence(&vec![
        transformation::scaling(0.25, 1.0, 0.25),
        transformation::rotation_z(-std::f64::consts::FRAC_PI_2),
        transformation::rotation_y(-std::f64::consts::FRAC_PI_6),
        transformation::translation(0.0, 0.0, -1.0),
    ]);
    Transformed::new(edge, InvertibleMatrix::try_from(transform).unwrap())
}

fn hexagon_side() -> impl Object {
    Bounded::new(Group::<Box<dyn Object>>::new(vec![
        Box::new(hexagon_corner()),
        Box::new(hexagon_edge()),
    ]))
}

fn hexagon() -> impl Object {
    let sides: Vec<_> = (0..6)
        .map(|n| {
            let transform = transformation::rotation_y(n as f64 * std::f64::consts::FRAC_PI_3);
            Transformed::new(
                hexagon_side(),
                InvertibleMatrix::try_from(transform).unwrap(),
            )
        })
        .map(|s| Box::new(s) as Box<dyn Object>)
        .collect();

    Group::new(sides)
}

fn test_hexagon_world() -> World {
    let light_source = PointLight {
        position: Point3d::new(-10.0, 10.0, -10.0),
        intensity: color::white(),
    };

    let hexagon = Transformed::new(
        hexagon(),
        InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::scaling(2.0, 2.0, 2.0),
            transformation::rotation_x(-std::f64::consts::FRAC_PI_4),
            transformation::translation(0.0, 2.0, 0.0),
        ]))
        .unwrap(),
    );

    World {
        objects: vec![Box::new(hexagon)],
        lights: vec![light_source],
        ..Default::default()
    }
}

fn test_mirror_world() -> World {
    let gs1 = Transformed::new(
        Sphere::unit(),
        InvertibleMatrix::try_from(transformation::translation(-0.5, 0.0, 0.0)).unwrap(),
    );
    let gs2 = Transformed::new(
        Sphere::unit(),
        InvertibleMatrix::try_from(transformation::translation(0.5, 0.0, 0.0)).unwrap(),
    );
    let sphere_group = Bounded::new(Transformed::new(
        Group::new(vec![Box::new(gs1), Box::new(gs2)]),
        InvertibleMatrix::try_from(transformation::translation(-1.0, 0.0, 0.0)).unwrap(),
    ));

    let floor = Plane {
        material: Material {
            surface: Surface::Pattern(Box::new(Checker3d {
                a: color::white(),
                b: color::black(),
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, -0.01, 0.0))
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
        InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::rotation_y(-consts::FRAC_PI_3),
            transformation::translation(-8.0, 0.0, 0.0),
        ]))
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
        InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::rotation_y(consts::FRAC_PI_4),
            transformation::translation(10.0, 0.0, 0.0),
        ]))
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
        InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::translation(0.0, 0.0, 7.0),
        ]))
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
        InvertibleMatrix::try_from(transformation::translation(0.0, 2.0, 0.0)).unwrap(),
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
        InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::scaling(0.5, 0.5, 0.5),
            transformation::translation(0.0, 2.0, 0.0),
        ]))
        .unwrap(),
    );

    let behind_cube = Transformed::new(
        Cube {
            material: Material {
                surface: Surface::Pattern(Box::new(Stripe {
                    a: Color::new(0.545, 0.0, 0.0),
                    b: Color::new(0.0, 0.392, 0.0),
                    transform: InvertibleMatrix::try_from(transformation::scaling(0.2, 1.0, 1.0))
                        .unwrap(),
                })),
                ..Default::default()
            },
        },
        InvertibleMatrix::try_from(transformation::translation(3.0, 0.0, -10.0)).unwrap(),
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
        InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::translation(0.0, 0.0, -100.0),
        ]))
        .unwrap(),
    );

    let light_source = PointLight {
        position: Point3d::new(-10.0, 10.0, -10.0),
        intensity: color::white(),
    };

    World {
        objects: vec![
            Box::new(floor),
            Box::new(left_wall),
            Box::new(right_wall),
            Box::new(middle_wall),
            Box::new(ball),
            Box::new(inner_air_pocket),
            Box::new(behind_cube),
            Box::new(behind_wall),
            Box::new(sphere_group),
        ],
        lights: vec![light_source],
        ..Default::default()
    }
}
