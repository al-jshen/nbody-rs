use std::collections::HashMap;

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

    fn calcForce(&mut self, other: &mut Body) {
        let dx: f64 = self.x - other.x;
        let dy: f64 = self.y - other.y;
        let dist: f64 = (dx * dx + dy * dy).sqrt();
        let force: f64 = 6.67408E-11 * self.mass * other.mass / ((dist + 6E+4) * (dist + 6E+4));
        self.fx += force * dx / dist;
        self.fy += force * dy / dist;
        other.fx += force * dx / dist;
        other.fy += force * dy / dist;
    }

    fn reset(&mut self) {
        self.fx = 0.0;
        self.fy = 0.0;
    }
}

fn main() {
    let mut constants = HashMap::new();
    constants.insert(String::from("G"), 6.67408e-11); // gravitational constant
    constants.insert(String::from("eps"), 6E+4); // softening parameter
    println!("Hello, world!");
}
