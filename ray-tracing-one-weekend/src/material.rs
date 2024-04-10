use rand::Rng;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&mut self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Flat;

impl Material for Flat {
    fn scatter(&mut self, _ray: &Ray, _record: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
}

pub struct Lambertian<R> {
    pub albedo: Color,
    pub rng: R,
}

impl<R: Rng> Material for Lambertian<R> {
    fn scatter(&mut self, _ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let random_scatter_direction = &*record.normal + Vec3::random_unit_vector(&mut self.rng);
        let scatter_direction = if random_scatter_direction.near_zero() {
            // Catch degenerate scatter direction
            (&*record.normal).clone()
        } else {
            random_scatter_direction
        };
        let scattered = Ray::new(record.p.clone(), scatter_direction);
        Some((self.albedo.clone(), scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&mut self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected_direction = ray.direction.reflect(&record.normal);
        let reflected_ray = Ray::new(record.p.clone(), reflected_direction);
        Some((self.albedo.clone(), reflected_ray))
    }
}
