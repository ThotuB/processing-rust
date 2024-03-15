use p5::{lerp, Color, GraphicsP2D, Processing, Vector2D, RAD_TO_DEG};

pub struct MaxToMinFn {
    pub max: f32,
    pub min: f32,
    func: fn(f32, f32, f32) -> f32,
}

impl MaxToMinFn {
    pub fn custom_fn(max: f32, min: f32, func: fn(f32, f32, f32) -> f32) -> MaxToMinFn {
        MaxToMinFn { max, min, func }
    }

    pub fn lerp(max: f32, min: f32) -> MaxToMinFn {
        MaxToMinFn::custom_fn(max, min, lerp)
    }

    pub fn map(&self, value: f32) -> f32 {
        (self.func)(self.max, self.min, value)
    }
}

pub struct BranchOptions {}

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

    fn from_angle(start: Vector2D, angle: f32, length: f32, width: f32, color: Color) -> Branch {
        let end = Vector2D::from_angle(angle) * length + start;
        Branch::new(start, end, width, color)
    }

    fn length(&self) -> f32 {
        self.start.distance(self.end)
    }

    fn angle(&self) -> f32 {
        self.start.angle_to(self.end)
    }

    fn draw(&self, p: &mut Processing<(), GraphicsP2D>) {
        p.stroke(self.color);
        p.stroke_weight(self.width);
        p.line(self.start.x, self.start.y, self.end.x, self.end.y);
    }
}

pub struct TreeOptions {
    pub iterations: u32,
    pub position: Vector2D,
    pub color: Color,
    // length
    pub branch_length: MaxToMinFn,
    // width
    pub branch_width: MaxToMinFn,
    // branching
    pub branch_branching: MaxToMinFn,
    // angle
    pub branch_max_angle: MaxToMinFn,
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

    pub fn generate(mut self) -> Tree {
        let trunk = Branch::new(
            self.options.position,
            Vector2D::new(
                self.options.position.x,
                self.options.position.y + self.options.branch_length.max,
            ),
            self.options.branch_width.max,
            self.options.color,
        );
        self.branches.push(trunk);

        println!("trunk: {:#?}", trunk.angle() * RAD_TO_DEG);

        self.grow(&trunk, 0);
        self
    }

    fn branch_out(&mut self, branch: &Branch, iteration: u32) {
        self.grow_out(branch, iteration);
        self.grow_out(branch, iteration);
    }

    fn grow_out(&mut self, branch: &Branch, iteration: u32) {
        let t = iteration as f32 / self.options.iterations as f32;

        let start = branch.end;
        let angle =
            branch.angle() + self.options.branch_max_angle.map(t) * (rand::random::<f32>() - 0.5);
        let length = self.options.branch_length.map(t);
        let width = self.options.branch_width.map(t);
        let color = self.options.color;

        let new_branch = Branch::from_angle(start, angle, length, width, color);
        // println!("new_branch {:?}: {:#?}", iteration, new_branch);
        self.branches.push(new_branch);

        self.grow(&new_branch, iteration + 1);
    }

    fn grow(&mut self, branch: &Branch, iteration: u32) {
        if iteration == self.options.iterations {
            return;
        }

        let t = iteration as f32 / self.options.iterations as f32;
        let branching = self.options.branch_branching.map(t);
        if rand::random::<f32>() < branching {
            self.branch_out(branch, iteration);
        } else {
            self.grow_out(branch, iteration);
        }
    }

    pub fn draw(&self, p: &mut Processing<(), GraphicsP2D>) {
        for branch in &self.branches {
            branch.draw(p);
        }
    }
}
