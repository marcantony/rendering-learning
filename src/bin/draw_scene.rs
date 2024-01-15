use std::f64::consts;

use ray_tracer_challenge::{
    draw::color::{self, Color},
    math::{matrix::InvertibleMatrix, point::Point3d, vector::Vec3d},
    scene::{
        camera::Camera,
        light::PointLight,
        material::{Material, Surface},
        object::{cube::Cube, plane::Plane, sphere::Sphere},
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

    let floor = Plane {
        transform: InvertibleMatrix::identity(),
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

    let left_wall = Plane {
        transform: InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::rotation_y(-consts::FRAC_PI_3),
            transformation::translation(-8.0, 0.0, 0.0),
        ]))
        .unwrap(),
        material: Material {
            surface: Surface::Color(color::white()),
            specular: 1.0,
            reflectivity: 0.9,
            shininess: 400.0,
            diffuse: 0.0,
            ..Default::default()
        },
    };

    let right_wall = Plane {
        transform: InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::rotation_y(consts::FRAC_PI_4),
            transformation::translation(10.0, 0.0, 0.0),
        ]))
        .unwrap(),
        material: Material {
            surface: Surface::Color(color::white()),
            specular: 1.0,
            reflectivity: 1.0,
            shininess: 400.0,
            diffuse: 0.0,
            ..Default::default()
        },
    };

    let middle_wall = Plane {
        transform: InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::translation(0.0, 0.0, 7.0),
        ]))
        .unwrap(),
        material: Material {
            surface: Surface::Color(Color::new(0.945, 0.788, 0.647)),
            specular: 0.1,
            shininess: 50.0,
            ..Default::default()
        },
    };

    let ball = Sphere {
        transform: InvertibleMatrix::try_from(transformation::translation(0.0, 2.0, 0.0)).unwrap(),
        material: Material {
            surface: Surface::Color(Color::new(0.059, 0.322, 0.729)),
            diffuse: 0.3,
            specular: 1.0,
            reflectivity: 0.9,
            transparency: 0.75,
            refractive_index: 1.52,
            ..Default::default()
        },
    };

    let inner_air_pocket = Sphere {
        transform: InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::scaling(0.5, 0.5, 0.5),
            transformation::translation(0.0, 2.0, 0.0),
        ]))
        .unwrap(),
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
    };

    let behind_cube = Cube {
        transform: InvertibleMatrix::try_from(transformation::translation(3.0, 0.0, -10.0))
            .unwrap(),
        material: Material {
            surface: Surface::Pattern(Box::new(Stripe {
                a: Color::new(0.545, 0.0, 0.0),
                b: Color::new(0.0, 0.392, 0.0),
                transform: InvertibleMatrix::try_from(transformation::scaling(0.2, 1.0, 1.0))
                    .unwrap(),
            })),
            ..Default::default()
        },
    };

    let behind_wall = Plane {
        transform: InvertibleMatrix::try_from(transformation::sequence(&vec![
            transformation::rotation_x(consts::FRAC_PI_2),
            transformation::translation(0.0, 0.0, -100.0),
        ]))
        .unwrap(),
        material: Material {
            surface: Surface::Color(Color::new(0.678, 0.847, 0.902)),
            specular: 0.1,
            shininess: 50.0,
            ..Default::default()
        },
    };

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
            Box::new(behind_cube),
            Box::new(behind_wall),
        ],
        lights: vec![light_source],
        ..Default::default()
    };

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
