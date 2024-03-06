use p5::{Color, GraphicsP2D, Processing, Vector2D};

pub struct TreeOptions {
    pub iterations: u32,
    pub position: Vector2D,
    pub color: Color,
    // length
    pub trunk_length: f32,
    pub twig_length: f32,
    // width
    pub trunk_width: f32,
    pub twig_width: f32,
    // branching
    pub trunk_branching: f32,
    pub twig_branching: f32,
    pub max_angle_offset: f32,
}

#[derive(Debug, Clone, Copy)]
struct Branch {
    pub start: Vector2D,
    pub end: Vector2D,
    pub width: f32,
    pub color: Color,
}

impl Branch {
    fn new(start: Vector2D, end: Vector2D, width: f32, color: Color) -> Branch {
        Branch {
            start,
            end,
            width,
            color,
        }
    }

    fn length(&self) -> f32 {
        self.start.distance(self.end)
    }

    fn angle(&self) -> f32 {
        self.start.angle_between(self.end)
    }

    fn draw(&self, p: &mut Processing<(), GraphicsP2D>) {
        p.stroke(self.color);
        p.stroke_weight(self.width);
        p.line(self.start.x, self.start.y, self.end.x, self.end.y);
    }
}

pub struct Tree {
    options: TreeOptions,
    branches: Vec<Branch>,
}

impl Tree {
    pub fn new(options: TreeOptions) -> Tree {
        Tree {
            options,
            branches: vec![],
        }
    }

    pub fn generate(&mut self) {
        let trunk = Branch::new(
            self.options.position,
            Vector2D::new(
                self.options.position.x,
                self.options.position.y + self.options.trunk_length,
            ),
            self.options.trunk_width,
            self.options.color,
        );
        self.branches.push(trunk);

        self.branch(
            &trunk,
            self.options.trunk_branching,
            self.options.iterations,
        );
    }

    fn branch(&mut self, branch: &Branch, branching: f32, iterations: u32) {
        if iterations == 0 {
            return;
        }

        let mut end = branch.end;
        let mut angle = branch.angle();
        let mut length = branch.length();

        let mut left_angle = angle - branching;
        let mut right_angle = angle + branching;

        let mut left_end = Vector2D::new(
            end.x + left_angle.cos() * length,
            end.y + left_angle.sin() * length,
        );
        let mut right_end = Vector2D::new(
            end.x + right_angle.cos() * length,
            end.y + right_angle.sin() * length,
        );

        let left_branch = Branch::new(
            end,
            left_end,
            branch.width * self.options.twig_width,
            branch.color,
        );
        let right_branch = Branch::new(
            end,
            right_end,
            branch.width * self.options.twig_width,
            branch.color,
        );

        self.branches.push(left_branch);
        self.branches.push(right_branch);

        self.branch(&left_branch, self.options.twig_branching, iterations - 1);
        self.branch(&right_branch, self.options.twig_branching, iterations - 1);
    }

    pub fn draw(&self, p: &mut Processing<(), GraphicsP2D>) {
        for branch in &self.branches {
            branch.draw(p);
        }
    }
}
