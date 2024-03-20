use std::fmt::Debug;

use crate::{
    draw::color::Color,
    math::{
        point::Point3d,
        vector::{NormalizedVec3d, Vec3d},
    },
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::{bounded::Bounds, Object, PhysicalObject};

const EPSILON: f64 = 1e-8;

pub struct Triangle {
    points: [Point3d; 3],
    edges: [Vec3d; 2],
    normal: NormalizedVec3d,
    material: Material,
}

impl Triangle {
    pub fn new(points: [Point3d; 3], material: Material) -> Self {
        let e1 = &points[1] - &points[0];
        let e2 = &points[2] - &points[0];
        let normal = NormalizedVec3d::try_from(e2.cross(&e1)).unwrap();

        Triangle {
            points,
            edges: [e1, e2],
            normal,
            material,
        }
    }
}

impl PhysicalObject for Triangle {
    fn normal_at(&self, _: &Point3d) -> NormalizedVec3d {
        self.normal.clone()
    }
}

impl Object for Triangle {
    fn material(&self) -> &Material {
        &self.material
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>> {
        let dir_cross_e2 = ray.direction.cross(&self.edges[1]);
        let det = self.edges[0].dot(&dir_cross_e2);

        if det.abs() < EPSILON {
            vec![]
        } else {
            let f = 1.0 / det;
            let p1_to_origin = &ray.origin - &self.points[0];
            let u = f * p1_to_origin.dot(&dir_cross_e2);

            if u < 0.0 || u > 1.0 {
                vec![]
            } else {
                let origin_cross_e1 = p1_to_origin.cross(&self.edges[0]);
                let v = f * ray.direction.dot(&origin_cross_e1);

                if v < 0.0 || (u + v) > 1.0 {
                    vec![]
                } else {
                    let t = f * self.edges[1].dot(&origin_cross_e1);
                    vec![super::build_basic_intersection(ray, t, self)]
                }
            }
        }
    }

    fn bounds(&self) -> Bounds {
        Bounds::from_points(&self.points).expect("triangle should have points")
    }
}

impl Debug for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Triangle")
            .field("points", &self.points)
            .field("edges", &self.edges)
            .field("normal", &self.normal)
            .field("material", &format!("{:p}", &self.material))
            .finish()
    }
}

#[cfg(test)]
mod test_utils {
    use super::*;

    impl PartialEq for Triangle {
        fn eq(&self, other: &Self) -> bool {
            self.points.iter().all(|p| other.points.contains(p))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_triangle() -> Triangle {
        Triangle::new(
            [
                Point3d::new(0.0, 1.0, 0.0),
                Point3d::new(-1.0, 0.0, 0.0),
                Point3d::new(1.0, 0.0, 0.0),
            ],
            Default::default(),
        )
    }

    #[test]
    fn constructing_a_triangle() {
        let points = [
            Point3d::new(0.0, 1.0, 0.0),
            Point3d::new(-1.0, 0.0, 0.0),
            Point3d::new(1.0, 0.0, 0.0),
        ];

        let triangle = Triangle::new(points.clone(), Default::default());

        assert_eq!(triangle.points, points);
        assert_eq!(
            triangle.edges,
            [Vec3d::new(-1.0, -1.0, 0.0), Vec3d::new(1.0, -1.0, 0.0)]
        );
        assert_eq!(
            triangle.normal,
            NormalizedVec3d::new(0.0, 0.0, -1.0).unwrap()
        )
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = test_triangle();

        assert_eq!(t.normal, NormalizedVec3d::new(0.0, 0.0, -1.0).unwrap());
        assert_eq!(t.normal_at(&Point3d::new(0.0, 0.5, 0.0)), t.normal);
        assert_eq!(t.normal_at(&Point3d::new(-0.5, 0.75, 0.0)), t.normal);
        assert_eq!(t.normal_at(&Point3d::new(0.5, 0.25, 0.0)), t.normal);
    }

    mod intersect {
        use super::*;

        #[test]
        fn intersecting_a_ray_parallel_to_the_triangle() {
            let t = test_triangle();
            let r = Ray::new(Point3d::new(0.0, -1.0, -2.0), Vec3d::new(0.0, 1.0, 0.0));

            let xs = t.intersect(&r);

            assert!(xs.is_empty());
        }

        #[test]
        fn a_ray_misses_the_p1_p3_edge() {
            let t = test_triangle();
            let r = Ray::new(Point3d::new(1.0, 1.0, -2.0), Vec3d::new(0.0, 0.0, 1.0));

            let xs = t.intersect(&r);

            assert!(xs.is_empty());
        }

        #[test]
        fn a_ray_misses_the_p1_p2_edge() {
            let t = test_triangle();
            let r = Ray::new(Point3d::new(-1.0, 1.0, -2.0), Vec3d::new(0.0, 0.0, 1.0));

            let xs = t.intersect(&r);

            assert!(xs.is_empty());
        }

        #[test]
        fn a_ray_misses_the_p2_p3_edge() {
            let t = test_triangle();
            let r = Ray::new(Point3d::new(0.0, -1.0, -2.0), Vec3d::new(0.0, 0.0, 1.0));

            let xs = t.intersect(&r);

            assert!(xs.is_empty());
        }

        #[test]
        fn a_ray_strikes_a_triangle() {
            let t = test_triangle();
            let r = Ray::new(Point3d::new(0.0, 0.5, -2.0), Vec3d::new(0.0, 0.0, 1.0));

            let xs = t.intersect(&r);

            assert_eq!(xs.len(), 1);
            assert_eq!(xs[0].t(), 2.0);
            for x in xs {
                let p = r.position(x.t());
                let n = t.normal_at(&p);
                let c = t.material().surface.color_at(&p);
                assert_eq!(x.normal, n);
                assert_eq!(x.color, c);
            }
        }
    }

    mod bounds {
        use super::*;

        #[test]
        fn bounds_of_a_triangle() {
            let t = Triangle::new(
                [
                    Point3d::new(0.0, 0.0, 0.0),
                    Point3d::new(0.0, 1.0, 0.0),
                    Point3d::new(1.0, 0.0, 1.0),
                ],
                Default::default(),
            );

            assert_eq!(
                t.bounds(),
                Bounds {
                    minimum: Point3d::new(0.0, 0.0, 0.0),
                    maximum: Point3d::new(1.0, 1.0, 1.0)
                }
            );
        }
    }
}
