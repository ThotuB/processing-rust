use crate::shapes::Point;

pub(crate) fn line_angle(a: Point, b: Point) -> f32 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    dy.atan2(dx)
}

pub(crate) fn extend_line(a: Point, b: Point, weight: f32) -> (Point, Point) {
    let half_weight = weight / 2.0;
    let angle = line_angle(a, b);
    let ex = half_weight * angle.cos();
    let ey = half_weight * angle.sin();

    (Point(a.0 - ex, a.1 - ey), Point(b.0 + ex, b.1 + ey))
}
