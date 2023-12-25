use ray_tracer_challenge::math::tuple::Tuple3;

struct Projectile {
    position: Tuple3,
    velocity: Tuple3
}

struct Environment {
    gravity: Tuple3,
    wind: Tuple3
}

fn main() {
    println!("Projectile simulation!");

    let mut p = Projectile {
        position: Tuple3::point(0.0, 1.0, 0.0),
        velocity: Tuple3::vec(1.0, 1.0, 0.0).norm()
    };

    let e = Environment {
        gravity: Tuple3::vec(0.0, -0.1, 0.0),
        wind: Tuple3::vec(-0.01, 0.0, 0.0)
    };

    let mut ticks = 0;
    while p.position.y() > 0.0 {
        p = tick(&e, &p);
        ticks += 1;
        println!("New position: {}", p.position);
    }

    println!("Simulation took {} ticks.", ticks);
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = &proj.position + &proj.velocity;
    let velocity = &(&proj.velocity + &env.gravity) + &env.wind;
    Projectile { position, velocity }
}
