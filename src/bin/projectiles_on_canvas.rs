use punytracer::canvas::Canvas;
use punytracer::color::Color;
use punytracer::core::point::Point;
use punytracer::core::tuple::Tuple;
use punytracer::core::vector::Vector;

struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity, }
    }
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind, }
    }

    fn tick(&self, projectile: &Projectile) -> Projectile {
        let position = projectile.position + projectile.velocity;
        let velocity = projectile.velocity + self.gravity + self.wind;

        Projectile::new(position, velocity)
    }
}

fn main() {
    let v = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;
    let g = Vector::new(0.0, -0.1, 0.0);
    let w = Vector::new(-0.01, 0.0, 0.0);
    let e = Environment::new(g, w);

    let mut p = Projectile::new(Point::new(0.0, 1.0, 0.0), v);
    let mut c = Canvas::new(900, 550);

    let mut counter = 0u64;
    let height = c.height();

    while p.position.y() > 0.0 {
        let (x, y) = (p.position.x() as usize, (height as f64 - p.position.y()).floor() as usize);
        c[(x, y)] = Color::new(1.0, 0.0, 0.0);
        p = e.tick(&p);
        counter = counter + 1;
        println!("Current position of the projectile: {:?}", &p.position);
    }

    println!("It took {} ticks for the projectile to reach the ground!", counter);
    c.save_to(&"projectile.png");
}
