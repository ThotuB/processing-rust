use crate::{core::vertex::Vertex, Color};

pub enum GeometryKind {
    Points,
    Lines,
    Triangles,
    TriangleFan,
    TriangleStrip,
    Quads,
    QuadStrip,
    Polygon,
}

pub struct GeometryVertex {
    x: f32,
    y: f32,
    z: f32,
    fill: Option<Color>,
    stroke: Option<Color>,
    stroke_weight: f32,
}

impl GeometryVertex {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        fill: Option<Color>,
        stroke: Option<Color>,
        stroke_weight: f32,
    ) -> Self {
        Self {
            x,
            y,
            z,
            fill,
            stroke,
            stroke_weight,
        }
    }
}

pub struct Geometry {
    kind: GeometryKind,
    vertices: Vec<GeometryVertex>,
}

impl Geometry {
    pub fn new(kind: GeometryKind) -> Self {
        Self {
            kind,
            vertices: Vec::new(),
        }
    }

    pub fn push_vertex(&mut self, vertex: GeometryVertex) {
        self.vertices.push(vertex);
    }

    pub fn tessellate(&self) -> Vec<Vertex> {
        unimplemented!()
    }
}
