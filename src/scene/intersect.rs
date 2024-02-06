use crate::math::{point::Point3d, util, vector::NormalizedVec3d};

use super::{object::Object, ray::Ray};

const POINT_OFFSET_BIAS: f64 = 1e-5;

#[derive(Debug, Clone)]
pub struct Intersection<'a, T: Object + ?Sized> {
    t: f64,
    object: &'a T,
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

    pub fn prepare_computations(&self, ray: &Ray, xs: &[Intersection<T>]) -> Precomputation<T> {
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

        let over_point = &point + &(&*adjusted_normal_v * POINT_OFFSET_BIAS);
        let under_point = &point - &(&*adjusted_normal_v * POINT_OFFSET_BIAS);

        let reflect_v =
            NormalizedVec3d::try_from(ray.direction.reflect(&adjusted_normal_v)).unwrap();

        let mut containers = Vec::<&T>::new();
        let mut n1: f64 = 1.0;
        let mut n2: f64 = 1.0;
        for i in xs {
            if i == self {
                n1 = match containers.last() {
                    Some(o) => o.material().refractive_index,
                    None => 1.0,
                };
            }

            if let Some(index) = containers
                .iter()
                .position(|&obj| std::ptr::eq(obj, i.object()))
            {
                containers.remove(index);
            } else {
                containers.push(i.object());
            }

            if i == self {
                n2 = match containers.last() {
                    Some(o) => o.material().refractive_index,
                    None => 1.0,
                };
                break;
            }
        }

        Precomputation {
            t,
            object,
            point,
            eye_v,
            normal_v: adjusted_normal_v,
            inside,
            over_point,
            under_point,
            reflect_v,
            refraction_exiting: n1,
            refraction_entering: n2,
        }
    }
}

impl<'a, T: Object + ?Sized> PartialEq for Intersection<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        util::are_equal(self.t, other.t) && std::ptr::eq(self.object, other.object)
    }
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
    pub under_point: Point3d,
    pub reflect_v: NormalizedVec3d,
    pub refraction_exiting: f64,
    pub refraction_entering: f64,
}

impl<'a, T: Object + ?Sized> Precomputation<'a, T> {
    pub fn schlick(&self) -> f64 {
        let cos = self.eye_v.dot(&self.normal_v);
        let n = self.refraction_exiting / self.refraction_entering;
        let sin2_t = n * n * (1.0 - cos * cos);
        let cos_t = f64::sqrt(1.0 - sin2_t);

        let cos_adjusted = if n > 1.0 { cos_t } else { cos };

        if sin2_t > 1.0 && n > 1.0 {
            1.0
        } else {
            let r0 = ((self.refraction_exiting - self.refraction_entering)
                / (self.refraction_exiting + self.refraction_entering))
                .powi(2);
            r0 + (1.0 - r0) * (1.0 - cos_adjusted).powi(5)
        }
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
pub mod test_utils {
    use super::*;

    pub fn to_ts<'a, T: Object + ?Sized>(ts: Vec<Intersection<'a, T>>) -> Vec<f64> {
        ts.into_iter().map(|i| i.t()).collect()
    }
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
        use crate::{
            math::matrix::InvertibleMatrix,
            scene::{
                object::{plane::Plane, sphere},
                transformation,
            },
        };

        use super::*;

        #[test]
        fn precomputing_the_state_of_an_intersection() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let i = Intersection::new(4.0, &s);

            let comps = i.prepare_computations(&r, &vec![]);

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
        fn precomputing_the_reflection_vector() {
            let shape: Plane = Default::default();
            let t = std::f64::consts::SQRT_2 / 2.0;
            let r = Ray::new(Point3d::new(0.0, 1.0, -1.0), Vec3d::new(0.0, -t, t));
            let i = Intersection::new(std::f64::consts::SQRT_2, &shape);

            let comps = i.prepare_computations(&r, &vec![]);

            assert_eq!(*comps.reflect_v, Vec3d::new(0.0, t, t));
        }

        #[test]
        fn hit_when_an_intersection_occurs_on_the_outside() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let i = Intersection { t: 4.0, object: &s };

            let comps = i.prepare_computations(&r, &vec![]);

            assert_eq!(comps.inside, false);
        }

