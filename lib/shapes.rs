use crate::tess::tessellator::Tessellator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub f32, pub f32);
pub struct Line(pub Point, pub Point);
pub struct Triangle(pub Point, pub Point, pub Point);
pub struct Quad(pub Point, pub Point, pub Point, pub Point);
pub struct Ellipse {
    pub center: Point,
    pub axes: (f32, f32),
}
pub struct EllipseArc {
    pub center: Point,
    pub axes: (f32, f32),
    pub start_angle: f32,
    pub end_angle: f32,
}

impl Tessellator for Point {}
impl Tessellator for Line {}
impl Tessellator for Triangle {}
impl Tessellator for Quad {}
impl Tessellator for Ellipse {}
impl Tessellator for EllipseArc {}

pub fn point(p: (f32, f32)) -> Point {
    Point(p.0, p.1)
}

pub fn line(a: (f32, f32), b: (f32, f32)) -> Line {
    Line(Point(a.0, a.1), Point(b.0, b.1))
}

pub fn triangle(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> Triangle {
    Triangle(Point(a.0, a.1), Point(b.0, b.1), Point(c.0, c.1))
}

pub fn quad(a: (f32, f32), b: (f32, f32), c: (f32, f32), d: (f32, f32)) -> Quad {
    Quad(
        Point(a.0, a.1),
        Point(b.0, b.1),
        Point(c.0, c.1),
        Point(d.0, d.1),
    )
}

pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Quad {
    Quad(
        Point(x, y),
        Point(x + width, y),
        Point(x + width, y + height),
        Point(x, y + height),
    )
}

pub fn ellipse(center: (f32, f32), axes: (f32, f32)) -> Ellipse {
    Ellipse {
        center: Point(center.0, center.1),
        axes,
    }
}

pub fn ellipse_arc(
    center: (f32, f32),
    axes: (f32, f32),
    start_angle: f32,
    end_angle: f32,
) -> EllipseArc {
    EllipseArc {
        center: Point(center.0, center.1),
        axes,
        start_angle,
        end_angle,
    }
}
