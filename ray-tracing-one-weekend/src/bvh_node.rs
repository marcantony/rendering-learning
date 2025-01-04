use rand::Rng;

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
    pub fn new(mut hs: Vec<H>, rand: &mut impl Rng) -> Self {
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
            sort_hittables(&mut hs, rand);

            let mid = hs.len() / 2;
            let rs = hs.split_off(mid);
            let ls = hs;

            let left = Bvh::new(ls, rand);
            let right = Bvh::new(rs, rand);
            let b = left.bounding_box().merge(&right.bounding_box());
            (Children::Inner(vec![left, right]), b)
        };

        Bvh { children, bbox }
    }
}

fn sort_hittables<H: Hittable>(hs: &mut [H], rand: &mut impl Rng) {
    let choice = rand.gen_range(0..=2);

    let key_fn = match choice {
        0 => |h: &H| h.bounding_box().x.min,
        1 => |h: &H| h.bounding_box().y.min,
        2 => |h: &H| h.bounding_box().z.min,
        _ => panic!("Sorting choice should be between 0 and 2"),
    };

    hs.sort_unstable_by(|l, r| key_fn(l).total_cmp(&key_fn(r)));
}

impl<M, H: Hittable<Material = M>> Hittable for Bvh<H> {
    type Material = M;
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<(&M, HitRecord)> {
        if self.bbox.hit(ray, ray_t) {
            match &self.children {
                Children::Leaf(c) => hit_vec(c, ray, ray_t),
                Children::Inner(c) => hit_vec(c, ray, ray_t),
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

fn hit_vec<'a, M, H: Hittable<Material = M>>(
    hs: &'a Vec<H>,
    r: &Ray,
    ray_t: &Interval,
) -> Option<(&'a M, HitRecord)> {
    hs.iter().fold(None, |closest_so_far, current_hittable| {
        let new_max = closest_so_far
            .as_ref()
            .map_or(ray_t.max, |closest| closest.1.t);
        current_hittable
            .hit(
                r,
                &Interval {
                    min: ray_t.min,
                    max: new_max,
                },
            )
            .or(closest_so_far)
    })
}
