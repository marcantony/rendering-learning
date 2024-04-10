use rand::Rng;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(&mut self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
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
