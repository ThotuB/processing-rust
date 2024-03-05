use crate::primitives::shapes_2d::Point;

pub(crate) fn line_angle(a: Point, b: Point) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    dy.atan2(dx)
}

pub(crate) fn extend_line(a: Point, b: Point, weight: f32) -> (Point, Point) {
    let half_weight = weight / 2.0;
    let angle = line_angle(a, b);
    let ex = half_weight * angle.cos();
    let ey = half_weight * angle.sin();

    (
        Point::new((a.x - ex, a.y - ey)),
        Point::new((b.x + ex, b.y + ey)),
    )
}
