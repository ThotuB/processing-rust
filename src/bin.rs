use p5::Color;

extern crate glium;
extern crate processing as p5;
extern crate winit;

struct State {
    frame: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = State { frame: 0 };
    let processing = p5::with_state(state)
        .setup(|p| {
            p.size(1000, 1000);
            p.title("TEST");

            p.background(Color::rgb(100, 100, 32));

            p.fill(Color::rgb(0, 0, 255));
            // star of david
            p.triangle(400.0, 200.0, 500.0, 200.0, 450.0, 114.0);
            p.triangle(500.0, 200.0, 550.0, 286.0, 600.0, 200.0);
            p.triangle(550.0, 286.0, 500.0, 372.0, 600.0, 372.0);
            p.triangle(400.0, 372.0, 500.0, 372.0, 450.0, 458.0);
            p.triangle(300.0, 372.0, 400.0, 372.0, 350.0, 286.0);
            p.triangle(300.0, 200.0, 400.0, 200.0, 350.0, 286.0);
        })
        .draw(|p| {
            p.background(Color::rgb(100, 100, 32));
            p.circle(p.state.frame as f32, 500.0, 100.0);
            p.state.frame += 1;

            if p.state.frame > 100 {
                p.no_loop();
            }
        })
        .build();

    processing.run()?;

    Ok(())
}
