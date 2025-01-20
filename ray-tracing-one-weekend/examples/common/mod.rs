use std::{
    fs,
    io::{self, BufWriter},
    time::SystemTime,
};
use std::io::{Cursor, Write};

use image::ImageFormat;
use ray_tracing_one_weekend::{
    camera::{Camera, Canvas},
    hittable::{flat::quad::Quad, Hittable},
    material::Material,
    output,
    vec3::{Point3, Vec3},
};

#[allow(dead_code)]
pub fn render_to_stdout<M: Material, H: Hittable<Material = M>>(world: &H, camera: &Camera) {
    let canvas = camera.render(&world);

    let mut buffer = Cursor::new(Vec::new());
    output::output_image(&canvas, ImageFormat::Png, &mut buffer).unwrap();

    let mut out = std::io::stdout();
    out.write_all(&buffer.into_inner()).unwrap();
    out.flush().unwrap();
}

#[allow(dead_code)]
pub fn render_save_checkpoint<M: Material, H: Hittable<Material = M>>(
    world: &H,
    camera: &Camera,
    checkpoint_name: &str,
) {
    let mut out = BufWriter::new(io::stdout().lock());
    let canvas = camera.render(&world);

    let output_checkpoint = bincode::serialize(&canvas).unwrap();
    write_to_file(&output_checkpoint, checkpoint_name, "chkpt");

    output::output_ppm(&canvas, &mut out).unwrap();
}

#[allow(dead_code)]
pub fn render_from_checkpoint<M: Material, H: Hittable<Material = M>>(
    world: &H,
    camera: &Camera,
    checkpoint_name: &str,
) {
    let checkpoint_bytes: Vec<u8> = read_from_file(checkpoint_name);
    let checkpoint: Canvas = bincode::deserialize(&checkpoint_bytes.as_slice()).unwrap();

    let mut out = BufWriter::new(io::stdout().lock());
    let new_render = camera.render_from_checkpoint(&world, &checkpoint);

    let output_checkpoint = bincode::serialize(&new_render).unwrap();
    write_to_file(&output_checkpoint, checkpoint_name, "chkpt");

    output::output_ppm(&new_render, &mut out).unwrap();
}

fn write_to_file(data: &[u8], filename_prefix: &str, extension: &str) {
    let filename = format!(
        "{}-{}.{}",
        filename_prefix,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs(),
        extension
    );
    fs::write(filename, data).expect("unable to write file")
}

fn read_from_file(filename: &str) -> Vec<u8> {
    fs::read(filename).unwrap()
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
