use crate::{core::vertex::Vertex, settings::StrokeJoin, Color, StrokeCap};

pub trait Renderer {
    fn shapes(&self) -> &Vec<Vertex>;
    fn stroke(&mut self, color: Option<Color>);
    fn stroke_weight(&mut self, weight: f32);
    fn stroke_cap(&mut self, cap: StrokeCap);
    fn stroke_join(&mut self, join: StrokeJoin);
    fn fill(&mut self, color: Option<Color>);
}
