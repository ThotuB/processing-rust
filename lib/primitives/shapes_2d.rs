use crate::{
    tess::{
        fns::gl_triangle,
        primitives::{GlTriangle, GlTriangleVec},
        tessellate::Tessellate,
        tessellator::Tessellator,
    },
    StrokeCap,
};

/// Point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(vertex: (f32, f32)) -> Point {
        Point {
            x: vertex.0,
            y: vertex.1,
        }
    }

    pub fn tessellate_fill(
        self,
        weight: f32,
        cap: StrokeCap,
    ) -> Tessellate<Point, impl FnOnce(Point) -> GlTriangleVec, GlTriangleVec> {
        self.tessellate(move |point| gl_triangle::point(point, weight, cap))
    }
}

impl Tessellator for Point {}

/// Edge
pub struct Edge {
    pub a: Point,
    pub b: Point,
}

impl Edge {
    pub fn new(a: Point, b: Point) -> Edge {
        Edge { a, b }
    }
}

pub struct Edges(Vec<Edge>);

impl Edges {
    pub fn new(edges: Vec<Edge>) -> Edges {
        Edges(edges)
    }

    pub fn intersections(&self) -> Vec<(&Edge, &Edge)> {
        self.0.iter().zip(self.0.iter().cycle().skip(1)).collect()
    }

    pub fn tessellate_stroke(
        self,
        weight: f32,
    ) -> Tessellate<Edges, impl FnOnce(Edges) -> GlTriangleVec, GlTriangleVec> {
        self.tessellate(move |edges| gl_triangle::stroke(edges, weight))
    }
}

impl Tessellator for Edges {}

impl IntoIterator for Edges {
    type Item = Edge;
    type IntoIter = std::vec::IntoIter<Edge>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Line
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl Line {
    pub fn new(a: (f32, f32), b: (f32, f32)) -> Line {
        Line {
            a: Point::new(a),
            b: Point::new(b),
        }
    }

    pub fn tessellate_fill(
        self,
        weight: f32,
        cap: StrokeCap,
    ) -> Tessellate<Line, impl FnOnce(Line) -> GlTriangleVec, GlTriangleVec> {
        self.tessellate(move |line| gl_triangle::line(line.a, line.b, weight, cap))
    }
}

impl Tessellator for Line {}

/// Triangle
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> Triangle {
        Triangle {
            a: Point::new(a),
            b: Point::new(b),
            c: Point::new(c),
        }
    }

    fn edges(self) -> Edges {
        Edges::new(vec![
            Edge::new(self.a, self.b),
            Edge::new(self.b, self.c),
            Edge::new(self.c, self.a),
        ])
    }

    pub fn tessellate_fill(
        self,
    ) -> Tessellate<Triangle, impl FnOnce(Triangle) -> GlTriangle, GlTriangle> {
        self.tessellate(move |triangle| gl_triangle::triangle(triangle.a, triangle.b, triangle.c))
    }

    pub fn tessellate_stroke(
        self,
        weight: f32,
    ) -> Tessellate<Edges, impl FnOnce(Edges) -> GlTriangleVec, GlTriangleVec> {
        self.edges().tessellate_stroke(weight)
    }
}

impl Tessellator for Triangle {}

/// Quad
pub struct Quad {
    pub a: Point,
    pub b: Point,
    pub c: Point,
    pub d: Point,
}

impl Quad {
    pub fn new(a: (f32, f32), b: (f32, f32), c: (f32, f32), d: (f32, f32)) -> Quad {
        Quad {
            a: Point::new(a),
            b: Point::new(b),
            c: Point::new(c),
            d: Point::new(d),
        }
    }

    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Quad {
        Quad::new((x, y), (x + w, y), (x + w, y + h), (x, y + h))
    }

    fn edges(self) -> Edges {
        Edges::new(vec![
            Edge::new(self.a, self.b),
            Edge::new(self.b, self.c),
            Edge::new(self.c, self.d),
            Edge::new(self.d, self.a),
        ])
    }

