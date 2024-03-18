use crate::{
    draw::color::Color,
    math::{point::Point3d, vector::NormalizedVec3d},
};

use self::bounded::Bounds;

use super::{intersect::Intersection, material::Material, ray::Ray};

pub trait Object {
    fn material(&self) -> &Material;
    fn intersect(&self, ray: &Ray)
        -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>>;
    fn bounds(&self) -> Bounds;
}

trait PhysicalObject: Object {
    fn normal_at(&self, point: &Point3d) -> NormalizedVec3d;
}

fn build_basic_intersection<'a, T: PhysicalObject>(
    ray: &Ray,
    t: f64,
    object: &'a T,
) -> Intersection<&'a dyn Object, Color, NormalizedVec3d> {
    let p = ray.position(t);
    Intersection::new(
        t,
        object,
        object.material().surface.color_at(&p),
        object.normal_at(&p),
    )
}

pub mod bounded;
pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod group;
pub mod impls;
pub mod plane;
pub mod sphere;
pub mod transformed;

#[cfg(test)]
pub mod test_utils {
    use crate::math::{point::Point3d, vector::NormalizedVec3d};

    use super::*;

    #[derive(Default)]
    pub struct MockObject {
        pub intersect_local_arg_expectation: Option<Ray>,
        pub material: Material,
        pub bounds: Bounds,
    }

    impl PhysicalObject for MockObject {
        fn normal_at(&self, object_point: &Point3d) -> NormalizedVec3d {
            NormalizedVec3d::new(object_point.x(), object_point.y(), object_point.z()).unwrap()
        }
    }

    impl Object for MockObject {
        fn material(&self) -> &Material {
            &self.material
        }

        fn intersect(
            &self,
            object_ray: &Ray,
        ) -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>> {
            if let Some(expected) = self.intersect_local_arg_expectation.as_ref() {
                assert_eq!(object_ray, expected);
            }
            let t = 1.0;
            let color = self.material().surface.color_at(&object_ray.origin);
            let normal = self.normal_at(&object_ray.origin);
            vec![Intersection::new(t, self, color, normal)]
        }

        fn bounds(&self) -> Bounds {
            self.bounds.clone()
        }
    }
}
