use crate::shapes::{Ellipse, EllipseArc, Line, Point, Quad, Triangle};

pub mod gl_triangle {
    use std::f32::consts::PI;

    use super::*;
    use crate::{
        tess::{
            primitives::{GlTriangle, GlTriangleVec},
            utils::{extend_line, line_angle},
        },
        StrokeCap,
    };

    fn _point(point: Point, weight: f32, cap: StrokeCap) -> GlTriangleVec {
        let half_weight = weight / 2.0;

        match cap {
            StrokeCap::Butt => GlTriangleVec::new(),
            StrokeCap::Round => _circle(point, half_weight, 20),
            StrokeCap::Square => _quad(
                Point(point.0 - half_weight, point.1 - half_weight),
                Point(point.0 + half_weight, point.1 - half_weight),
                Point(point.0 + half_weight, point.1 + half_weight),
                Point(point.0 - half_weight, point.1 + half_weight),
            ),
        }
    }

    fn _line_no_cap(a: Point, b: Point, weight: f32) -> GlTriangleVec {
        let half_weight = weight / 2.0;
        let angle = line_angle(a, b) + PI / 2.0;
        let ex = half_weight * angle.cos();
        let ey = half_weight * angle.sin();

        _quad(
            Point(a.0 - ex, a.1 - ey),
            Point(b.0 - ex, b.1 - ey),
            Point(b.0 + ex, b.1 + ey),
            Point(a.0 + ex, a.1 + ey),
        )
    }

    fn _line(a: Point, b: Point, weight: f32, cap: StrokeCap) -> GlTriangleVec {
        match cap {
            StrokeCap::Butt => _line_no_cap(a, b, weight),
            StrokeCap::Round => GlTriangleVec::new()
                .and(_circle(a, weight / 2.0, 20))
                .and(_circle(b, weight / 2.0, 20))
                .and(_line_no_cap(a, b, weight)),
            StrokeCap::Square => {
                let (a, b) = extend_line(a, b, weight);
                _line_no_cap(a, b, weight)
            }
        }
    }

    fn _triangle(a: Point, b: Point, c: Point) -> GlTriangle {
        GlTriangle::new(a, b, c)
    }

    fn _quad(a: Point, b: Point, c: Point, d: Point) -> GlTriangleVec {
        GlTriangleVec::from_elem(vec![GlTriangle::new(a, b, c), GlTriangle::new(a, c, d)])
    }

    fn _ellipse(center: Point, axes: (f32, f32), segments: usize) -> GlTriangleVec {
        let step = 2.0 * PI / segments as f32;
        GlTriangleVec::from_elem(
            (0..segments)
                .map(|i| (i as f32 * step, (i + 1) as f32 * step))
                .map(|(a, b)| {
                    GlTriangle::new(
                        Point(center.0, center.1),
                        Point(center.0 + a.cos() * axes.0, center.1 + a.sin() * axes.1),
                        Point(center.0 + b.cos() * axes.0, center.1 + b.sin() * axes.1),
                    )
                })
                .collect(),
        )
    }

    fn _ellipse_arc(
        center: Point,
        axes: (f32, f32),
        start: f32,
        end: f32,
        segments: usize,
    ) -> GlTriangleVec {
        let step = (end - start) / segments as f32;
        GlTriangleVec::from_elem(
            (0..segments)
                .map(|i| (start + i as f32 * step, start + (i + 1) as f32 * step))
                .map(|(a, b)| {
                    GlTriangle::new(
                        Point(center.0, center.1),
                        Point(center.0 + a.cos() * axes.0, center.1 + a.sin() * axes.1),
                        Point(center.0 + b.cos() * axes.0, center.1 + b.sin() * axes.1),
                    )
                })
                .collect(),
        )
    }

    fn _circle(center: Point, radius: f32, segments: usize) -> GlTriangleVec {
        _ellipse(center, (radius, radius), segments)
    }

    fn _arc(center: Point, radius: f32, start: f32, end: f32, segments: usize) -> GlTriangleVec {
        _ellipse_arc(center, (radius, radius), start, end, segments)
    }

    pub fn point(weight: f32, cap: StrokeCap) -> impl Fn(Point) -> GlTriangleVec {
        move |point| _point(point, weight, cap)
    }

    pub fn line(weight: f32, cap: StrokeCap) -> impl Fn(Line) -> GlTriangleVec {
        move |line| _line(line.0, line.1, weight, cap)
    }

    pub fn triangle() -> impl Fn(Triangle) -> GlTriangle {
        move |triangle| _triangle(triangle.0, triangle.1, triangle.2)
    }

    pub fn quad() -> impl Fn(Quad) -> GlTriangleVec {
        move |quad| _quad(quad.0, quad.1, quad.2, quad.3)
    }

    pub fn ellipse(segments: usize) -> impl Fn(Ellipse) -> GlTriangleVec {
        move |ellipse| _ellipse(ellipse.center, ellipse.axes, segments)
    }

    pub fn ellipse_arc(segments: usize) -> impl Fn(EllipseArc) -> GlTriangleVec {
        move |arc| {
            _ellipse_arc(
                arc.center,
                arc.axes,
                arc.start_angle,
                arc.end_angle,
                segments,
            )
        }
    }
}
