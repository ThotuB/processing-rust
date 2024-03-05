use crate::{
    core::vertex::{vert2d, Vertex},
    primitives::shapes_2d::Point,
    Color,
};

pub struct Tessellate<S, F, P>
where
    F: FnOnce(S) -> P,
{
    input: S,
    tess_fn: F,
}

impl<S, F, P> Tessellate<S, F, P>
where
    F: FnOnce(S) -> P,
    P: IntoIterator<Item = Point>,
{
    pub fn new(input: S, tess_fn: F) -> Tessellate<S, F, P> {
        Tessellate { input, tess_fn }
    }

    pub fn color(self, color: Color) -> impl Iterator<Item = Vertex> {
        let points = (self.tess_fn)(self.input);
        points
            .into_iter()
            .map(move |point| vert2d(point.x, point.y, color))
    }
}
