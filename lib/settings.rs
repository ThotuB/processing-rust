use crate::Color;

#[derive(Debug)]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

impl Default for WindowSettings {
    fn default() -> WindowSettings {
        WindowSettings {
            width: 800,
            height: 600,
            title: "Processing".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum StrokeCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Copy, Clone)]
pub enum StrokeJoin {
    Miter,
    Round,
    Bevel,
}

#[derive(Debug, Copy, Clone)]
pub struct StrokeSettings {
    pub color: Color,
    pub weight: f32,
    pub cap: StrokeCap,
    pub join: StrokeJoin,
}
