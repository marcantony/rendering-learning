use crate::{
    draw::color::Color,
    math::{point::Point3d, util, vector::NormalizedVec3d},
};

use super::{object::Object, ray::Ray};

const POINT_OFFSET_BIAS: f64 = 1e-5;

#[derive(Debug, Clone)]
pub struct Intersection<T, C, N> {
    t: f64,
    object: T,
    pub color: C,
    pub normal: N,
}

impl<T, C, N> Intersection<T, C, N> {
    pub fn new(t: f64, object: T, color: C, normal: N) -> Self {
        Intersection {
            t,
            object,
            color,
            normal,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &T {
        &self.object
    }
}

impl<'a, T: Object + ?Sized> Intersection<&'a T, Color, NormalizedVec3d> {
    pub fn prepare_computations(
        &self,
        ray: &Ray,
        xs: &[Intersection<&T, Color, NormalizedVec3d>],
    ) -> Precomputation<&T> {
        prepare_computations_helper(self, ray, self.normal.clone(), self.color.clone(), xs)
    }
}

fn prepare_computations_helper<'a, T: Object + ?Sized, C1, N1, C2, N2>(
    intersection: &'a Intersection<&T, C1, N1>,
    ray: &Ray,
    normal: NormalizedVec3d,
    color: Color,
    xs: &[Intersection<&T, C2, N2>],
) -> Precomputation<&'a T> {
    let t = intersection.t();
    let object = intersection.object();
    let point = ray.position(t);
    let eye_v = NormalizedVec3d::try_from(-&ray.direction).unwrap();
    let normal_v = normal;

    let normal_dot_eye = normal_v.dot(&eye_v);
    let (adjusted_normal_v, inside) = if normal_dot_eye < 0.0 {
        (-normal_v, true)
    } else {
        (normal_v, false)
    };

    let over_point = &point + &(&*adjusted_normal_v * POINT_OFFSET_BIAS);
    let under_point = &point - &(&*adjusted_normal_v * POINT_OFFSET_BIAS);

    let reflect_v = NormalizedVec3d::try_from(ray.direction.reflect(&adjusted_normal_v)).unwrap();

    let mut containers = Vec::<&T>::with_capacity(xs.len());
    let mut n1: f64 = 1.0;
    let mut n2: f64 = 1.0;
    for i in xs {
        if i == intersection {
            n1 = match containers.last() {
                Some(o) => o.material().refractive_index,
                None => 1.0,
            };
        }

        if let Some(index) = containers
            .iter()
            .position(|&obj| std::ptr::eq(obj, *i.object()))
        {
            containers.remove(index);
        } else {
            containers.push(i.object());
        }

        if i == intersection {
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
        object_color: color,
    }
}

impl<T: ?Sized, CL, NL, CR, NR> PartialEq<Intersection<&T, CR, NR>> for Intersection<&T, CL, NL> {
    fn eq(&self, other: &Intersection<&T, CR, NR>) -> bool {
        util::are_equal(self.t, other.t) && std::ptr::eq(self.object, other.object)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Precomputation<T> {
    pub t: f64,
    pub object: T,
    pub point: Point3d,
    pub eye_v: NormalizedVec3d,
    pub normal_v: NormalizedVec3d,
    pub inside: bool,
    pub over_point: Point3d,
    pub under_point: Point3d,
    pub reflect_v: NormalizedVec3d,
    pub refraction_exiting: f64,
    pub refraction_entering: f64,
    pub object_color: Color,
}

impl<T> Precomputation<T> {
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

pub fn hit<T, C, N>(intersections: &[Intersection<T, C, N>]) -> Option<&Intersection<T, C, N>> {
    intersections.iter().fold(None, |acc, i| {
        if i.t() >= 0.0 {
            acc.map(|lowest| if lowest.t() < i.t() { lowest } else { i })
                .or(Some(i))
        } else {
            acc
        }
    })
}

pub fn sort<T, C, N>(xs: &mut [Intersection<T, C, N>]) {
    xs.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap())
}

#[cfg(test)]
pub mod test_utils {
    use super::*;

    pub fn to_ts<T, C, N>(ts: &[Intersection<T, C, N>]) -> Vec<f64> {
        ts.into_iter().map(|i| i.t()).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        draw::color,
        math::{point::Point3d, vector::Vec3d},
        scene::object::sphere::Sphere,
    };

    use super::*;

    fn default_color() -> Color {
        color::black()
    }

    fn default_normal() -> NormalizedVec3d {
        NormalizedVec3d::new(1.0, 1.0, 1.0).unwrap()
    }

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::unit();

        let i = Intersection::new(3.5, &s, default_color(), default_normal());

        assert_eq!(i.t(), 3.5);
        assert!(std::ptr::eq(*i.object(), &s));
    }

    mod hit {
        use super::*;

        #[test]
        fn hit_when_all_intersections_have_positive_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(1.0, &s, default_color(), default_normal());
            let i2 = Intersection::new(2.0, &s, default_color(), default_normal());
            let xs = vec![i2, i1];

            let i = hit(&xs);

            assert!(std::ptr::eq(i.unwrap(), &xs[1]));
        }

        #[test]
        fn hit_when_some_intersections_have_negative_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(-1.0, &s, default_color(), default_normal());
            let i2 = Intersection::new(1.0, &s, default_color(), default_normal());
            let xs = vec![i2, i1];

            let i = hit(&xs);

            assert!(std::ptr::eq(i.unwrap(), &xs[0]));
        }

