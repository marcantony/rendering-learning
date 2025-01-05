use rand::Rng;

use crate::vec3::Point3;

const BIT_COUNT: usize = 8;
const POINT_COUNT: usize = (2 as usize).pow(BIT_COUNT as u32);
const INDEX_BITMASK: i32 = POINT_COUNT as i32 - 1;

pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new(rand: &mut impl Rng) -> Self {
        let mut randfloat = [0f64; POINT_COUNT];
        rand.fill(&mut randfloat);

        let mut perm_x = std::array::from_fn(|i| i);
        permute(&mut perm_x, rand);

        let mut perm_y = std::array::from_fn(|i| i);
        permute(&mut perm_y, rand);

        let mut perm_z = std::array::from_fn(|i| i);
        permute(&mut perm_z, rand);

        Perlin {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0f64; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[self.perm_x
                        [((i + di as i32) & INDEX_BITMASK) as usize]
                        ^ self.perm_y[((j + dj as i32) & INDEX_BITMASK) as usize]
                        ^ self.perm_z[((k + dk as i32) & INDEX_BITMASK) as usize]];
                }
            }
        }

        trilinear_interp(&c, u, v, w)
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

fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accum = 0.0;
    for i in 0..2 as usize {
        for j in 0..2 as usize {
            for k in 0..2 as usize {
                let i_f = i as f64;
                let j_f = j as f64;
                let k_f = k as f64;
                accum += (i_f * u + (1.0 - i_f) * (1.0 - u))
                    * (j_f * v + (1.0 - j_f) * (1.0 - v))
                    * (k_f * w + (1.0 - k_f) * (1.0 - w))
                    * c[i][j][k]
            }
        }
    }
    accum
}
