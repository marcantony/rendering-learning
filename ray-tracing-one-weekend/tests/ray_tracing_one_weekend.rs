use ray_tracing_one_weekend::{
    camera::{Camera, CameraParams},
    color::Color,
    hittable::{
        sphere::{Center, Sphere},
        Hittable,
    },
    material::{Dielectric, Lambertian, Material, Metal},
    output,
    texture::SolidColor,
    vec3::{Point3, Vec3},
};

fn test_scene() -> (impl Hittable<Material = impl Material>, CameraParams) {
    // World
    let world = [
        Sphere {
            center: Center::Stationary(Point3::new(0.0, -100.5, -1.0)),
            radius: 100.0,
            material: Box::new(Lambertian {
                texture: SolidColor {
                    albedo: Color::new(0.8, 0.8, 0.0),
                },
            }) as Box<dyn Material>,
        },
        Sphere {
            center: Center::Stationary(Point3::new(0.0, 0.0, -1.2)),
            radius: 0.5,
            material: Box::new(Lambertian {
                texture: SolidColor {
                    albedo: Color::new(0.1, 0.2, 0.5),
                },
            }) as Box<dyn Material>,
        },
        Sphere {
            center: Center::Stationary(Point3::new(-1.0, 0.0, -1.0)),
            radius: 0.5,
            material: Box::new(Dielectric {
                refraction_index: 1.5,
            }) as Box<dyn Material>,
        },
        Sphere {
            center: Center::Stationary(Point3::new(-1.0, 0.0, -1.0)),
            radius: 0.4,
            material: Box::new(Dielectric {
                refraction_index: 1.0 / 1.5,
            }) as Box<dyn Material>,
        },
        Sphere {
            center: Center::Stationary(Point3::new(1.0, 0.0, -1.0)),
            radius: 0.5,
            material: Box::new(Metal {
                albedo: Color::new(0.8, 0.6, 0.2),
                fuzz: 1.0,
            }) as Box<dyn Material>,
        },
    ];

    let params = CameraParams {
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
        seed: 0,
    };

    (world, params)
}

#[test]
fn test_render() {
    let expected_data = include_str!("expectations/test.ppm");

    let (world, params) = test_scene();
    let camera = Camera::new(params);

    let canvas = camera.render(&world);

    let mut out = Vec::new();
    output::output_ppm(&canvas, &mut out).unwrap();

    let out_str = String::from_utf8(out).unwrap();
    std::fs::write("test-out.ppm", &out_str).unwrap(); // Uncomment to view test output
    assert!(
        &out_str == expected_data,
        "output image did not equal expectation"
    );
}

#[test]
fn test_rendering_deterministic() {
    let (world, mut params) = test_scene();
    // Increase test speed
    params.samples_per_pixel = 5;
    params.image_width = 100;

    let camera = Camera::new(params);

    let render1 = camera.render(&world);
    let render2 = camera.render(&world);

    let data1 = render1.pixel_data().collect::<Vec<_>>();
    let data2 = render2.pixel_data().collect::<Vec<_>>();

    assert!(data1 == data2);
}

#[test]
fn test_checkpointing_deterministic() {
    let (world, mut params) = test_scene();
    // Increase test speed
    params.samples_per_pixel = 5;
    params.image_width = 100;

    let camera = Camera::new(params);

    let render1 = camera.render(&world);
    let checkpointed1 = camera.render_from_checkpoint(&world, &render1);
    let checkpointed2 = camera.render_from_checkpoint(&world, &render1);

    let render1_data = render1.pixel_data().collect::<Vec<_>>();
    let checkpoint1_data = checkpointed1.pixel_data().collect::<Vec<_>>();
    let checkpoint2_data = checkpointed2.pixel_data().collect::<Vec<_>>();

    // A render and a subsequent render from the checkpoint should be different
    // due to using different RNG states
    assert!(render1_data != checkpoint1_data);

    // Renders from the same checkpoint should equivalent
    assert!(checkpoint1_data == checkpoint2_data);
}

#[test]
fn checkpointing_and_no_checkpointing_not_equivalent() {
    let (world, mut params) = test_scene();
    // Increase test speed
    params.samples_per_pixel = 10;
    params.image_width = 100;

    let mut low_sample_params = params.clone();
    low_sample_params.samples_per_pixel = 5;

    let lq_camera = Camera::new(low_sample_params);
    let hq_camera = Camera::new(params);

    let lq1 = lq_camera.render(&world);
    let lq2 = lq_camera.render_from_checkpoint(&world, &lq1);

    let hq = hq_camera.render(&world);

    let lq_data = lq2.pixel_data().collect::<Vec<_>>();
    let hq_data = hq.pixel_data().collect::<Vec<_>>();

    assert!(lq_data != hq_data);
}
