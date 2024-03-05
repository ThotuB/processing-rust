pub fn lerp<T>(from: T, to: T, t: f32) -> T
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + Copy,
{
    from + (to - from) * t
}
