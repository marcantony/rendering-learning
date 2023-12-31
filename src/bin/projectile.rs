use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use ray_tracer_challenge::{
    draw::{canvas::Canvas, color::Color},
    math::tuple::Tuple3,
};

struct Projectile {
    position: Tuple3,
    velocity: Tuple3,
}

struct Environment {
    gravity: Tuple3,
    wind: Tuple3,
}

fn main() {
    println!("Projectile simulation!");

    let mut p = Projectile {
        position: Tuple3::point(0.0, 1.0, 0.0),
        velocity: &Tuple3::vec(1.0, 1.8, 0.0).norm() * 11.25,
    };

    let e = Environment {
        gravity: Tuple3::vec(0.0, -0.1, 0.0),
        wind: Tuple3::vec(-0.01, 0.0, 0.0),
    };

    let mut c = Canvas::new(900, 550);

    let mut ticks = 0;
    while p.position.y() > 0.0 {
        p = tick(&e, &p);

        let canvas_x = p.position.x().round() as usize;
        let canvas_y = c.height() - (p.position.y().round() as usize);
        c.write((canvas_x, canvas_y), Color::new(1.0, 0.5, 0.5));

        ticks += 1;

        println!("New position: {}", p.position);
    }

    println!("Simulation took {} ticks.", ticks);

    let ppm_data = c.ppm();

    let filename = format!(
        "projectile-{}.ppm",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs()
    );
    fs::write(&filename, ppm_data).expect("unable to write file")
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = &proj.position + &proj.velocity;
    let velocity = &(&proj.velocity + &env.gravity) + &env.wind;
    Projectile { position, velocity }
}
