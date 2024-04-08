use std::io::{Result, Write};

use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
}

const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;

impl Camera {
    pub fn render<H: Hittable>(&self, world: &H, out: &mut impl Write) -> Result<()> {
        let image_width = self.image_width;
        // Calculate image height, ensuring it's at least 1
        let image_height: usize = ((image_width as f64 / self.aspect_ratio) as usize).max(1);

        // Camera
        let viewport_width = VIEWPORT_HEIGHT * (image_width as f64 / image_height as f64); // Use "real aspect ratio" and not "ideal aspect ratio"
        let camera_center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_du = &viewport_u / image_width as f64;
        let pixel_dv = &viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = &camera_center
            - Vec3::new(0.0, 0.0, FOCAL_LENGTH)
            - &viewport_u / 2.0
            - &viewport_v / 2.0;
        let pixel_00_location = viewport_upper_left + 0.5 * (&pixel_du + &pixel_dv);

        writeln!(out, "P3")?;
        writeln!(out, "{} {}", image_width, image_height)?;
        writeln!(out, "255")?;

        for j in 0..image_height {
            eprintln!("Scanlines remaining: {}", (image_height - j));
            for i in 0..image_width {
                let pixel_center =
                    &pixel_00_location + (i as f64 * &pixel_du) + (j as f64 * &pixel_dv);
                let ray_direction = &pixel_center - &camera_center;
                let ray = Ray::new(camera_center.clone(), ray_direction);
                let color = self.ray_color(&ray, world);

                color.write_ppm(out)?;
            }
        }

        eprintln!("Done.");

        Ok(())
    }

    fn ray_color<H: Hittable>(&self, r: &Ray, world: &H) -> Color {
        let interval = Interval::nonnegative();
        world.hit(r, &interval).map_or_else(
            || {
                let direction = r.direction.normalize();
                let a = 0.5 * (direction.y() + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            },
            |h| 0.5 * (&*h.normal + Color::new(1.0, 1.0, 1.0)),
        )
    }
}
