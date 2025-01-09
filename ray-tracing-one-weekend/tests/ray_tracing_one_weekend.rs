use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    color::Color,
    hittable::sphere::{Center, Sphere},
    material::{Dielectric, Lambertian, Material, Metal},
    texture::SolidColor,
    vec3::{Point3, Vec3},
};

#[test]
fn test_render() {
    let expected_data = include_str!("expectations/test.ppm");

    let material_ground = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.8, 0.8, 0.0),
        },
    };
    let material_center = Lambertian {
        texture: SolidColor {
            albedo: Color::new(0.1, 0.2, 0.5),
        },
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
    let world: [Sphere<&dyn Material>; 5] = [
        Sphere {
            center: Center::Stationary(Point3::new(0.0, -100.5, -1.0)),
            radius: 100.0,
            material: &material_ground,
        },
        Sphere {
            center: Center::Stationary(Point3::new(0.0, 0.0, -1.2)),
            radius: 0.5,
            material: &material_center,
        },
        Sphere {
            center: Center::Stationary(Point3::new(-1.0, 0.0, -1.0)),
            radius: 0.5,
            material: &material_left,
        },
        Sphere {
            center: Center::Stationary(Point3::new(-1.0, 0.0, -1.0)),
            radius: 0.4,
            material: &material_bubble,
        },
        Sphere {
            center: Center::Stationary(Point3::new(1.0, 0.0, -1.0)),
            radius: 0.5,
            material: &material_right,
        },
    ];

    let camera = Camera::new(CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 300,
        samples_per_pixel: 10,
        max_depth: 10,
        vfov: 20.0,
        lookfrom: Point3::new(-2.0, 2.0, 1.0),
        lookat: Point3::new(0.0, 0.0, -1.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 10.0,
        focus_dist: 3.4,
        background: Color::new(0.7, 0.8, 1.0),
    });

    let mut out = Vec::new();

    camera.render(0, &world.as_slice(), &mut out).unwrap();

    let out_str = String::from_utf8(out).unwrap();
    // std::fs::write("test-out.ppm", &out_str).unwrap(); // Uncomment to view test output
    assert!(
        &out_str == expected_data,
        "output image did not equal expectation"
    );
}
