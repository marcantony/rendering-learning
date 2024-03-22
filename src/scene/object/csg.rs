use crate::{
    draw::color::Color,
    math::vector::NormalizedVec3d,
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::{bounded::Bounds, Object};

pub enum CsgOperation {
    Union,
    Intersection,
    Difference,
}

impl CsgOperation {
    fn intersection_allowed(&self, is_left_hit: bool, is_in_left: bool, is_in_right: bool) -> bool {
        match self {
            CsgOperation::Union => (is_left_hit && !is_in_right) || (!is_left_hit && !is_in_left),
            CsgOperation::Intersection => {
                (is_left_hit && is_in_right) || (!is_left_hit && is_in_left)
            }
            CsgOperation::Difference => {
                (is_left_hit && !is_in_right) || (!is_left_hit && is_in_left)
            }
        }
    }
}

pub struct Csg<T> {
    pub left: T,
    pub right: T,
    pub operation: CsgOperation,
}

struct SidedIntersection<T, C, N> {
    pub i: Intersection<T, C, N>,
    pub side: Side,
}
enum Side {
    Left,
    Right,
}

impl<T> Csg<T> {
    fn filter_intersections<U, C, N>(
        &self,
        xs: Vec<SidedIntersection<U, C, N>>,
    ) -> Vec<Intersection<U, C, N>> {
        let mut is_in_left = false;
        let mut is_in_right = false;
        xs.into_iter()
            .filter_map(|x| {
                let is_left_hit = match x.side {
                    Side::Left => true,
                    Side::Right => false,
                };

                let is_allowed =
                    self.operation
                        .intersection_allowed(is_left_hit, is_in_left, is_in_right);

                if is_left_hit {
                    is_in_left = !is_in_left;
                } else {
                    is_in_right = !is_in_right;
                };

                if is_allowed {
                    Some(x.i)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<T: Object> Object for Csg<T> {
    fn material(&self) -> &Material {
        unimplemented!()
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>> {
        let left_xs = self
            .left
            .intersect(&ray)
            .into_iter()
            .map(|i| SidedIntersection {
                i,
                side: Side::Left,
            })
            .collect::<Vec<_>>();
        let mut right_xs = self
            .right
            .intersect(&ray)
            .into_iter()
            .map(|i| SidedIntersection {
                i,
                side: Side::Right,
            })
            .collect::<Vec<_>>();

        let mut all_xs = left_xs;
        all_xs.append(&mut right_xs);
        all_xs.sort_by(|a, b| a.i.t().partial_cmp(&b.i.t()).unwrap());

        self.filter_intersections(all_xs)
    }

    fn bounds(&self) -> Bounds {
        Bounds::from_bounds(&[self.left.bounds(), self.right.bounds()])
    }
}

#[cfg(test)]
mod operation_tests {
    use super::*;

    macro_rules! op_tests {
        ($($name:ident: $args:expr, $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    let (operation, is_left_hit, is_in_left, is_in_right) = $args;
                    let result = operation.intersection_allowed(is_left_hit, is_in_left, is_in_right);
                    assert_eq!(result, $expected);
                }
            )*
        };
    }

    op_tests! {
        union1: (CsgOperation::Union, true, true, true), false,
        union2: (CsgOperation::Union, true, true, false), true,
        union3: (CsgOperation::Union, true, false, true), false,
        union4: (CsgOperation::Union, true, false, false), true,
        union5: (CsgOperation::Union, false, true, true), false,
        union6: (CsgOperation::Union, false, true, false), false,
        union7: (CsgOperation::Union, false, false, true), true,
        union8: (CsgOperation::Union, false, false, false), true,

        intersection1: (CsgOperation::Intersection, true, true, true), true,
        intersection2: (CsgOperation::Intersection, true, true, false), false,
        intersection3: (CsgOperation::Intersection, true, false, true), true,
        intersection4: (CsgOperation::Intersection, true, false, false), false,
        intersection5: (CsgOperation::Intersection, false, true, true), true,
        intersection6: (CsgOperation::Intersection, false, true, false), true,
        intersection7: (CsgOperation::Intersection, false, false, true), false,
        intersection8: (CsgOperation::Intersection, false, false, false), false,

        difference1: (CsgOperation::Difference, true, true, true), false,
        difference2: (CsgOperation::Difference, true, true, false), true,
        difference3: (CsgOperation::Difference, true, false, true), false,
        difference4: (CsgOperation::Difference, true, false, false), true,
        difference5: (CsgOperation::Difference, false, true, true), true,
        difference6: (CsgOperation::Difference, false, true, false), true,
        difference7: (CsgOperation::Difference, false, false, true), false,
        difference8: (CsgOperation::Difference, false, false, false), false
    }
}

#[cfg(test)]
mod csg_tests {
    use crate::{
        math::{matrix::InvertibleMatrix, point::Point3d, vector::Vec3d},
        scene::{
            object::{cube::Cube, sphere::Sphere, transformed::Transformed},
            transformation,
        },
    };

    use super::*;

    #[test]
    fn a_ray_misses_a_csg_object() {
        let c = Csg::<Box<dyn Object>> {
            left: Box::new(Sphere::default()),
            right: Box::new(Cube::default()),
            operation: CsgOperation::Union,
        };
        let r = Ray::new(Point3d::new(0.0, 2.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));

        let xs = c.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_hits_a_csg_object() {
        let c = Csg::<Box<dyn Object>> {
            left: Box::new(Sphere::default()),
            right: Box::new(Transformed::new(
                Sphere::default(),
                InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, 0.5)).unwrap(),
            )),
            operation: CsgOperation::Union,
        };
        let r = Ray::new(Point3d::new(0.0, 0.0, -5.0), Vec3d::new(0.0, 0.0, 1.0));

        let xs = c.intersect(&r);

        let left_intersections = c.left.intersect(&r);
        let right_intersections = c.right.intersect(&r);

        assert_eq!(xs.len(), 2);

        assert!(xs[0] == left_intersections[0]);
        assert_eq!(xs[0].color, left_intersections[0].color);
        assert_eq!(xs[0].normal, left_intersections[0].normal);

        assert!(xs[1] == right_intersections[1]);
        assert_eq!(xs[1].color, right_intersections[1].color);
        assert_eq!(xs[1].normal, right_intersections[1].normal);
    }

    #[test]
    fn bounds_cover_both_children() {
        let c = Csg::<Box<dyn Object>> {
            left: Box::new(Sphere::default()),
            right: Box::new(Transformed::new(
                Sphere::default(),
                InvertibleMatrix::try_from(transformation::translation(0.0, 0.0, 0.5)).unwrap(),
            )),
            operation: CsgOperation::Union,
        };

        assert_eq!(
            c.bounds(),
            Bounds {
                minimum: Point3d::new(-1.0, -1.0, -1.0),
                maximum: Point3d::new(1.0, 1.0, 1.5)
            }
        )
    }
}
