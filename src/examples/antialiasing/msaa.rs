use p5::{App, Color, GraphicsP2D, EIGHTH_PI, HALF_PI, QUARTER_PI, SIXTEENTH_PI};
use winit::keyboard::KeyCode;

pub fn app() -> App<(), GraphicsP2D> {
    p5::new()
        .with_size(1000, 1200)
        .with_title("msaa")
        .setup(|p| {
            p.background(Color::rgb(0, 255, 255));
            p.no_fill();
            p.stroke(Color::rgb(255, 0, 0));
            p.stroke_weight(10.0);

            p.ellipse(200.0, 200.0, 50.0, 100.0);

            for i in 0..=16 {
                let angle = i as f32 * SIXTEENTH_PI / 2.0;
                p.line(
                    200.0,
                    200.0,
                    400.0 * angle.cos() + 200.0,
                    400.0 * angle.sin() + 200.0,
                );
            }
        })
        .key_pressed(|p, key| {
            if key == KeyCode::Space {
                p.screenshot("./resources/l-systems/screenshot.png")
                    .unwrap();
            }
        })
}
