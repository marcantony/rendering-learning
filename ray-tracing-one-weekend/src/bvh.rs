use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

/// A bounding-volume hierarchy for hittables.
pub struct Bvh<H> {
    children: Children<H>,
    bbox: AABB,
}

enum Children<H> {
    Leaf(Vec<H>),
    Inner(Vec<Bvh<H>>),
}

impl<M, H: Hittable<Material = M>> Bvh<H> {
    pub fn new(mut hs: Vec<H>) -> Self {
        if hs.is_empty() {
            panic!("Cannot make a BVH node without hittables.")
        }

        let (children, bbox) = if hs.len() == 1 {
            let child = hs.swap_remove(0);
            let b = child.bounding_box();
            (Children::Leaf(vec![child]), b)
        } else if hs.len() == 2 {
            let left = hs.swap_remove(0);
            let right = hs.swap_remove(0);
            let b = left.bounding_box().merge(&right.bounding_box());
            (Children::Leaf(vec![left, right]), b)
        } else {
            let bbox = hs
                .iter()
                .fold(AABB::empty(), |b, h| b.merge(&h.bounding_box()));
            let choice = find_longest_axis(&bbox);
            let key_fn = match choice {
                0 => |h: &H| h.bounding_box().x().min,
                1 => |h: &H| h.bounding_box().y().min,
                2 => |h: &H| h.bounding_box().z().min,
                _ => panic!("Sorting choice should be between 0 and 2"),
            };

            hs.sort_unstable_by(|l, r| key_fn(l).total_cmp(&key_fn(r)));

            let mid = hs.len() / 2;
            let rs = hs.split_off(mid);
            let ls = hs;

            let left = Bvh::new(ls);
            let right = Bvh::new(rs);
            (Children::Inner(vec![left, right]), bbox)
        };

        Bvh { children, bbox }
    }
}

fn find_longest_axis(bbox: &AABB) -> u8 {
    if bbox.x().size() > bbox.y().size() {
        if bbox.x().size() > bbox.z().size() {
            0
        } else {
            2
        }
    } else {
        if bbox.y().size() > bbox.z().size() {
            1
        } else {
            2
        }
    }
}

impl<M, H: Hittable<Material = M>> Hittable for Bvh<H> {
    type Material = M;
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<(&M, HitRecord)> {
        if self.bbox.hit(ray, ray_t) {
            match &self.children {
                Children::Leaf(c) => c.as_slice().hit(ray, ray_t),
                Children::Inner(c) => c.as_slice().hit(ray, ray_t),
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