        #[test]
        fn hit_when_an_intersection_occurs_on_the_inside() {
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let i = Intersection { t: 1.0, object: &s };

            let comps = i.prepare_computations(&r, &vec![]);

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
        fn the_hit_should_offset_the_over_point() {
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

            let comps = i.prepare_computations(&r, &vec![]);

            assert!(comps.over_point.z() < -POINT_OFFSET_BIAS / 2.0);
            assert!(comps.point.z() > comps.over_point.z());
        }

        #[test]
        fn the_under_point_is_below_the_surface() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let mut shape = sphere::glass_sphere();
            shape.transform =
                InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, 1.0)).unwrap();
            let i = Intersection::new(5.0, &shape);
            let xs = vec![i];

            let comps = xs[0].prepare_computations(&r, &xs);

            assert!(comps.under_point.z() > POINT_OFFSET_BIAS / 2.0);
            assert!(comps.point.z() < comps.under_point.z());
        }

        mod refraction {
            use super::*;

            fn get_objects() -> (Sphere, Sphere, Sphere) {
                let mut a = sphere::glass_sphere();
                a.transform =
                    InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap();
                a.material.refractive_index = 1.5;

                let mut b = sphere::glass_sphere();
                b.transform =
                    InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, -0.25))
                        .unwrap();
                b.material.refractive_index = 2.0;

                let mut c = sphere::glass_sphere();
                c.transform =
                    InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, 0.25))
                        .unwrap();
                c.material.refractive_index = 2.5;

                (a, b, c)
            }

            fn get_xs<'a>(
                a: &'a Sphere,
                b: &'a Sphere,
                c: &'a Sphere,
            ) -> Vec<Intersection<'a, Sphere>> {
                vec![
                    (2.0, a),
                    (2.75, b),
                    (3.25, c),
                    (4.75, b),
                    (5.25, c),
                    (6.0, a),
                ]
                .into_iter()
                .map(|(t, o)| Intersection::new(t, o))
                .collect()
            }

            macro_rules! refractive_index_tests {
                ($($name:ident: $value:expr,)*) => {
                    $(
                        #[test]
                        fn $name() {
                            let (index, n1, n2) = $value;
                            let (a, b, c) = get_objects();
                            let xs = get_xs(&a, &b, &c);
                            let r = Ray::new(Point3d::new(0.0, 0.0, -4.0), Vec3d::new(0.0, 0.0, 1.0));
                            let comps = xs[index].prepare_computations(&r, &xs);

                            assert_eq!(comps.refraction_exiting, n1);
                            assert_eq!(comps.refraction_entering, n2);
                        }
                    )*
                };
            }

            refractive_index_tests! {
                refractive_index_0: (0, 1.0, 1.5),
                refractive_index_1: (1, 1.5, 2.0),
                refractive_index_2: (2, 2.0, 2.5),
                refractive_index_3: (3, 2.5, 2.5),
                refractive_index_4: (4, 2.5, 1.5),
                refractive_index_5: (5, 1.5, 1.0),
            }
        }

        mod schlick {
            use super::*;

            #[test]
            fn schlick_approximation_under_total_internal_reflection() {
                let shape = sphere::glass_sphere();
                let t = std::f64::consts::SQRT_2 / 2.0;
                let r = Ray::new(Point3d::new(0.0, 0.0, t), Vec3d::new(0.0, 1.0, 0.0));
                let xs = vec![Intersection::new(-t, &shape), Intersection::new(t, &shape)];

                let comps = xs[1].prepare_computations(&r, &xs);
                let reflectance = comps.schlick();

                assert_eq!(reflectance, 1.0);
            }

            #[test]
            fn schlick_approximation_with_a_perpendicular_viewing_angle() {
                let shape = sphere::glass_sphere();
                let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 1.0, 0.0));
                let xs = vec![
                    Intersection::new(-1.0, &shape),
                    Intersection::new(1.0, &shape),
                ];

                let comps = xs[1].prepare_computations(&r, &xs);
                let reflectance = comps.schlick();

                assert!(util::test_utils::are_within_tolerance(
                    reflectance,
                    0.04,
                    1e-2
                ));
            }

            #[test]
            fn schlick_approximation_with_a_small_angle_and_n2_greater_than_n1() {
                let shape = sphere::glass_sphere();
                let r = Ray::new(Point3d::new(0.0, 0.99, -2.0), Vec3d::new(0.0, 0.0, 1.0));
                let xs = vec![Intersection::new(1.8589, &shape)];

                let comps = xs[0].prepare_computations(&r, &xs);
                let reflectance = comps.schlick();

                assert!(util::test_utils::are_within_tolerance(
                    reflectance,
                    0.49010,
                    1e-5
                ));
            }
        }
    }
}
