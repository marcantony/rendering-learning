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
        let i = (4.0 * p.x()) as i32 & INDEX_BITMASK;
        let j = (4.0 * p.y()) as i32 & INDEX_BITMASK;
        let k = (4.0 * p.z()) as i32 & INDEX_BITMASK;

        self.randfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
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
