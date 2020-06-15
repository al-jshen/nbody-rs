use rand::Rng;
use std::f64::consts::PI;
use std::cell::RefCell;

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

    fn calc_force(&mut self, other: &mut Body) {
        let dx: f64 = self.x - other.x;
        let dy: f64 = self.y - other.y;
        let dist: f64 = (dx * dx + dx * dx).sqrt();
        let force: f64 = 6.67408E-11 * self.mass * other.mass / ((dist + 6E+3) * (dist + 6E+3));
        self.fx += force * dx / dist;
        self.fy += force * dy / dist;
        other.fx += force * dx / dist;
        other.fy += force * dy / dist;
    }
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

fn initialize(n: u32) -> Vec<RefCell<Body>> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| RefCell::new(gen_random_body(&mut rng, 5E20))).collect::<Vec<RefCell<Body>>>()
}

fn integrate(n: u32, bodies: &Vec<RefCell<Body>>) {
    for _ in 0..n {
        for i in 0..bodies.len() {
            for j in i+1..bodies.len() {
                let mut b1 = bodies[i as usize].borrow_mut();
                let mut b2 = bodies[j as usize].borrow_mut();
                b1.calc_force(&mut b2);
            }
        }

        for k in 0..bodies.len() {
            let mut body = bodies[k as usize].borrow_mut();
            body.update(1E9);
            body.reset();
        }
        
        println!("({}, {}),\\", bodies[0].borrow().x, bodies[0].borrow().y);
    }


    
}

fn main() {
    let bodies = initialize(100);
    println!("{:?}", &bodies[0]);
    integrate(100, &bodies);
    println!("{:?}", &bodies[0]);
}

