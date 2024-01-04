use crate::{
    draw::canvas::Canvas,
    math::{matrix::InvertibleMatrix, point::Point3d},
};

use super::{ray::Ray, world::World};

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

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        // untransformed coordinates of the pixel in world space
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.inverse() * &Point3d::new(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * &Point3d::new(0.0, 0.0, 0.0);
        let direction = (&pixel - &origin).norm().unwrap();

        Ray { origin, direction }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write((x, y), color);
            }
        }

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

            let r = c.ray_for_pixel(100, 50);

            assert_eq!(
                r,
                Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, -1.0))
            );
        }

        #[test]
        fn constructing_ray_through_corner_of_canvas() {
            let c = Camera::default(201, 101, consts::FRAC_PI_2);

            let r = c.ray_for_pixel(0, 0);

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

            let r = c.ray_for_pixel(100, 50);

            let t = consts::SQRT_2 / 2.0;
            assert_eq!(
                r,
                Ray::new(Point3d::new(0.0, 2.0, -5.0), Vec3d::new(t, 0.0, -t))
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

        let image = c.render(&w);

        color::test_utils::assert_colors_approx_equal(
            image.at(5, 5).unwrap(),
            &Color::new(0.38066, 0.47583, 0.2855),
        );
    }
}
