use rand::Rng;
use std::f64::consts::PI;
use std::cmp::{min, max};

#[derive(Debug)]
struct Body {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    fx: f64,
    fy: f64,
    mass: f64,
}

impl Body {
    fn update(&mut self, dt: f64) {
        self.vx += dt * self.fx / self.mass;
        self.vy += dt * self.fy / self.mass;
        self.x += dt * self.vx;
        self.y += dt * self.vy;
    }

    fn reset(&mut self) {
        self.fx = 0.0;
        self.fy = 0.0;
    }
}

fn calcForce(mut B: &mut Vec<Body>, b1_idx: usize, b2_idx: usize) -> &mut Vec<Body> {
    let b1 = &B.split_at_mut(min(b1_idx, b2_idx)).1.first_mut().unwrap();
    let b2 = &B.split_at_mut(max(b1_idx, b2_idx)).1.first_mut().unwrap();

    let dx: f64 = b1.x - b2.x;
    let dy: f64 = b1.y - b2.y;
    let dist: f64 = (dx * dx + dy * dy).sqrt();
    let force: f64 = 6.67408E-11 * b1.mass * b2.mass / ((dist + 6E+4) * (dist + 6E+4));
    // (force * dx / dist, force * dy / dist)
    b1.fx += force * dx / dist;
    b1.fy += force * dy / dist;
    b2.fx += force * dx / dist;
    b2.fy += force * dy / dist;

    B
}


fn gen_random_body(rng: &mut rand::rngs::ThreadRng, radius: f64) -> Body {
    let a: f64 = rng.gen::<f64>() * 2.0 * PI;
    let r: f64 = rng.gen::<f64>().sqrt() * radius;
    let x: f64 = a.cos() * r;
    let y: f64 = a.sin() * r;
    let perp_ratio: f64 = - y / x;
    let vx: f64 = if rng.gen_bool(0.5) {a * 4E4} else {-a * 4E4};
    let vy: f64 = perp_ratio * vx;
    let mass: f64 = (rng.gen::<f64>() + 1.0) * 1.989E30;
    Body{x, y, vx, vy, fx: 0.0, fy: 0.0, mass}
}

fn initialize(n: u32) -> Vec<Body> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| gen_random_body(&mut rng, 5E20)).collect::<Vec<Body>>()
}

fn integrate(n: u32, mut bodies: &mut Vec<Body>) -> Vec<Body> {
     *calcForce(&mut bodies, 0, 1)
}

fn main() {
    let mut bodies = initialize(100);
    bodies = integrate(10, &mut bodies);
    println!("{:?}", bodies[0]);
    // for _ in 0..1000 {

    //     for  in &bodies {
    //         for j in &bodies {
    //             *i.calcForce(&j);
    //         }
    //     }
    //     
    //     bodies.iter_mut().map(|b| {
    //         b.update(1E10);
    //         b.reset();
    //     });
    // }
}

