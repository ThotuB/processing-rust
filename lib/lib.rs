#[macro_use]
extern crate glium;

pub use color::Color;
pub use consts::*;
pub use geometry::GeometryKind;
pub use graphics::{GraphicsP2D, GraphicsP3D};
pub use processing::Processing;
use processing_builder::ProcessingBuilder;
pub use settings::StrokeCap;
pub use utils::*;
pub use vector::Vector2D;

use crate::traits::Renderer;

mod color;
mod consts;
mod core;
mod geometry;
mod gl_shape;
mod graphics;
mod primitives;
mod processing;
mod processing_builder;
mod settings;
mod tess;
mod traits;
mod utils;
mod vector;

pub fn new<R: Renderer + Default>() -> ProcessingBuilder<(), R> {
    ProcessingBuilder::new(())
}

pub fn with_state<R: Renderer + Default, T>(state: T) -> ProcessingBuilder<T, R> {
    ProcessingBuilder::new(state)
}
