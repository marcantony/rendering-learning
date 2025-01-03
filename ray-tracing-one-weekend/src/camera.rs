use std::io::{Result, Write};

use rand::Rng;
use rand_distr::{Distribution, UnitDisc};

use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    material::Material,
    ray::Ray,
    utility,
    vec3::{NormalizedVec3, Point3, Vec3},
};

pub struct CameraParams {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    /// Vertical FOV in degrees
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    /// In degrees
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

pub struct Camera {
    params: CameraParams,
    image_height: usize,
    pixel_00_location: Point3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(params: CameraParams) -> Self {
        let image_width = params.image_width;
        // Calculate image height, ensuring it's at least 1
        let image_height: usize = ((image_width as f64 / params.aspect_ratio) as usize).max(1);

        let camera_center = &params.lookfrom;

        // Determine viewport dimensions
        let theta = utility::degrees_to_radians(params.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * params.focus_dist;
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
            camera_center - (params.focus_dist * &*w) - &viewport_u / 2.0 - &viewport_v / 2.0;
        let pixel_00_location = viewport_upper_left + 0.5 * (&pixel_du + &pixel_dv);

        // Calculate the camera defocus disk basis vectors
        let defocus_radius =
            params.focus_dist * f64::tan(utility::degrees_to_radians(params.defocus_angle / 2.0));
        let defocus_disk_u = &*u * defocus_radius;
        let defocus_disk_v = &*v * defocus_radius;

        Camera {
            params,
            image_height,
            pixel_00_location,
            pixel_du,
            pixel_dv,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}

impl Camera {
    pub fn render<R: Rng, M: Material, H: Hittable<Material = M>>(
        &self,
        mut rng: &mut R,
        world: &H,
        out: &mut impl Write,
    ) -> Result<()> {
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
                        let ray = self.get_ray(&mut rng, i, j);
                        ray_color(rng, &ray, world, self.params.max_depth)
                    })
                    .fold(Color::new(0.0, 0.0, 0.0), |acc, c| acc + c)
                    / self.params.samples_per_pixel as f64;

                color.write_ppm(out)?;
            }
        }

        eprintln!("Done.");

        Ok(())
    }

    /// Returns a randomly sampled camera ray for the pixel at location (i, j).
    /// The ray will originate from the defocus disk.
    fn get_ray(&self, rng: &mut impl Rng, i: usize, j: usize) -> Ray {
        let pixel_center =
            &self.pixel_00_location + (i as f64 * &self.pixel_du) + (j as f64 * &self.pixel_dv);
        let pixel_sample = pixel_center + self.pixel_sample_square(rng);

        let ray_origin = if self.params.defocus_angle <= 0.0 {
            self.params.lookfrom.clone()
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = &pixel_sample - &ray_origin;
        let ray_time = rng.gen();
        Ray::new_at_time(ray_origin, ray_direction, ray_time)
    }

    /// Returns a random offset vector in the square surrounding a pixel
    fn pixel_sample_square(&self, rng: &mut impl Rng) -> Vec3 {
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();

        (px * &self.pixel_du) + (py * &self.pixel_dv)
    }

    /// Returns a random point in the camera defocus disk
    fn defocus_disk_sample(&self, rng: &mut impl Rng) -> Vec3 {
        let [px, py]: [f64; 2] = UnitDisc.sample(rng);
        &self.params.lookfrom + (px * &self.defocus_disk_u) + (py * &self.defocus_disk_v)
    }
}

fn ray_color<R: Rng, M: Material, H: Hittable<Material = M>>(
    mut rng: &mut R,
    r: &Ray,
    world: &H,
    depth: usize,
) -> Color {
    if depth == 0 {
        Color::new(0.0, 0.0, 0.0)
    } else {
        let interval = Interval {
            min: 1e-10,
            max: f64::INFINITY,
        };
        let hit = world.hit(r, &interval);
        let scattered = hit.map(|(m, h)| m.scatter(&mut rng, r, &h));
        scattered.map_or_else(
            || {
                let direction = r.direction.normalize();
                let a = 0.5 * (direction.y() + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            },
            |s| {
                s.map_or(Color::new(0.0, 0.0, 0.0), |(attenuation, scattered)| {
                    attenuation * ray_color(rng, &scattered, world, depth - 1)
                })
            },
        )
    }
}