        #[test]
        fn hit_when_all_intersections_have_negative_t() {
            let s = Sphere::unit();
            let i1 = Intersection::new(-2.0, &s, default_color(), default_normal());
            let i2 = Intersection::new(-1.0, &s, default_color(), default_normal());
            let xs = vec![i2, i1];

            let i = hit(&xs);

            assert!(i.is_none());
        }

        #[test]
        fn hit_is_always_lowest_nonnegative_intersection() {
            let s = Sphere::unit();
            let i1 = Intersection::new(5.0, &s, default_color(), default_normal());
            let i2 = Intersection::new(7.0, &s, default_color(), default_normal());
            let i3 = Intersection::new(-3.0, &s, default_color(), default_normal());
            let i4 = Intersection::new(2.0, &s, default_color(), default_normal());
            let xs = vec![i1, i2, i3, i4];

            let i = hit(&xs);

            assert!(std::ptr::eq(i.unwrap(), &xs[3]));
        }
    }

    mod prepare_computations {
        use crate::{
            math::matrix::InvertibleMatrix,
            scene::{
                object::{
                    plane::Plane,
                    sphere::{self, glass_sphere},
                    transformed::Transformed,
                },
                transformation,
            },
        };

        use super::*;

        #[test]
        fn precomputing_the_state_of_an_intersection() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let is = s.intersect(&r);
            let i = &is[0];

            let comps = i.prepare_computations(&r, &vec![]);

            assert_eq!(comps.t, i.t());
            assert!(std::ptr::eq(comps.object, *i.object()));
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
            let is = shape.intersect(&r);

            let comps = is[0].prepare_computations(&r, &vec![]);

            assert_eq!(*comps.reflect_v, Vec3d::new(0.0, t, t));
        }

        #[test]
        fn hit_when_an_intersection_occurs_on_the_outside() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let is = s.intersect(&r);

            let comps = is[0].prepare_computations(&r, &vec![]);

            assert_eq!(comps.inside, false);
        }

        #[test]
        fn hit_when_an_intersection_occurs_on_the_inside() {
            let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 0.0, 1.0));
            let s: Sphere = Default::default();
            let is = s.intersect(&r);

            let comps = is[1].prepare_computations(&r, &vec![]);

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
            let shape = Transformed::new(
                Sphere::unit(),
                InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, 1.0)).unwrap(),
            );
            let is = shape.intersect(&r);
            let i = &is[0];

            let comps = i.prepare_computations(&r, &vec![]);

            assert!(comps.over_point.z() < -POINT_OFFSET_BIAS / 2.0);
            assert!(comps.point.z() > comps.over_point.z());
        }

        #[test]
        fn the_under_point_is_below_the_surface() {
            let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));
            let shape = Transformed::new(
                glass_sphere(),
                InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, 1.0)).unwrap(),
            );
            let xs = shape.intersect(&r);

            let comps = xs[0].prepare_computations(&r, &xs);

            assert!(comps.under_point.z() > POINT_OFFSET_BIAS / 2.0);
            assert!(comps.point.z() < comps.under_point.z());
        }

        mod refraction {
            use super::*;

            fn get_objects() -> (
                Transformed<Sphere>,
                Transformed<Sphere>,
                Transformed<Sphere>,
            ) {
                let mut a_s = sphere::glass_sphere();
                a_s.material.refractive_index = 1.5;
                let a = Transformed::new(
                    a_s,
                    InvertibleMatrix::try_from(transformation::scaling(2.0, 2.0, 2.0)).unwrap(),
                );

                let mut b_s = sphere::glass_sphere();
                b_s.material.refractive_index = 2.0;
                let b = Transformed::new(
                    b_s,
                    InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, -0.25))
                        .unwrap(),
                );

                let mut c_s = sphere::glass_sphere();
                c_s.material.refractive_index = 2.5;
                let c = Transformed::new(
                    c_s,
                    InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, 0.25))
                        .unwrap(),
                );

                (a, b, c)
            }

            fn get_xs<'a>(
                a: &'a Transformed<Sphere>,
                b: &'a Transformed<Sphere>,
                c: &'a Transformed<Sphere>,
            ) -> Vec<Intersection<&'a Transformed<Sphere>, Color, NormalizedVec3d>> {
                vec![
                    (2.0, a),
                    (2.75, b),
                    (3.25, c),
                    (4.75, b),
                    (5.25, c),
                    (6.0, a),
                ]
                .into_iter()
                .map(|(t, o)| Intersection::new(t, o, default_color(), default_normal()))
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
                let xs = shape.intersect(&r);

                let comps = xs[1].prepare_computations(&r, &xs);
                let reflectance = comps.schlick();

                assert_eq!(reflectance, 1.0);
            }

            #[test]
            fn schlick_approximation_with_a_perpendicular_viewing_angle() {
                let shape = sphere::glass_sphere();
                let r = Ray::new(Point3d::new(0.0, 0.0, 0.0), Vec3d::new(0.0, 1.0, 0.0));
                let xs = shape.intersect(&r);

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
                let xs = shape.intersect(&r);

                let comps = xs[0].prepare_computations(&r, &xs);
                let reflectance = comps.schlick();

                assert!(util::test_utils::are_within_tolerance(
                    reflectance,
                    0.49018,
                    1e-5
                ));
            }
        }
    }
}
