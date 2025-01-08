use std::io::{self, BufWriter};

use rand::SeedableRng;

use rand_xoshiro::Xoshiro256PlusPlus;
use ray_tracing_one_weekend::{
    camera::Camera,
    hittable::{quad::Quad, Hittable},
    material::Material,
    vec3::{Point3, Vec3},
};

pub fn render_to_stdout<M: Material, H: Hittable<Material = M>>(world: &H, camera: &Camera) {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(68237004);
    let mut out = BufWriter::new(io::stdout().lock());
    camera.render(&mut rng, &world, &mut out).unwrap();
}

/// Returns the 3D box (six sides) that contains the two opposite vertices a & b
#[allow(dead_code)] // Silence warnings because this isn't used in every example
pub fn make_box<'a, M: ?Sized>(a: &Point3, b: &Point3, material: &'a M) -> [Quad<&'a M>; 6] {
    // Construct the opposite vertices with the min and max coordinates
    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    [
        Quad::new(
            // front
            Point3::new(min.x(), min.y(), max.z()),
            dx.clone(),
            dy.clone(),
            material,
        ),
        Quad::new(
            // right
            Point3::new(max.x(), min.y(), max.z()),
            -&dz,
            dy.clone(),
            material,
        ),
        Quad::new(
            // back
            Point3::new(max.x(), min.y(), min.z()),
            -&dx,
            dy.clone(),
            material,
        ),
        Quad::new(
            // left
            Point3::new(min.x(), min.y(), min.z()),
            dz.clone(),
            dy.clone(),
            material,
        ),
        Quad::new(
            // top
            Point3::new(min.x(), max.y(), max.z()),
            dx.clone(),
            -&dz,
            material,
        ),
        Quad::new(
            // bottom
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            material,
        ),
    ]
}
