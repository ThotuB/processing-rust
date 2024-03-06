use p5::{Color, GraphicsP2D, Processing, Vector2D, QUARTER_PI};

use crate::examples::trees::tree::{Tree, TreeOptions};

pub fn app() -> Processing<(), GraphicsP2D> {
    p5::new()
        .setup(|p| {
            p.size(1000, 1000);
            p.title("trees");
            p.background(Color::hex_code("#74726E").unwrap());

            let options = TreeOptions {
                iterations: 10,
                position: Vector2D::new(500.0, 0.0),
                color: Color::hex_code("#74726E").unwrap(),
                trunk_length: 120.0,
                twig_length: 20.0,
                trunk_width: 40.0,
                twig_width: 1.0,
                trunk_branching: 0.0,
                twig_branching: 0.4,
                max_angle_offset: QUARTER_PI,
            };

            let mut tree = Tree::new(options);
            tree.generate();
            tree.draw(p);
        })
        .build()
}
