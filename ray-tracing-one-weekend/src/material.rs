use rand::{Rng, RngCore};

use crate::{
    color::Color,
    hittable::Face,
    ray::Ray,
    vec3::{NormalizedVec3, Point3, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        normal: &NormalizedVec3,
        point: &Point3,
        face: &Face,
    ) -> Option<(Color, Ray)>;
}

impl<T: Material + ?Sized> Material for &T {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        normal: &NormalizedVec3,
        point: &Point3,
        face: &Face,
    ) -> Option<(Color, Ray)> {
        (**self).scatter(rng, ray, normal, point, face)
    }
}

impl<T: Material + ?Sized> Material for Box<T> {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        normal: &NormalizedVec3,
        point: &Point3,
        face: &Face,
    ) -> Option<(Color, Ray)> {
        (**self).scatter(rng, ray, normal, point, face)
    }
}

pub struct Flat;

impl Material for Flat {
    fn scatter(
        &self,
        _rng: &mut dyn RngCore,
        _ray: &Ray,
        _normal: &NormalizedVec3,
        _point: &Point3,
        _face: &Face,
    ) -> Option<(Color, Ray)> {
        None
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        _ray: &Ray,
        normal: &NormalizedVec3,
        point: &Point3,
        _face: &Face,
    ) -> Option<(Color, Ray)> {
        let random_scatter_direction = &**normal + Vec3::random_unit_vector(rng);
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
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        normal: &NormalizedVec3,
        point: &Point3,
        _face: &Face,
    ) -> Option<(Color, Ray)> {
        let reflected_direction = ray.direction.reflect(normal);
        let fuzzed_direction =
            reflected_direction.normalize() + (self.fuzz * Vec3::random_unit_vector(rng));
        let reflected_ray = Ray::new(point.clone(), fuzzed_direction);

        // Absorb fuzzed reflection if it scatters below the surface of the object
        if reflected_ray.direction.dot(normal) > 0.0 {
            Some((self.albedo.clone(), reflected_ray))
        } else {
            None
        }
    }
}

/// The incident index of refraction is always assumed to be 1.0, so
/// if this material describes an object embedded in a different transparent
/// material, then its index of refraction should be relative (the "true"
/// refractive index of this material divided by the refractive index of the
/// surrounding material).
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        normal: &NormalizedVec3,
        point: &Point3,
        face: &Face,
    ) -> Option<(Color, Ray)> {
        let refraction_index = match &face {
            Face::Front => 1.0 / self.refraction_index,
            Face::Back => self.refraction_index,
        };

        let unit_direction = NormalizedVec3::try_from(&ray.direction)
            .expect("How did the incident ray have magnitude 0?");

        let cos_theta = (-&unit_direction).dot(normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_index * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, refraction_index) > rng.gen() {
            unit_direction.reflect(normal)
        } else {
            unit_direction.refract(normal, refraction_index)
        };
        let scattered = Ray::new(point.clone(), direction);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}

/// Approximate reflectance of a dielectric using Schlick's approximation
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
