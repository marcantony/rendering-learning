use crate::{aabb::AABB, interval::Interval, material::Material, ray::Ray, vec3::NormalizedVec3};

use super::{Face, HitRecord, Hittable};

/// A hittable for which rays have a random chance to intersect, proportional to the density,
/// at any point in the medium. Only intended to be used with Isotropic material.
#[derive(Clone)]
pub struct ConstantMedium<M, H> {
    boundary: H,
    neg_inv_density: f64,
    phase_function: M,
}

impl<M, H> ConstantMedium<M, H> {
    pub fn new(boundary: H, density: f64, material: M) -> Self {
        ConstantMedium {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: material,
        }
    }
}

impl<MOUT: Material, M: Material, H: Hittable<Material = M>> Hittable for ConstantMedium<MOUT, H> {
    type Material = MOUT;

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<(&Self::Material, HitRecord)> {
        self.boundary
            .hit(r, &Interval::universe())
            .and_then(|(_, rec1)| {
                self.boundary
                    .hit(
                        r,
                        &Interval {
                            min: rec1.t + 1e-4,
                            max: f64::INFINITY,
                        },
                    )
                    .map(|(_, rec2)| (rec1, rec2))
            })
            .and_then(|(mut rec1, mut rec2)| {
                rec1.t = rec1.t.max(ray_t.min);
                rec2.t = rec2.t.min(ray_t.max);

                if rec1.t >= rec2.t {
                    None
                } else {
                    rec1.t = rec1.t.max(0.0);

                    let ray_length = r.direction.length();
                    let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                    /* Using global RNG source instead of passing an RNG source through the struct/method because
                    I'm near the end of the book and don't care enough to rework the Hittable trait. This breaks
                    deterministic/repeatable renders. TODO: fix it one day if I care enough. */
                    let hit_distance = self.neg_inv_density * rand::random::<f64>().ln();

                    if hit_distance > distance_inside_boundary {
                        None
                    } else {
                        let t = rec1.t + hit_distance / ray_length;
                        let p = r.at(t);

                        let hitrecord = HitRecord {
                            p,
                            normal: NormalizedVec3::new(1.0, 0.0, 0.0), // arbitrary
                            t,
                            uv: (0.0, 0.0),    // arbitrary
                            face: Face::Front, // arbitrary
                        };

                        Some((&self.phase_function, hitrecord))
                    }
                }
            })
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
