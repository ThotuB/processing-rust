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

#[derive(Debug)]
pub struct StrokeSettings {
    pub fill: Option<Color>,

    pub stroke: Option<Color>,
    pub stroke_weight: f32,
    pub stroke_cap: StrokeCap,
    pub stroke_join: StrokeJoin,
}

impl Default for StrokeSettings {
    fn default() -> Self {
        StrokeSettings {
            fill: Some(Color::rgb(255, 255, 255)),
            stroke: Some(Color::rgb(0, 0, 0)),
            stroke_weight: 1.0,
            stroke_cap: StrokeCap::Butt,
            stroke_join: StrokeJoin::Miter,
        }
    }
}
