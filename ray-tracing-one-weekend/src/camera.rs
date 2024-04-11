use std::io::{Result, Write};

use rand::Rng;

use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    utility,
    vec3::{NormalizedVec3, Point3, Vec3},
};

pub struct CameraParams<R> {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub rng: R,
    pub max_depth: usize,
    /// Vertical FOV in degrees
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
}

pub struct Camera<R> {
    params: CameraParams<R>,
    image_height: usize,
    pixel_00_location: Point3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
}

impl<R> Camera<R> {
    pub fn new(params: CameraParams<R>) -> Self {
        let image_width = params.image_width;
        // Calculate image height, ensuring it's at least 1
        let image_height: usize = ((image_width as f64 / params.aspect_ratio) as usize).max(1);

        let camera_center = &params.lookfrom;

        // Determine viewport dimensions
        let focal_length = (camera_center - &params.lookat).length();
        let theta = utility::degrees_to_radians(params.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64); // Use "real aspect ratio" and not "ideal aspect ratio"

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        let w = NormalizedVec3::try_from(camera_center - &params.lookat).unwrap();
        let u = NormalizedVec3::try_from(params.vup.cross(&w)).unwrap();
        let v = NormalizedVec3::try_from(w.cross(&u)).unwrap();

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * &*u;
        let viewport_v = viewport_height * -&*v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_du = &viewport_u / image_width as f64;
        let pixel_dv = &viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            camera_center - (focal_length * &*w) - &viewport_u / 2.0 - &viewport_v / 2.0;
        let pixel_00_location = viewport_upper_left + 0.5 * (&pixel_du + &pixel_dv);

        Camera {
            params,
            image_height,
            pixel_00_location,
            pixel_du,
            pixel_dv,
        }
    }
}

impl<R: Rng> Camera<R> {
    pub fn render<H: Hittable>(&mut self, world: &mut H, out: &mut impl Write) -> Result<()> {
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
                        ray_color(&ray, world, self.params.max_depth)
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

        let ray_origin = self.params.lookfrom.clone();
        let ray_direction = &pixel_sample - &ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    /// Returns a random offset vector in the square surrounding a pixel
    fn pixel_sample_square(&mut self) -> Vec3 {
        let px = -0.5 + self.params.rng.gen::<f64>();
        let py = -0.5 + self.params.rng.gen::<f64>();

        (px * &self.pixel_du) + (py * &self.pixel_dv)
    }
}

fn ray_color<H: Hittable>(r: &Ray, world: &mut H, depth: usize) -> Color {
    if depth == 0 {
        Color::new(0.0, 0.0, 0.0)
    } else {
        let interval = Interval {
            min: 1e-10,
            max: f64::INFINITY,
        };
        let hit = world.hit(r, &interval);
        let scattered = hit.map(|h| h.material.scatter(r, &h.normal, &h.p, &h.face));
        scattered.map_or_else(
            || {
                let direction = r.direction.normalize();
                let a = 0.5 * (direction.y() + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            },
            |s| {
                s.map_or(Color::new(0.0, 0.0, 0.0), |(attenuation, scattered)| {
                    attenuation * ray_color(&scattered, world, depth - 1)
                })
            },
        )
    }
}
