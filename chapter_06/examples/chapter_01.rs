use rustic_ray::{Point, Vector};

#[derive(Debug)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

fn main() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };

    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    loop {
        p = tick(&e, p);
        println!("{:?}", p);
        if p.position.y <= 0.0 {
            break;
        }
    }
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}
