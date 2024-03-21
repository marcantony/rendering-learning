use std::fmt::Debug;

use crate::{
    draw::color::Color,
    math::{
        point::Point3d,
        vector::{NormalizedVec3d, Vec3d},
    },
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::{bounded::Bounds, Object};

const EPSILON: f64 = 1e-8;

#[derive(Debug, PartialEq, Clone)]
enum TriangleNormal {
    Flat(NormalizedVec3d),
    Smooth([Vec3d; 3]),
}

pub struct Triangle {
    points: [Point3d; 3],
    edges: [Vec3d; 2],
    normal: TriangleNormal,
    material: Material,
}

impl Triangle {
    pub fn flat(points: [Point3d; 3], material: Material) -> Self {
        let e1 = &points[1] - &points[0];
        let e2 = &points[2] - &points[0];
        let normal = NormalizedVec3d::try_from(e2.cross(&e1)).unwrap();

        Triangle {
            points,
            edges: [e1, e2],
            normal: TriangleNormal::Flat(normal),
            material,
        }
    }

    pub fn smooth(vertices: [(Point3d, Vec3d); 3], material: Material) -> Self {
        let [(p1, v1), (p2, v2), (p3, v3)] = vertices;

        let e1 = &p2 - &p1;
        let e2 = &p3 - &p1;

        Triangle {
            points: [p1, p2, p3],
            edges: [e1, e2],
            normal: TriangleNormal::Smooth([v1, v2, v3]),
            material,
        }
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

                    let p = ray.position(t);
                    let color = self.material().surface.color_at(&p);
                    let normal = match &self.normal {
                        TriangleNormal::Flat(n) => n.clone(),
                        TriangleNormal::Smooth([v1, v2, v3]) => NormalizedVec3d::try_from(
                            &(&(v2 * u) + &(v3 * v)) + &(v1 * (1.0 - u - v)),
                        )
                        .unwrap(),
                    };

                    let i = Intersection::new(t, self as &dyn Object, color, normal);

                    vec![i]
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

    mod flat {
        use super::*;

        fn test_triangle() -> Triangle {
            Triangle::flat(
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

            let triangle = Triangle::flat(points.clone(), Default::default());

            assert_eq!(triangle.points, points);
            assert_eq!(
                triangle.edges,
                [Vec3d::new(-1.0, -1.0, 0.0), Vec3d::new(1.0, -1.0, 0.0)]
            );
            assert_eq!(
                triangle.normal,
                TriangleNormal::Flat(NormalizedVec3d::new(0.0, 0.0, -1.0).unwrap())
            )
        }

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
                let n = NormalizedVec3d::new(0.0, 0.0, -1.0).unwrap();
                let c = t.material().surface.color_at(&p);
                assert_eq!(x.normal, n);
                assert_eq!(x.color, c);
            }
        }
    }

    mod smooth {
        use crate::math::vector;

        use super::*;

        #[test]
        fn normal_vector_of_a_smooth_triangle_interpolates_vertex_normals() {
            let t = Triangle::smooth(
                [
                    (Point3d::new(0.0, 1.0, 0.0), Vec3d::new(0.0, 1.0, 0.0)),
                    (Point3d::new(-1.0, 0.0, 0.0), Vec3d::new(-1.0, 0.0, 0.0)),
                    (Point3d::new(1.0, 0.0, 0.0), Vec3d::new(1.0, 0.0, 0.0)),
                ],
                Default::default(),
            );
            let r = Ray::new(Point3d::new(-0.2, 0.3, -2.0), Vec3d::new(0.0, 0.0, 1.0));

            let is = t.intersect(&r);

            vector::test_utils::assert_vec_approx_equals(
                &is[0].normal,
                &Vec3d::new(-0.55470, 0.83205, 0.0),
            )
        }
    }

    mod bounds {
        use super::*;

        #[test]
        fn bounds_of_a_triangle() {
            let t = Triangle::flat(
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
