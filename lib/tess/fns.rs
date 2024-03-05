use crate::primitives::shapes_2d::Point;

pub mod gl_triangle {
    use std::f32::consts::PI;

    use super::*;
    use crate::{
        primitives::shapes_2d::Edges,
        tess::{
            primitives::{GlTriangle, GlTriangleVec},
            utils::{extend_line, line_angle},
        },
        StrokeCap,
    };

    pub fn point(point: Point, weight: f32, cap: StrokeCap) -> GlTriangleVec {
        let half_weight = weight / 2.0;

        match cap {
            StrokeCap::Butt => GlTriangleVec::new(),
            StrokeCap::Round => circle(point, half_weight, 20),
            StrokeCap::Square => quad(
                Point::new((point.x - half_weight, point.y - half_weight)),
                Point::new((point.x + half_weight, point.y - half_weight)),
                Point::new((point.x + half_weight, point.y + half_weight)),
                Point::new((point.x - half_weight, point.y + half_weight)),
            ),
        }
    }

    pub fn line_no_cap(a: Point, b: Point, weight: f32) -> GlTriangleVec {
        let half_weight = weight / 2.0;
        let angle = line_angle(a, b) + PI / 2.0;
        let ex = half_weight * angle.cos();
        let ey = half_weight * angle.sin();

        quad(
            Point::new((a.x - ex, a.y - ey)),
            Point::new((b.x - ex, b.y - ey)),
            Point::new((b.x + ex, b.y + ey)),
            Point::new((a.x + ex, a.y + ey)),
        )
    }

    pub fn line(a: Point, b: Point, weight: f32, cap: StrokeCap) -> GlTriangleVec {
        match cap {
            StrokeCap::Butt => line_no_cap(a, b, weight),
            StrokeCap::Round => GlTriangleVec::new()
                .and(circle(a, weight / 2.0, 20))
                .and(circle(b, weight / 2.0, 20))
                .and(line_no_cap(a, b, weight)),
            StrokeCap::Square => {
                let (a, b) = extend_line(a, b, weight);
                line_no_cap(a, b, weight)
            }
        }
    }

    pub fn triangle(a: Point, b: Point, c: Point) -> GlTriangle {
        GlTriangle::new(a, b, c)
    }

    pub fn quad(a: Point, b: Point, c: Point, d: Point) -> GlTriangleVec {
        GlTriangleVec::from_elem(vec![GlTriangle::new(a, b, c), GlTriangle::new(a, c, d)])
    }

    pub fn ellipse(center: Point, axes: (f32, f32), segments: usize) -> GlTriangleVec {
        let step = 2.0 * PI / segments as f32;
        GlTriangleVec::from_elem(
            (0..segments)
                .map(|i| (i as f32 * step, (i + 1) as f32 * step))
                .map(|(a, b)| {
                    GlTriangle::new(
                        Point::new((center.x, center.y)),
                        Point::new((center.x + a.cos() * axes.0, center.y + a.sin() * axes.1)),
                        Point::new((center.x + b.cos() * axes.0, center.y + b.sin() * axes.1)),
                    )
                })
                .collect(),
        )
    }

    pub fn ellipse_arc(
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
                        Point::new((center.x, center.y)),
                        Point::new((center.x + a.cos() * axes.0, center.y + a.sin() * axes.1)),
                        Point::new((center.x + b.cos() * axes.0, center.y + b.sin() * axes.1)),
                    )
                })
                .collect(),
        )
    }

    pub fn circle(center: Point, radius: f32, segments: usize) -> GlTriangleVec {
        ellipse(center, (radius, radius), segments)
    }

    pub fn arc(center: Point, radius: f32, start: f32, end: f32, segments: usize) -> GlTriangleVec {
        ellipse_arc(center, (radius, radius), start, end, segments)
    }

    pub fn stroke(edges: Edges, weight: f32) -> GlTriangleVec {
        edges
            .into_iter()
            .map(|edge| line(edge.a, edge.b, weight, StrokeCap::Round))
            .fold(GlTriangleVec::new(), |acc, x| acc.and(x))
    }
}
