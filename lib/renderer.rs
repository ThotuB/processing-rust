use crate::{core::vertex::Vertex, geometry::GeometryKind, settings::StrokeJoin, Color, StrokeCap};

pub trait Renderer {
    fn shapes(&self) -> &Vec<Vertex>;
}

pub trait Stroke {
    fn stroke(&mut self, color: Option<Color>);
    fn stroke_weight(&mut self, weight: f32);
    fn stroke_cap(&mut self, cap: StrokeCap);
    fn stroke_join(&mut self, join: StrokeJoin);
    fn fill(&mut self, color: Option<Color>);
}

pub trait VertexShape {
    type Item;

    fn begin_shape(&mut self, kind: GeometryKind);
    fn vertex(&mut self, vertex: Self::Item);
    fn end_shape(&mut self);
}
