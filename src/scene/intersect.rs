use crate::math::{point::Point3d, util, vector::NormalizedVec3d};

use super::{object::Object, ray::Ray};

const SHADOW_BIAS: f64 = 1e-5;

#[derive(Debug, Clone)]
pub struct Intersection<'a, T: Object + ?Sized> {
    t: f64,
    object: &'a T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Precomputation<'a, T: Object + ?Sized> {
    pub t: f64,
    pub object: &'a T,
    pub point: Point3d,
    pub eye_v: NormalizedVec3d,
    pub normal_v: NormalizedVec3d,
    pub inside: bool,
    pub over_point: Point3d,
}

impl<'a, T: Object + ?Sized> Intersection<'a, T> {
    pub fn new(t: f64, object: &T) -> Intersection<T> {
        Intersection { t, object }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &T {
        self.object
    }

    pub fn prepare_computations(&self, ray: &Ray) -> Precomputation<T> {
        let t = self.t();
        let object = self.object();
        let point = ray.position(t);
        let eye_v = NormalizedVec3d::try_from(-&ray.direction).unwrap();
        let normal_v = object.normal_at(&point);

        let normal_dot_eye = normal_v.dot(&eye_v);
        let (adjusted_normal_v, inside) = if normal_dot_eye < 0.0 {
            (-normal_v, true)
        } else {
            (normal_v, false)
        };

        let over_point = &point + &(&*adjusted_normal_v * SHADOW_BIAS);

        Precomputation {
            t,
            object,
            point,
            eye_v,
            normal_v: adjusted_normal_v,
            inside,
            over_point,
        }
    }
}

impl<'a, T: Object + PartialEq + ?Sized> PartialEq for Intersection<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        util::are_equal(self.t, other.t) && std::ptr::eq(self.object, other.object)
    }
}

pub fn hit<'a, 'b, T: Object + ?Sized>(
    intersections: &'a [Intersection<'b, T>],
) -> Option<&'a Intersection<'b, T>> {
    intersections.iter().fold(None, |acc, i| {
        if i.t() >= 0.0 {
            acc.map(|lowest| if lowest.t() < i.t() { lowest } else { i })
                .or(Some(i))
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use crate::{
        math::{point::Point3d, vector::Vec3d},
        scene::object::sphere::Sphere,
    };

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::unit();

        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t(), 3.5);
        assert!(std::ptr::eq(i.object(), &s));
    }

    mod hit {
        use super::*;

        #[test]
        fn hit_when_all_intersections_have_positive_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(1.0, &s);
            let i2 = Intersection::new(2.0, &s);
            let xs = vec![i2, i1];

            let i = hit(&xs);

            assert!(std::ptr::eq(i.unwrap(), &xs[1]));
        }

        #[test]
        fn hit_when_some_intersections_have_negative_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(-1.0, &s);
            let i2 = Intersection::new(1.0, &s);
            let xs = vec![i2, i1];

            let i = hit(&xs);

            assert!(std::ptr::eq(i.unwrap(), &xs[0]));
        }

        #[test]
        fn hit_when_all_intersections_have_negative_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(-2.0, &s);
            let i2 = Intersection::new(-1.0, &s);
            let xs = vec![i2, i1];

            let i = hit(&xs);

            assert!(i.is_none());
        }

        #[test]
        fn hit_is_always_lowest_nonnegative_intersection() {
            let s = Sphere::unit();
            let i1 = Intersection::new(5.0, &s);
            let i2 = Intersection::new(7.0, &s);
            let i3 = Intersection::new(-3.0, &s);
            let i4 = Intersection::new(2.0, &s);
            let xs = vec![i1, i2, i3, i4];

            let i = hit(&xs);

            assert!(std::ptr::eq(i.unwrap(), &xs[3]));
        }
    }

    mod prepare_computations {
        use crate::{math::matrix::InvertibleMatrix, scene::transformation};

        use super::*;

        #[test]
        fn precomputing_the_state_of_an_intersection() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let i = Intersection::new(4.0, &s);

            let comps = i.prepare_computations(&r);

            assert_eq!(comps.t, i.t());
            assert!(std::ptr::eq(comps.object, i.object()));
            assert_eq!(comps.point, Point3d::new(0.0, 0.0, -1.0));
            assert_eq!(
                comps.eye_v,
                NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap()
            );
            assert_eq!(
                comps.normal_v,
                NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap()
            );
        }

        #[test]
        fn hit_when_an_intersection_occurs_on_the_outside() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let i = Intersection { t: 4.0, object: &s };

            let comps = i.prepare_computations(&r);

            assert_eq!(comps.inside, false);
        }

        #[test]
        fn hit_when_an_intersection_occurs_on_the_inside() {
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let i = Intersection { t: 1.0, object: &s };

            let comps = i.prepare_computations(&r);

            assert_eq!(comps.point, Point3d::new(0.0, 0.0, 1.0));
            assert_eq!(
                comps.eye_v,
                NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap()
            );
            assert_eq!(comps.inside, true);
            assert_eq!(
                comps.normal_v,
                NormalizedVec3d::try_from(Vec3d::new(0.0, 0.0, -1.0)).unwrap()
            );
        }

        #[test]
        fn the_hit_should_offset_the_point() {
            let r = Ray {
                origin: Point3d::new(0.0, 0.0, -5.0),
                direction: Vec3d::new(0.0, 0.0, 1.0),
            };
            let shape = Sphere {
                transform: InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, 1.0))
                    .unwrap(),
                ..Default::default()
            };
            let i = Intersection::new(5.0, &shape);

            let comps = i.prepare_computations(&r);

            assert!(comps.over_point.z() < -SHADOW_BIAS / 2.0);
            assert!(comps.point.z() > comps.over_point.z());
        }
    }
}
