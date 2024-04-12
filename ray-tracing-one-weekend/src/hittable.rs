use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{NormalizedVec3, Point3},
};

#[derive(Debug, PartialEq)]
pub enum Face {
    Front,
    Back,
}

pub struct HitRecord<M> {
    pub p: Point3,
    pub normal: NormalizedVec3,
    pub t: f64,
    pub face: Face,
    pub material: M,
}

pub fn calculate_face_normal(r: &Ray, outward_normal: NormalizedVec3) -> (NormalizedVec3, Face) {
    if r.direction.dot(&outward_normal) <= 0.0 {
        (outward_normal, Face::Front)
    } else {
        (-&outward_normal, Face::Back)
    }
}

pub trait Hittable<M> {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<&M>>;
}

impl<M, H: Hittable<M>> Hittable<M> for &[H] {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<&M>> {
        self.iter().fold(None, |closest_so_far, current_hittable| {
            let new_max = closest_so_far
                .as_ref()
                .map_or(ray_t.max, |closest| closest.t);
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
        use std::marker::PhantomData;

        use rand::rngs::ThreadRng;

        use crate::{material::Flat, sphere::Sphere};

        use super::*;

        #[test]
        fn hitting_nothing() {
            let sphere = Sphere {
                center: Point3::new(0.0, 0.0, -10.0),
                radius: 1.0,
                material: Flat,
                phantom: PhantomData::<ThreadRng>::default(),
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
                center: Point3::new(0.0, 0.0, -10.0),
                radius: 1.0,
                material: Flat,
                phantom: PhantomData::<ThreadRng>::default(),
            };
            let arr = [sphere];
            let slice = arr.as_slice();

            let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

            let hit = slice.hit(&ray, &Interval::nonnegative());

            assert_eq!(hit.map(|h| h.t), Some(14.0));
        }

        #[test]
        fn hitting_the_closest_hittable() {
            let back_sphere = Sphere {
                center: Point3::new(0.0, 0.0, -10.0),
                radius: 1.0,
                material: Flat,
                phantom: PhantomData::<ThreadRng>::default(),
            };
            let middle_sphere = Sphere {
                center: Point3::new(0.0, 0.0, -5.0),
                radius: 1.0,
                material: Flat,
                phantom: PhantomData::<ThreadRng>::default(),
            };
            let front_sphere = Sphere {
                center: Point3::new(0.0, 0.0, 0.0),
                radius: 1.0,
                material: Flat,
                phantom: PhantomData::<ThreadRng>::default(),
            };
            let arr = [back_sphere, front_sphere, middle_sphere];
            let slice = arr.as_slice();

            let ray = Ray::new(Point3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));

            let hit = slice.hit(&ray, &Interval::nonnegative());

            assert_eq!(hit.map(|h| h.t), Some(4.0));
        }
    }
}
