#[macro_use]
extern crate glium;

pub use app::App;
pub use app::Application;
pub use color::Color;
pub use consts::*;
pub use geometry::GeometryKind;
pub use graphics::{GraphicsP2D, GraphicsP3D};
pub use processing::Processing;
pub use settings::StrokeCap;
pub use utils::*;
pub use vector::Vector2D;

use crate::traits::Renderer;

mod app;
mod color;
mod consts;
mod core;
mod geometry;
mod gl_shape;
mod graphics;
mod noise;
mod painter;
mod primitives;
mod processing;
mod settings;
mod tess;
mod traits;
mod utils;
mod vector;

pub fn new<R: Renderer + Default>() -> App<(), R> {
    App::new(())
}

pub fn with_state<R: Renderer + Default, T>(state: T) -> App<T, R> {
    App::new(state)
}
