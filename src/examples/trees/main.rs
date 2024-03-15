use p5::{App, Color, GraphicsP2D, Processing, Vector2D, QUARTER_PI, SIXTEENTH_PI};

use crate::examples::trees::tree::{MaxToMinFn, Tree, TreeOptions};

pub fn app() -> App<(), GraphicsP2D> {
    p5::new()
        .with_size(1000, 1000)
        .with_title("trees")
        .setup(|p| {
            p.background(Color::hex(0x84c3e3));

            let options = TreeOptions {
                iterations: 5,
                position: Vector2D::new(500.0, 0.0),
                color: Color::hex(0xa38446),
                branch_length: MaxToMinFn::lerp(80.0, 50.0),
                branch_width: MaxToMinFn::lerp(40.0, 1.0),
                branch_branching: MaxToMinFn::lerp(1.0, 1.0),
                branch_max_angle: MaxToMinFn::lerp(SIXTEENTH_PI, 3.0 * QUARTER_PI),
            };

            let tree = Tree::new(options).generate();
            tree.draw(p);
        })
}
