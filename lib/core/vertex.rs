use crate::{Color};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    position: [f32; 3], // x, y, z
    color: [f32; 4],    // r, g, b, a
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32, color: Color) -> Self {
        Vertex {
            position: [x, y, z],
            color: color.into(),
        }
    }
}

pub fn vert2d(x: f32, y: f32, color: Color) -> Vertex {
    Vertex::new(x, y, 0.0, color)
}

pub fn vert3d(x: f32, y: f32, z: f32, color: Color) -> Vertex {
    Vertex::new(x, y, z, color)
}

implement_vertex!(Vertex, position, color);
