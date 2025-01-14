use crate::{aabb::AABB, interval::Interval, material::Material, ray::Ray, vec3::Vec3};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Translate<H> {
    pub object: H,
    pub offset: Vec3,
}

impl<M: Material, H: Hittable<Material = M>> Hittable for Translate<H> {
    type Material = M;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        let offset_r = Ray::new_at_time(&r.origin - &self.offset, r.direction.clone(), r.time);

        self.object.hit(&offset_r, ray_t).map(|(m, mut hit_rec)| {
            hit_rec.p = hit_rec.p + &self.offset;
            (m, hit_rec)
        })
    }

    fn bounding_box(&self) -> AABB {
        &self.object.bounding_box() + &self.offset
    }
}
