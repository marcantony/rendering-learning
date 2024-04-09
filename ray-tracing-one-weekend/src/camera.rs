use std::io::{Result, Write};

use rand::Rng;

use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct CameraParams<R> {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub rng: R,
    pub max_depth: usize,
}

pub struct Camera<R> {
    params: CameraParams<R>,
    image_height: usize,
    center: Point3,
    pixel_00_location: Point3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
}

const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;

impl<R> Camera<R> {
    pub fn new(params: CameraParams<R>) -> Self {
        let image_width = params.image_width;
        // Calculate image height, ensuring it's at least 1
        let image_height: usize = ((image_width as f64 / params.aspect_ratio) as usize).max(1);

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

        Camera {
            params,
            image_height,
            center: camera_center,
            pixel_00_location,
            pixel_du,
            pixel_dv,
        }
    }
}

impl<R: Rng> Camera<R> {
    pub fn render<H: Hittable>(&mut self, world: &H, out: &mut impl Write) -> Result<()> {
        let image_width = self.params.image_width;
        let image_height = self.image_height;

        writeln!(out, "P3")?;
        writeln!(out, "{} {}", image_width, image_height)?;
        writeln!(out, "255")?;

        for j in 0..image_height {
            eprintln!("Scanlines remaining: {}", (image_height - j));
            for i in 0..image_width {
                let color = (0..self.params.samples_per_pixel)
                    .map(|_n| {
                        let ray = self.get_ray(i, j);
                        self.ray_color(&ray, world, self.params.max_depth)
                    })
                    .fold(Color::new(0.0, 0.0, 0.0), |acc, c| acc + c)
                    / self.params.samples_per_pixel as f64;

                color.write_ppm(out)?;
            }
        }

        eprintln!("Done.");

        Ok(())
    }

    /// Returns a randomly sampled camera ray for the pixel at location (i, j)
    fn get_ray(&mut self, i: usize, j: usize) -> Ray {
        let pixel_center =
            &self.pixel_00_location + (i as f64 * &self.pixel_du) + (j as f64 * &self.pixel_dv);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center.clone();
        let ray_direction = &pixel_sample - &ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    /// Returns a random offset vector in the square surrounding a pixel
    fn pixel_sample_square(&mut self) -> Vec3 {
        let px = -0.5 + self.params.rng.gen::<f64>();
        let py = -0.5 + self.params.rng.gen::<f64>();

        (px * &self.pixel_du) + (py * &self.pixel_dv)
    }

    fn ray_color<H: Hittable>(&mut self, r: &Ray, world: &H, depth: usize) -> Color {
        if depth == 0 {
            Color::new(0.0, 0.0, 0.0)
        } else {
            let interval = Interval {
                min: 1e-10,
                max: f64::INFINITY,
            };
            world.hit(r, &interval).map_or_else(
                || {
                    let direction = r.direction.normalize();
                    let a = 0.5 * (direction.y() + 1.0);
                    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
                },
                |h| {
                    let direction =
                        Vec3::random_on_unit_hemisphere(&mut self.params.rng, &h.normal);
                    let next_ray = Ray::new(h.p.clone(), direction);
                    0.5 * self.ray_color(&next_ray, world, depth - 1)
                },
            )
        }
    }
}
