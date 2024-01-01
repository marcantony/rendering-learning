use std::{fs, time::SystemTime};

use ray_tracer_challenge::{
    draw::{canvas::Canvas, color::Color},
    scene::transformation,
    math::tuple::Tuple3,
};

fn main() {
    let hour_points: Vec<Tuple3> = vec![Tuple3::point(0.0, 1.0, 0.0); 12]
        .into_iter()
        .enumerate()
        .map(|(i, p)| {
            let r = transformation::rotation_z(-std::f64::consts::FRAC_PI_6 * i as f64);
            &r * &p
        })
        .collect();

    const CLOCK_CANVAS_RATIO: f64 = 3.0 / 8.0;
    const CANVAS_SIZE: usize = 400;
    const CENTER: usize = CANVAS_SIZE / 2;
    let translation = Tuple3::vec(CENTER as f64, CENTER as f64, 0.0);
    let mut c = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);

    hour_points.iter().for_each(|point| {
        let coords = &(&(point * CLOCK_CANVAS_RATIO) * CANVAS_SIZE as f64) + &translation;
        c.write(
            (coords.x().round() as usize, coords.y().round() as usize),
            Color::new(1.0, 1.0, 1.0),
        );
    });

    let ppm_data = c.ppm();
    let filename = format!(
        "clock-{}.ppm",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs()
    );
    fs::write(&filename, ppm_data).expect("unable to write file")
}
