use std::io::{self, BufWriter};

use rand::SeedableRng;

use rand_xoshiro::Xoshiro256PlusPlus;
use ray_tracing_one_weekend::{camera::Camera, hittable::Hittable, material::Material};

pub fn render_to_stdout<M: Material, H: Hittable<Material = M>>(world: &H, camera: &Camera) {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(68237004);
    let mut out = BufWriter::new(io::stdout().lock());
    camera.render(&mut rng, &world, &mut out).unwrap();
}
