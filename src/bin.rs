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

            p.background(Color::rgb(100, 100, 200));

            p.stroke(Color::rgb(255, 0, 0));
            p.stroke_weight(30.0);
            p.fill(Color::rgb(0, 255, 0));
            p.triangle(100.0, 100.0, 500.0, 800.0, 900.0, 100.0);
        })
        //         .draw(|p| {
        //             p.background(Color::rgb(100, 100, 32));
        //             p.circle(p.state.frame as f32, 500.0, 100.0);
        //             p.state.frame += 1;
        //
        //             if p.state.frame > 100 {
        //                 panic!("test");
        //             }
        //         })
        .build();

    processing.run()?;

    Ok(())
}
