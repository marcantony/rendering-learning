use ray_tracer_challenge::{
    draw::{
        canvas::Canvas,
        color::{self, Color},
    },
    math::{point::Point3d, vector::NormalizedVec3d},
    scene::{
        intersect,
        light::PointLight,
        material::{self, Material},
        ray::Ray,
        sphere::Sphere,
        transformation,
    },
    util,
};

fn main() {
    let ray_origin = Point3d::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half_wall = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let object_transform = transformation::sequence(&vec![
        transformation::scaling(1.0, 1.0, 1.0),
        transformation::shearing(0.5, 0.0, 0.0, 0.0, 0.0, 0.0),
    ]);
    let object = Sphere::new(
        object_transform,
        Material {
            color: Color::new(1.0, 0.2, 1.0),
            ..Default::default()
        },
    );
    let light = PointLight {
        position: Point3d::new(10.0, -10.0, -10.0),
        intensity: color::white(),
    };

    println!("Rendering scene...");
    for y in 0..canvas_pixels {
        let world_y = half_wall - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half_wall + pixel_size * x as f64;
            let target = Point3d::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin.clone(), (&target - &ray_origin).norm().unwrap());

            let xs = object.intersect(&r);
            let hit = xs.as_ref().and_then(|x| intersect::hit(x));

            if let Some(intersect) = hit {
                let point = r.position(intersect.t());
                let normalv = intersect.object().normal_at(&point);
                let eyev = NormalizedVec3d::try_from(-r.direction()).unwrap();
                let color = material::lighting(
                    intersect.object().material(),
                    &point,
                    &light,
                    &eyev,
                    &normalv,
                );
                canvas.write((x, y), color);
            }
        }
    }
    println!("Scene rendered.");

    util::write_to_file(&canvas, "sphere");
}
