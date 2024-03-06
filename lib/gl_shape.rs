use glium::index;

use crate::core::vertex::Vertex;

pub struct GlShape {
    pub vertices: Vec<Vertex>,
    pub index_type: index::PrimitiveType,
}

pub struct LazyGlShape {
    shape: Box<dyn Iterator<Item = Vertex>>,
    index_type: index::PrimitiveType,
}

impl LazyGlShape {
    pub fn new(
        shape: impl Iterator<Item = Vertex> + 'static,
        index_type: index::PrimitiveType,
    ) -> LazyGlShape {
        LazyGlShape {
            shape: Box::new(shape),
            index_type,
        }
    }

    pub fn run(self) -> GlShape {
        let vertices = self.shape.collect();
        GlShape {
            vertices,
            index_type: self.index_type,
        }
    }
}