    pub fn tessellate_fill(
        self,
    ) -> Tessellate<Quad, impl FnOnce(Quad) -> GlTriangleVec, GlTriangleVec> {
        self.tessellate(move |quad| gl_triangle::quad(quad.a, quad.b, quad.c, quad.d))
    }

    pub fn tessellate_stroke(
        self,
        weight: f32,
    ) -> Tessellate<Edges, impl FnOnce(Edges) -> GlTriangleVec, GlTriangleVec> {
        self.edges().tessellate_stroke(weight)
    }
}

impl Tessellator for Quad {}

/// Ellipse
pub struct Ellipse {
    pub center: Point,
    pub axes: (f32, f32),
}

impl Ellipse {
    pub fn new(center: (f32, f32), axes: (f32, f32)) -> Ellipse {
        Ellipse {
            center: Point::new(center),
            axes,
        }
    }

    fn edges(self, segments: usize) -> Edges {
        let step = 2.0 * std::f32::consts::PI / segments as f32;
        let points = (0..segments)
            .map(|i| (i as f32 * step, (i + 1) as f32 * step))
            .map(|(a, b)| {
                (
                    Point::new((
                        self.center.x + a.cos() * self.axes.0,
                        self.center.y + a.sin() * self.axes.1,
                    )),
                    Point::new((
                        self.center.x + b.cos() * self.axes.0,
                        self.center.y + b.sin() * self.axes.1,
                    )),
                )
            })
            .map(|(a, b)| Edge::new(a, b))
            .collect();
        Edges::new(points)
    }

    pub fn tessellate_fill(
        self,
        segments: usize,
    ) -> Tessellate<Ellipse, impl FnOnce(Ellipse) -> GlTriangleVec, GlTriangleVec> {
        self.tessellate(move |ellipse| gl_triangle::ellipse(ellipse.center, ellipse.axes, segments))
    }

    pub fn tessellate_stroke(
        self,
        weight: f32,
        segments: usize,
    ) -> Tessellate<Edges, impl FnOnce(Edges) -> GlTriangleVec, GlTriangleVec> {
        self.edges(segments).tessellate_stroke(weight)
    }
}

impl Tessellator for Ellipse {}

/// EllipseArc
pub struct EllipseArc {
    pub center: Point,
    pub axes: (f32, f32),
    pub start_angle: f32,
    pub end_angle: f32,
}

impl EllipseArc {
    pub fn new(
        center: (f32, f32),
        axes: (f32, f32),
        start_angle: f32,
        end_angle: f32,
    ) -> EllipseArc {
        EllipseArc {
            center: Point::new(center),
            axes,
            start_angle,
            end_angle,
        }
    }

    fn edges(self, segments: usize) -> Edges {
        let step = (self.end_angle - self.start_angle) / segments as f32;
        let points = (0..segments)
            .map(|i| {
                (
                    self.start_angle + i as f32 * step,
                    self.start_angle + (i + 1) as f32 * step,
                )
            })
            .map(|(a, b)| {
                (
                    Point::new((
                        self.center.x + a.cos() * self.axes.0,
                        self.center.y + a.sin() * self.axes.1,
                    )),
                    Point::new((
                        self.center.x + b.cos() * self.axes.0,
                        self.center.y + b.sin() * self.axes.1,
                    )),
                )
            })
            .map(|(a, b)| Edge::new(a, b))
            .collect();
        Edges::new(points)
    }

    pub fn tessellate_fill(
        self,
        segments: usize,
    ) -> Tessellate<EllipseArc, impl FnOnce(EllipseArc) -> GlTriangleVec, GlTriangleVec> {
        self.tessellate(move |arc| {
            gl_triangle::ellipse_arc(
                arc.center,
                arc.axes,
                arc.start_angle,
                arc.end_angle,
                segments,
            )
        })
    }

    pub fn tessellate_stroke(
        self,
        weight: f32,
        segments: usize,
    ) -> Tessellate<Edges, impl FnOnce(Edges) -> GlTriangleVec, GlTriangleVec> {
        self.edges(segments).tessellate_stroke(weight)
    }
}

impl Tessellator for EllipseArc {}
