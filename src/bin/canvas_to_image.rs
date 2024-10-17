use punytracer::canvas::Canvas;
use punytracer::color::Color;

fn main() {
    let mut canvas = Canvas::new(800, 600);

    for y in (0..600).step_by(10) {
        for x in (0..800).step_by(10) {
            canvas[(x, y)] = Color::new(1.0, 0.0, 0.0);
        }
    }

    canvas.save_to(&"traced.png");
}