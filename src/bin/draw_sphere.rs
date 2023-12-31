use ray_tracer_challenge::{
    draw::{canvas::Canvas, color::Color},
    geo::{intersect, ray::Ray, sphere::Sphere},
    math::{transformation, tuple::Tuple3},
    util,
};

fn main() {
    let ray_origin = Tuple3::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half_wall = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let object_transform = transformation::sequence(&vec![
        transformation::scaling(0.5, 1.0, 1.0),
        transformation::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ]);
    let object = Sphere::new(object_transform);

    for y in 0..canvas_pixels {
        let world_y = half_wall - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half_wall + pixel_size * x as f64;
            let target = Tuple3::point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin.clone(), (&target - &ray_origin).norm());

            let xs = object.intersect(&r);
            let hit = xs.as_ref().and_then(|x| intersect::hit(x));

            if hit.is_some() {
                canvas.write((x, y), color.clone());
            }
        }
    }

    util::write_to_file(&canvas, "sphere");
}
