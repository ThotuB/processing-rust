use noise::{NoiseFn, Perlin};
use p5::{Application, Color};
use winit::keyboard::KeyCode;

pub fn app() -> impl Application {
    let noise = Perlin::new(0);

    p5::new()
        .with_size(500, 500)
        .with_title("perlin noise")
        .setup(move |p| {
            p.stroke_weight(2.0);
            p.stroke_cap(p5::StrokeCap::Round);
            let mut max = 0.0;
            let mut min = 0.0;
            for x in 0..p.width() {
                for y in 0..p.height() {
                    let n = noise.get([x as f64 / 20.0, y as f64 / 20.0]) * 122.0 + 122.0;

                    if n > max {
                        max = n;
                    }
                    if n < min {
                        min = n;
                    }

                    p.stroke(Color::red(n as u8));
                    p.point(x as f32, y as f32);
                }
            }

            println!("max: {}, min: {}", max, min);
        })
        .key_pressed(|p, key| {
            if key == KeyCode::Space {
                p.screenshot("./resources/noise/screenshot-2.png").unwrap();
            }
        })
}
