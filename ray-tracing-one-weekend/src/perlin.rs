use rand::Rng;

use crate::vec3::{Point3, Vec3};

const BIT_COUNT: usize = 8;
const POINT_COUNT: usize = (2 as usize).pow(BIT_COUNT as u32);
const INDEX_BITMASK: i32 = POINT_COUNT as i32 - 1;

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new(rand: &mut impl Rng) -> Self {
        let randvec = std::array::from_fn(|_i| Vec3::random_unit_vector(rand));

        let mut perm_x = std::array::from_fn(|i| i);
        permute(&mut perm_x, rand);

        let mut perm_y = std::array::from_fn(|i| i);
        permute(&mut perm_y, rand);

        let mut perm_z = std::array::from_fn(|i| i);
        permute(&mut perm_z, rand);

        Perlin {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    /// Outputs a noise value for the given point in the range [-1, 1]
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        // This array initialization could probably use some cleaning up
        let vec = Vec3::zero();
        let vecs = std::array::from_fn(|_i| &vec);
        let vecs3 = std::array::from_fn(|_i| vecs.clone());
        let mut c: [[[&Vec3; 2]; 2]; 2] = std::array::from_fn(|_i| vecs3.clone());

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = &self.randvec[self.perm_x
                        [((i + di as i32) & INDEX_BITMASK) as usize]
                        ^ self.perm_y[((j + dj as i32) & INDEX_BITMASK) as usize]
                        ^ self.perm_z[((k + dk as i32) & INDEX_BITMASK) as usize]];
                }
            }
        }

        perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }
}

fn permute(p: &mut [usize], rand: &mut impl Rng) {
    for i in (1..=(p.len() - 1)).rev() {
        let target = rand.gen_range(0..i);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn perlin_interp(c: &[[[&Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;

    for i in 0..2 as usize {
        for j in 0..2 as usize {
            for k in 0..2 as usize {
                let i_f = i as f64;
                let j_f = j as f64;
                let k_f = k as f64;

                let weight_v = Vec3::new(u - i_f, v - j_f, w - k_f);

                accum += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                    * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                    * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                    * c[i][j][k].dot(&weight_v)
            }
        }
    }
    accum
}
