use crate::shapes::Point;

pub struct GlTriangle {
    a: Point,
    b: Point,
    c: Point,
}

impl GlTriangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }
}

impl IntoIterator for GlTriangle {
    type Item = Point;
    type IntoIter = std::vec::IntoIter<Point>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.a, self.b, self.c].into_iter()
    }
}

pub struct GlTriangleVec(Vec<GlTriangle>);

impl GlTriangleVec {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_elem(vec: Vec<GlTriangle>) -> Self {
        Self(vec)
    }

    pub fn and(mut self, other: GlTriangleVec) -> Self {
        self.0.extend(other.0);
        self
    }
}

impl IntoIterator for GlTriangleVec {
    type Item = Point;
    type IntoIter = std::vec::IntoIter<Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .flat_map(|triangle| triangle.into_iter())
            .collect::<Vec<Point>>()
            .into_iter()
    }
}
