use rand::Rng;
use std::f64::consts::PI;

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

    fn calcForce(&mut self, other: Body) {
        let dx: f64 = self.x - other.x;
        let dy: f64 = self.y - other.y;
        let dist: f64 = (dx * dx + dy * dy).sqrt();
        let force: f64 = 6.67408E-11 * self.mass * other.mass / ((dist + 6E+4) * (dist + 6E+4));
        self.fx += force * dx / dist;
        self.fy += force * dy / dist;
        // commenting this out for now. will double the calculations but im struggling with the
        // borrow checker. 
        //other.fx += force * dx / dist;
        //other.fy += force * dy / dist;
        
    }

    fn reset(&mut self) {
        self.fx = 0.0;
        self.fy = 0.0;
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

fn initialize(n: u32) -> Vec<Body> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| gen_random_body(&mut rng, 5E20)).collect::<Vec<Body>>()
}

fn integrate(n: u32, bodies: Vec<Body>) -> Vec<Body> {
    
    
    bodies
}

fn main() {
    let mut bodies = initialize(100);
    bodies[0].calcForce(bodies[1]);
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

