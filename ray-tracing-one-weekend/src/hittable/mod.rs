pub mod constant_medium;
pub mod flat;
pub mod sphere;
pub mod transform;
pub mod translate;

use transform::Transform;
use translate::Translate;

use crate::{
    aabb::AABB,
    interval::Interval,
    ray::Ray,
    vec3::{NormalizedVec3, Point3, Vec3},
};

#[derive(Debug, PartialEq)]
pub enum Face {
    Front,
    Back,
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: NormalizedVec3,
    pub t: f64,
    pub uv: (f64, f64),
    pub face: Face,
}

pub fn calculate_face_normal(r: &Ray, outward_normal: NormalizedVec3) -> (NormalizedVec3, Face) {
    if r.direction.dot(&outward_normal) <= 0.0 {
        (outward_normal, Face::Front)
    } else {
        (-&outward_normal, Face::Back)
    }
}

pub trait Hittable: Sync + Send {
    type Material;
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)>;
    fn bounding_box(&self) -> AABB;

    fn translate(self, offset: Vec3) -> Translate<Self>
    where
        Self: Sized,
    {
        Translate {
            object: self,
            offset,
        }
    }
    /// Rotates the hittable counter-clockwise about the Y-axis by the given number of degrees
    fn rotate_y(self, degrees: f64) -> Transform<Self>
    where
        Self: Sized,
        Self::Material: crate::material::Material,
    {
        Transform::rotate_y(self, degrees)
    }
}

impl<M, H: Hittable<Material = M>> Hittable for [H] {
    type Material = M;
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&M, HitRecord)> {
        self.iter().fold(None, |closest_so_far, current_hittable| {
            let new_max = closest_so_far
                .as_ref()
                .map_or(ray_t.max, |closest| closest.1.t);
            current_hittable
                .hit(
                    r,
                    &Interval {
                        min: ray_t.min,
                        max: new_max,
                    },
                )
                .or(closest_so_far)
        })
    }

    fn bounding_box(&self) -> AABB {
        self.iter()
            .fold(AABB::empty(), |bbox, h| bbox.merge(&h.bounding_box()))
    }
}

impl<const N: usize, M, H: Hittable<Material = M>> Hittable for [H; N] {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        self.as_slice().hit(r, ray_t)
    }

    fn bounding_box(&self) -> AABB {
        self.as_slice().bounding_box()
    }
}

impl<M, H: Hittable<Material = M> + ?Sized> Hittable for &H {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        (*self).hit(r, ray_t)
    }

    fn bounding_box(&self) -> AABB {
        (*self).bounding_box()
    }
}

impl<M, H: Hittable<Material = M> + ?Sized> Hittable for Box<H> {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        (**self).hit(r, ray_t)
    }

    fn bounding_box(&self) -> AABB {
        (**self).bounding_box()
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    use super::*;

    mod calculate_face_normal {
        use float_cmp::assert_approx_eq;

        use super::*;

        #[test]
        fn ray_and_outward_normal_in_same_direction() {
            let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, -1.0));
            let outward_normal = NormalizedVec3::new(0.0, 1.0, 0.0);

            let (n, face) = calculate_face_normal(&ray, outward_normal.clone());

            assert_eq!(face, Face::Back);
            assert_approx_eq!(&Vec3, &-&outward_normal, &n);
        }

        #[test]
        fn ray_and_outward_normal_in_different_directions() {
            let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, -1.0));
            let outward_normal = NormalizedVec3::new(0.0, -1.0, 0.0);

            let (n, face) = calculate_face_normal(&ray, outward_normal.clone());

            assert_eq!(face, Face::Front);
            assert_approx_eq!(&Vec3, &outward_normal, &n);
        }
    }

    mod slice_hittable {
        use crate::{
            hittable::sphere::{Center, Sphere},
            material::Flat,
        };

        use super::*;

        #[test]
        fn hitting_nothing() {
            let sphere = Sphere {
                center: Center::Stationary(Point3::new(0.0, 0.0, -10.0)),
                radius: 1.0,
                material: Flat,
            };
            let arr = [sphere];
            let slice = arr.as_slice();

            let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 1.0, 0.0));

            let hit = slice.hit(&ray, &Interval::nonnegative());

            assert!(hit.is_none());
        }

        #[test]
        fn hitting_a_hittable() {
            let sphere = Sphere {
                center: Center::Stationary(Point3::new(0.0, 0.0, -10.0)),
                radius: 1.0,
                material: Flat,
            };
            let arr = [sphere];
            let slice = arr.as_slice();

            let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

            let hit = slice.hit(&ray, &Interval::nonnegative());

            assert_eq!(hit.map(|h| h.1.t), Some(14.0));
        }

        #[test]
        fn hitting_the_closest_hittable() {
            let back_sphere = Sphere {
                center: Center::Stationary(Point3::new(0.0, 0.0, -10.0)),
                radius: 1.0,
                material: Flat,
            };
            let middle_sphere = Sphere {
                center: Center::Stationary(Point3::new(0.0, 0.0, -5.0)),
                radius: 1.0,
                material: Flat,
            };
            let front_sphere = Sphere {
                center: Center::Stationary(Point3::new(0.0, 0.0, 0.0)),
                radius: 1.0,
                material: Flat,
            };
            let arr = [back_sphere, front_sphere, middle_sphere];
            let slice = arr.as_slice();

            let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

            let hit = slice.hit(&ray, &Interval::nonnegative());

            assert_eq!(hit.map(|h| h.1.t), Some(4.0));
        }
    }
}
