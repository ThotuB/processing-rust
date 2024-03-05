use glium::index;

use crate::core::vertex::Vertex;

pub struct GlShape {
    vertices: Vec<Vertex>,
    index_type: index::PrimitiveType,
}

impl GlShape {
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn index_type(&self) -> index::PrimitiveType {
        self.index_type
    }
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
