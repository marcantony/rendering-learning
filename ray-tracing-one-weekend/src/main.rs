use std::io::{self, BufWriter, Result, Write};

use ray_tracing_one_weekend::{
    color::Color,
    ray::Ray,
    vec3::{Point3, Vec3},
};

fn main() -> Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;

    // Calculate image height, ensuring it's at least 1
    let image_height: usize = ((image_width as f64 / aspect_ratio) as usize).max(1);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64); // Use "real aspect ratio" and not "ideal aspect ratio"
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_du = &viewport_u / image_width as f64;
    let pixel_dv = &viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        &camera_center - Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
    let pixel_00_location = viewport_upper_left + 0.5 * (&pixel_du + &pixel_dv);

    let mut writer = BufWriter::new(io::stdout().lock());

    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", image_width, image_height)?;
    writeln!(writer, "255")?;

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", (image_height - j));
        for i in 0..image_width {
            let pixel_center = &pixel_00_location + (i as f64 * &pixel_du) + (j as f64 * &pixel_dv);
            let ray_direction = &pixel_center - &camera_center;
            let ray = Ray::new(camera_center.clone(), ray_direction);
            let color = ray_color(&ray);

            color.write_ppm(&mut writer)?;
        }
    }

    eprintln!("Done.");

    Ok(())
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        Color::new(1.0, 0.0, 0.0)
    } else {
        let direction = ray.direction.normalize();
        let a = 0.5 * (direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc: Vec3 = &ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
}
