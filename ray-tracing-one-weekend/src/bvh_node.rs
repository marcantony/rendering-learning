use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct BvhNode<H> {
    left: H,
    right: H,
    bbox: AABB,
}

impl<M, H: Hittable<Material = M>> Hittable for BvhNode<H> {
    type Material = M;
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&M, HitRecord)> {
        if self.bbox.hit(r, ray_t) {
            self.left.hit(r, ray_t).or_else(|| self.right.hit(r, ray_t))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
