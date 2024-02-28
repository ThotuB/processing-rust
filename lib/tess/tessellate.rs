use crate::{
    core::vertex::{vert2d, Vertex},
    shapes::Point,
    Color,
};

pub struct Tessellate<S, F, P>
where
    F: FnOnce(S) -> P,
    P: IntoIterator<Item = Point>,
{
    input: S,
    tessellator: F,
}

impl<S, F, P> Tessellate<S, F, P>
where
    F: FnOnce(S) -> P,
    P: IntoIterator<Item = Point>,
{
    pub fn new(input: S, tessellator: F) -> Tessellate<S, F, P> {
        Tessellate { input, tessellator }
    }

    pub fn color(self, color: Color) -> Vec<Vertex> {
        let points = (self.tessellator)(self.input);
        points
            .into_iter()
            .map(|point| vert2d(point.0, point.1, color))
            .collect()
    }
}
