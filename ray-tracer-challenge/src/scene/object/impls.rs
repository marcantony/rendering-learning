use crate::{
    draw::color::Color,
    math::vector::NormalizedVec3d,
    scene::{intersect::Intersection, material::Material, ray::Ray},
};

use super::{bounded::Bounds, Object};

impl<T: Object + ?Sized> Object for Box<T> {
    fn material(&self) -> &Material {
        (**self).material()
    }

    fn intersect(
        &self,
        object_ray: &Ray,
    ) -> Vec<Intersection<&dyn Object, Color, NormalizedVec3d>> {
        (**self).intersect(object_ray)
    }

    fn bounds(&self) -> Bounds {
        (**self).bounds()
    }
}
