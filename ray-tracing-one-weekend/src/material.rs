use rand::{Rng, RngCore};

use crate::{
    color::Color,
    hittable::{Face, HitRecord},
    ray::Ray,
    texture::Texture,
    vec3::{NormalizedVec3, Point3, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        hitrecord: &HitRecord,
    ) -> Option<(Color, Ray)>;

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color;
}

impl<T: Material + ?Sized> Material for &T {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        hitrecord: &HitRecord,
    ) -> Option<(Color, Ray)> {
        (**self).scatter(rng, ray, hitrecord)
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        (**self).emitted(u, v, p)
    }
}

impl<T: Material + ?Sized> Material for Box<T> {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        hitrecord: &HitRecord,
    ) -> Option<(Color, Ray)> {
        (**self).scatter(rng, ray, hitrecord)
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        (**self).emitted(u, v, p)
    }
}

pub struct Flat;

impl Material for Flat {
    fn scatter(
        &self,
        _rng: &mut dyn RngCore,
        _ray: &Ray,
        _hitrecord: &HitRecord,
    ) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian<T> {
    pub texture: T,
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(
        &self,
        rng: &mut dyn RngCore,
        ray: &Ray,
        hitrecord: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let random_scatter_direction = &*hitrecord.normal + Vec3::random_unit_vector(rng);
        let scatter_direction = if random_scatter_direction.near_zero() {
            // Catch degenerate scatter direction
            (*hitrecord.normal).clone()
        } else {
            random_scatter_direction
        };
        let scattered = Ray::new_at_time(hitrecord.p.clone(), scatter_direction, ray.time);
        let attenuation = self
            .texture
            .value(hitrecord.uv.0, hitrecord.uv.1, &hitrecord.p);
        Some((attenuation, scattered))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
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
        hitrecord: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let reflected_direction = ray.direction.reflect(&hitrecord.normal);
        let fuzzed_direction =
            reflected_direction.normalize() + (self.fuzz * Vec3::random_unit_vector(rng));
        let reflected_ray = Ray::new_at_time(hitrecord.p.clone(), fuzzed_direction, ray.time);

        // Absorb fuzzed reflection if it scatters below the surface of the object
        if reflected_ray.direction.dot(&hitrecord.normal) > 0.0 {
            Some((self.albedo.clone(), reflected_ray))
        } else {
            None
        }
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
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
        hitrecord: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let refraction_index = match &hitrecord.face {
            Face::Front => 1.0 / self.refraction_index,
            Face::Back => self.refraction_index,
        };

        let unit_direction = NormalizedVec3::try_from(&ray.direction)
            .expect("How did the incident ray have magnitude 0?");

        let cos_theta = (-&unit_direction).dot(&hitrecord.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_index * sin_theta > 1.0;

        let direction = if cannot_refract || reflectance(cos_theta, refraction_index) > rng.gen() {
            unit_direction.reflect(&hitrecord.normal)
        } else {
            unit_direction.refract(&hitrecord.normal, refraction_index)
        };
        let scattered = Ray::new_at_time(hitrecord.p.clone(), direction, ray.time);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

/// Approximate reflectance of a dielectric using Schlick's approximation
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct DiffuseLight<T> {
    pub tex: T,
}

impl<T: Texture> Material for DiffuseLight<T> {
    fn scatter(
        &self,
        _rng: &mut dyn RngCore,
        _ray: &Ray,
        _hitrecord: &HitRecord,
    ) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}
