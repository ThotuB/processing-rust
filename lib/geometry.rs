use glium::index;

use crate::{
    core::vertex::{vert3d},
    gl_shape::LazyGlShape,
    Color,
};

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

impl From<GeometryKind> for index::PrimitiveType {
    fn from(val: GeometryKind) -> Self {
        match val {
            GeometryKind::Points => index::PrimitiveType::Points,
            GeometryKind::Lines => index::PrimitiveType::LinesList,
            GeometryKind::Triangles => index::PrimitiveType::TrianglesList,
            GeometryKind::TriangleFan => index::PrimitiveType::TriangleFan,
            GeometryKind::TriangleStrip => index::PrimitiveType::TriangleStrip,
            GeometryKind::Quads => index::PrimitiveType::TrianglesList,
            GeometryKind::QuadStrip => index::PrimitiveType::TriangleStrip,
            GeometryKind::Polygon => index::PrimitiveType::TrianglesList,
        }
    }
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

    fn vertices(&mut self) -> Vec<GeometryVertex> {
        self.vertices.drain(..).collect()
    }

    pub fn tessellate(mut self) -> LazyGlShape {
        let shape = self
            .vertices()
            .into_iter()
            .map(|v| vert3d(v.x, v.y, v.z, v.fill.unwrap_or(Color::TRANSPARENT)));

        LazyGlShape::new(shape, self.kind.into())
    }
}
