#[macro_use]
extern crate glium;

pub use color::Color;
pub use consts::*;
pub use geometry::GeometryKind;
pub use processing::Processing;
use processing_builder::ProcessingBuilder;
pub use settings::StrokeCap;

use crate::renderer::Renderer;

mod color;
mod consts;
mod core;
mod geometry;
mod gl_shape;
mod graphics;
mod primitives;
mod processing;
mod processing_builder;
mod renderer;
mod settings;
mod tess;
mod utils;

pub fn new<R: Renderer + Default>() -> ProcessingBuilder<(), R> {
    ProcessingBuilder::new(())
}

pub fn with_state<R: Renderer + Default, T>(state: T) -> ProcessingBuilder<T, R> {
    ProcessingBuilder::new(state)
}
