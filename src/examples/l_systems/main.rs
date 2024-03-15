use std::collections::HashMap;

use p5::{
    lerp, App, Color, GraphicsP2D, Processing, Vector2D, EIGHTH_PI, HALF_PI, QUARTER_PI,
    SIXTEENTH_PI,
};
use winit::keyboard::KeyCode;

use crate::{examples::l_systems::l_system::LSystemConfig, l_system_config};

type P = Processing<State, GraphicsP2D>;

pub fn draw(config: &LSystemConfig, start_pos: Vector2D, iterations: usize, p: &mut P) {
    p.stroke_cap(p5::StrokeCap::Round);
    p.background(Color::hex(0x84c3e3));

    for iteration in (1..=iterations) {
        let generation = config.generate(iteration);
        let mut current_pos = start_pos;
        let mut current_angle = HALF_PI;
        let mut stack = vec![];

        let branch_length = 80.0 / iteration as f32;

        let stroke_weight = 20.0 / iteration as f32;
        p.stroke_weight(stroke_weight);

        let stroke = lerp(
            Color::hex(0x664229),
            Color::hex(0xB99976),
            iteration as f32 / iterations as f32,
        );
        p.stroke(stroke);

        for c in generation.chars() {
            match c {
                'F' => {
                    let new_pos = current_pos + Vector2D::from_angle(current_angle) * branch_length;
                    p.line(current_pos.x, current_pos.y, new_pos.x, new_pos.y);
                    current_pos = new_pos;
                }
                '+' => current_angle += EIGHTH_PI,
                '-' => current_angle -= EIGHTH_PI,
                '[' => {
                    stack.push((current_pos, current_angle));
                }
                ']' => {
                    let (pos, angle) = stack.pop().unwrap();
                    current_pos = pos;
                    current_angle = angle;
                }
                _ => {}
            }
        }
    }
}

pub struct State {
    pub l_system_config: LSystemConfig,
    pub iterations: usize,
}

pub fn app() -> App<State, GraphicsP2D> {
    let state = State {
        l_system_config: l_system_config!(
            "F",
            'F' => "FF+[+F-F-F]-[-F+F+F]"
        )
        .unwrap(),
        iterations: 0,
    };

    p5::with_state(state)
        .with_size(1000, 1200)
        .with_title("l systems")
        .setup(|p| {
            let l_system_config = p.state.l_system_config.clone();
            let iterations = p.state.iterations;

            draw(&l_system_config, Vector2D::new(500.0, 10.0), iterations, p);
        })
        .key_pressed(|p, key| {
            if key == KeyCode::Enter {
                p.state.iterations += 1;

                p.title(&format!("l systems - iterations: {}", p.state.iterations));

                let l_system_config = p.state.l_system_config.clone();
                let iterations = p.state.iterations;

                draw(&l_system_config, Vector2D::new(500.0, 10.0), iterations, p);
                p.redraw();
            } else if key == KeyCode::Space {
                p.screenshot("./resources/l-systems/screenshot.png")
                    .unwrap();
            }
        })
}
