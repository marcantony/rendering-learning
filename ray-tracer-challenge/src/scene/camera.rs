use crate::{
    draw::canvas::Canvas,
    math::{matrix::InvertibleMatrix, point::Point3d},
};

use super::{ray::Ray, world::World};

use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f64,
    pub transform: InvertibleMatrix<4>,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderOpts {
    pub anti_aliasing_samples: usize,
}

impl Default for RenderOpts {
    fn default() -> Self {
        Self {
            anti_aliasing_samples: 1,
        }
    }
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64, transform: InvertibleMatrix<4>) -> Self {
        let half_view = f64::tan(fov / 2.0);
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            // canvas is horizontal or square
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = half_width * 2.0 / hsize as f64;

        Camera {
            hsize,
            vsize,
            fov,
            transform,
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn default(hsize: usize, vsize: usize, fov: f64) -> Self {
        Camera::new(hsize, vsize, fov, InvertibleMatrix::identity())
    }

    fn rays_for_pixel(&self, px: usize, py: usize, samples: usize) -> impl Iterator<Item = Ray> {
        let pixel_size = self.pixel_size;
        let half_width = self.half_width;
        let half_height = self.half_height;
        let inverse = self.transform.inverse().clone();

        let mut rays = Vec::with_capacity(samples);

        for nx in 0..samples {
            for ny in 0..samples {
                // offset from the edge of the canvas to the pixel's center
                let sample_offset = 1.0 / samples as f64;
                let xoffset = (px as f64 + sample_offset * (nx as f64 + 0.5)) * pixel_size;
                let yoffset = (py as f64 + sample_offset * (ny as f64 + 0.5)) * pixel_size;

                // untransformed coordinates of the pixel in world space
                let world_x = half_width - xoffset;
                let world_y = half_height - yoffset;

                let pixel = &inverse * &Point3d::new(world_x, world_y, -1.0);
                let origin = &inverse * &Point3d::new(0.0, 0.0, 0.0);
                let direction = (&pixel - &origin).norm().unwrap();

                rays.push(Ray { origin, direction })
            }
        }

        rays.into_iter()
    }

    pub fn render(&self, world: &World, opts: &RenderOpts) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        let indices = (0..self.hsize)
            .into_par_iter()
            .flat_map_iter(|x| (0..self.vsize).map(move |y| (x, y)));

        let colors = indices
            .map(|(x, y)| {
                (
                    (x, y),
                    self.rays_for_pixel(x, y, opts.anti_aliasing_samples),
                )
            })
            .map(|(p, rays)| {
                (
                    p,
                    &(rays
                        .map(|r| world.color_at(&r))
                        .reduce(|acc, c| &acc + &c)
                        .unwrap())
                        * (1.0 / (opts.anti_aliasing_samples.pow(2)) as f64),
                )
            })
            .collect::<Vec<_>>();

        colors.into_iter().for_each(|(p, c)| {
            image.write(p, c);
        });

        image
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use crate::{
        draw::color::{self, Color},
        math::{util, vector::Vec3d},
        scene::{transformation, world::World},
    };

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = consts::FRAC_PI_2;

        let c = Camera::default(hsize, vsize, fov);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.fov, consts::FRAC_PI_2);
        assert_eq!(c.transform, InvertibleMatrix::identity())
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::default(200, 125, consts::FRAC_PI_2);
        assert!(util::are_equal(c.pixel_size, 0.01));
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::default(125, 200, consts::FRAC_PI_2);
        assert!(util::are_equal(c.pixel_size, 0.01));
    }

    mod ray {
        use crate::{
            math::{
                point::Point3d,
                vector::{self, Vec3d},
            },
            scene::transformation,
        };

        use super::*;

        #[test]
        fn constructing_ray_through_center_of_canvas() {
            let c = Camera::default(201, 101, consts::FRAC_PI_2);

            let r = c.rays_for_pixel(100, 50, 1);

            assert_eq!(
                r.collect::<Vec<_>>(),
                vec![Ray::new(
                    Point3d::new(0.0, 0.0, 0.0),
                    Vec3d::new(0.0, 0.0, -1.0)
                )]
            );
        }

        #[test]
        fn constructing_ray_through_corner_of_canvas() {
            let c = Camera::default(201, 101, consts::FRAC_PI_2);

            let r = &c.rays_for_pixel(0, 0, 1).collect::<Vec<_>>()[0];

            assert_eq!(r.origin, Point3d::new(0.0, 0.0, 0.0));
            vector::test_utils::assert_vec_approx_equals(
                &r.direction,
                &Vec3d::new(0.66519, 0.33259, -0.66851),
            );
        }

        #[test]
        fn constructing_ray_when_camera_is_transformed() {
            let transform = transformation::sequence(&vec![
                transformation::translation(0.0, -2.0, 5.0),
                transformation::rotation_y(consts::FRAC_PI_4),
            ]);
            let c = Camera::new(
                201,
                101,
                consts::FRAC_PI_2,
                InvertibleMatrix::try_from(transform).unwrap(),
            );

            let r = &c.rays_for_pixel(100, 50, 1).collect::<Vec<_>>()[0];

            let t = consts::SQRT_2 / 2.0;
            assert_eq!(
                r,
                &Ray::new(Point3d::new(0.0, 2.0, -5.0), Vec3d::new(t, 0.0, -t))
            );
        }

        #[test]
        fn generating_multiple_samples_for_antialiasing() {
            let c = Camera::default(1, 1, consts::FRAC_PI_2);

            let r = c.rays_for_pixel(0, 0, 2);
            let dt = c.pixel_size / 4.0;

            assert_eq!(
                r.collect::<Vec<_>>(),
                vec![
                    Ray::new(
                        Point3d::new(0.0, 0.0, 0.0),
                        Vec3d::new(dt, dt, -1.0).norm().unwrap()
                    ),
                    Ray::new(
                        Point3d::new(0.0, 0.0, 0.0),
                        Vec3d::new(dt, -dt, -1.0).norm().unwrap()
                    ),
                    Ray::new(
                        Point3d::new(0.0, 0.0, 0.0),
                        Vec3d::new(-dt, dt, -1.0).norm().unwrap()
                    ),
                    Ray::new(
                        Point3d::new(0.0, 0.0, 0.0),
                        Vec3d::new(-dt, -dt, -1.0).norm().unwrap()
                    ),
                ]
            );
        }
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::basic();
        let from = Point3d::new(0.0, 0.0, -5.0);
        let to = Point3d::new(0.0, 0.0, 0.0);
        let up = Vec3d::new(0.0, 1.0, 0.0);
        let transform =
            InvertibleMatrix::try_from(transformation::view_transform(&from, &to, &up)).unwrap();
        let c = Camera::new(11, 11, consts::FRAC_PI_2, transform);

        let image = c.render(&w, &Default::default());

        color::test_utils::assert_colors_approx_equal(
            image.at(5, 5).unwrap(),
            &Color::new(0.38066, 0.47583, 0.2855),
        );
    }
}
