use crate::{processing::Processing, traits::Renderer};

#[derive(Debug)]
pub struct ProcessingBuilder<T, R: Renderer + Default> {
    state: T,
    setup: fn(&mut Processing<T, R>),
    draw: fn(&mut Processing<T, R>),
}

impl<T, R: Renderer + Default> ProcessingBuilder<T, R> {
    pub(crate) fn new(state: T) -> ProcessingBuilder<T, R> {
        ProcessingBuilder {
            state,
            setup: |_| {},
            draw: |_| {},
        }
    }

    pub fn setup(mut self, f: fn(&mut Processing<T, R>)) -> ProcessingBuilder<T, R> {
        self.setup = f;
        self
    }

    pub fn draw(mut self, f: fn(&mut Processing<T, R>)) -> ProcessingBuilder<T, R> {
        self.draw = f;
        self
    }

    pub fn build(self) -> Processing<T, R> {
        Processing::new(self.setup, self.draw, self.state)
    }
}
