use crate::{primitives::shapes_2d::Point, tess::tessellate::Tessellate};

pub trait Tessellator {
    #[inline]
    fn tessellate<F, P>(self, f: F) -> Tessellate<Self, F, P>
    where
        Self: Sized,
        F: FnOnce(Self) -> P,
        P: IntoIterator<Item = Point>,
    {
        Tessellate::new(self, f)
    }
}
