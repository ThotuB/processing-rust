#[macro_use]
extern crate glium;

pub use color::Color;
pub use consts::*;
pub use processing::Processing;
use processing_builder::ProcessingBuilder;
pub use renderer::{P2D, P3D};
pub use settings::StrokeCap;

use crate::renderer::Renderer;

mod color;
mod consts;
mod core;
mod graphics;
mod processing;
mod processing_builder;
mod renderer;
mod settings;
mod shapes;
mod tess;

pub fn new<R: Renderer>() -> ProcessingBuilder<(), R> {
    ProcessingBuilder::new(())
}

pub fn with_state<R: Renderer, T>(state: T) -> ProcessingBuilder<T, R> {
    ProcessingBuilder::new(state)
}