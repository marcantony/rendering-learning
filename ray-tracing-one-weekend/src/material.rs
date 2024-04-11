use rand::Rng;

use crate::{
    color::Color,
    ray::Ray,
    vec3::{NormalizedVec3, Point3, Vec3},
};

pub trait Material {
    fn scatter(
        &mut self,
        ray: &Ray,
        normal: &NormalizedVec3,
        point: &Point3,
    ) -> Option<(Color, Ray)>;
}

impl Material for &mut dyn Material {
    fn scatter(
        &mut self,
        ray: &Ray,
        normal: &NormalizedVec3,
        point: &Point3,
    ) -> Option<(Color, Ray)> {
        (**self).scatter(ray, normal, point)
    }
}

pub struct Flat;

impl Material for Flat {
    fn scatter(
        &mut self,
        _ray: &Ray,
        _normal: &NormalizedVec3,
        _point: &Point3,
    ) -> Option<(Color, Ray)> {
        None
    }
}

pub struct Lambertian<R> {
    pub albedo: Color,
    pub rng: R,
}

impl<R: Rng> Material for Lambertian<R> {
    fn scatter(
        &mut self,
        _ray: &Ray,
        normal: &NormalizedVec3,
        point: &Point3,
    ) -> Option<(Color, Ray)> {
        let random_scatter_direction = &**normal + Vec3::random_unit_vector(&mut self.rng);
        let scatter_direction = if random_scatter_direction.near_zero() {
            // Catch degenerate scatter direction
            (**normal).clone()
        } else {
            random_scatter_direction
        };
        let scattered = Ray::new(point.clone(), scatter_direction);
        Some((self.albedo.clone(), scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(
        &mut self,
        ray: &Ray,
        normal: &NormalizedVec3,
        point: &Point3,
    ) -> Option<(Color, Ray)> {
        let reflected_direction = ray.direction.reflect(normal);
        let reflected_ray = Ray::new(point.clone(), reflected_direction);
        Some((self.albedo.clone(), reflected_ray))
    }
}
