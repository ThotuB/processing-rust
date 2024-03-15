use glium::index::PrimitiveType;

use crate::{
    geometry::{Geometry, GeometryKind, GeometryVertex},
    gl_shape::LazyGlShape,
    primitives::shapes_2d::{Ellipse, EllipseArc, Line, Point, Quad, Triangle},
    settings::{StrokeJoin, StrokeSettings},
    traits::{BeginShape, Renderer, Stroke},
    Color, StrokeCap,
};

#[derive(Default)]
pub struct GraphicsP2D {
    stroke_settings: StrokeSettings,

    geometry: Option<Geometry>,
    shapes: Vec<LazyGlShape>,
}

impl GraphicsP2D {
    pub fn background(&mut self, color: Color, width: u32, height: u32) {
        self.shapes.clear();

        let rect = Quad::rect(0.0, 0.0, width as f32, height as f32)
            .tessellate_fill()
            .color(color);

        self.shapes
            .push(LazyGlShape::new(rect, PrimitiveType::TrianglesList));
    }

    pub fn point(&mut self, vertex: (f32, f32)) {
        if let Some(stroke) = self.stroke_settings.stroke {
            let point = Point::new(vertex)
                .tessellate_fill(
                    self.stroke_settings.stroke_weight,
                    self.stroke_settings.stroke_cap,
                )
                .color(stroke);
            self.shapes
                .push(LazyGlShape::new(point, PrimitiveType::TrianglesList));
        }
    }

    pub fn line(&mut self, a: (f32, f32), b: (f32, f32)) {
        if let Some(stroke) = self.stroke_settings.stroke {
            let line = Line::new(a, b)
                .tessellate_fill(
                    self.stroke_settings.stroke_weight,
                    self.stroke_settings.stroke_cap,
                )
                .color(stroke);
            self.shapes
                .push(LazyGlShape::new(line, PrimitiveType::TrianglesList));
        }
    }

    pub fn triangle(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32)) {
        if let Some(fill) = self.stroke_settings.fill {
            let triangle = Triangle::new(a, b, c).tessellate_fill().color(fill);
            self.shapes
                .push(LazyGlShape::new(triangle, PrimitiveType::TrianglesList));
        }
        if let Some(stroke) = self.stroke_settings.stroke {
            let triangle = Triangle::new(a, b, c)
                .tessellate_stroke(self.stroke_settings.stroke_weight)
                .color(stroke);
            self.shapes
                .push(LazyGlShape::new(triangle, PrimitiveType::TrianglesList));
        }
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(fill) = self.stroke_settings.fill {
            let rect = Quad::rect(x, y, width, height)
                .tessellate_fill()
                .color(fill);
            self.shapes
                .push(LazyGlShape::new(rect, PrimitiveType::TrianglesList));
        }
        if let Some(stroke) = self.stroke_settings.stroke {
            let rect = Quad::rect(x, y, width, height)
                .tessellate_stroke(self.stroke_settings.stroke_weight)
                .color(stroke);
            self.shapes
                .push(LazyGlShape::new(rect, PrimitiveType::TrianglesList));
        }
    }

    pub fn ellipse(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(fill) = self.stroke_settings.fill {
            let ellipse = Ellipse::new((x, y), (width, height))
                .tessellate_fill(20)
                .color(fill);
            self.shapes
                .push(LazyGlShape::new(ellipse, PrimitiveType::TrianglesList));
        }
        if let Some(stroke) = self.stroke_settings.stroke {
            let ellipse = Ellipse::new((x, y), (width, height))
                .tessellate_stroke(self.stroke_settings.stroke_weight, 20)
                .color(stroke);
            self.shapes
                .push(LazyGlShape::new(ellipse, PrimitiveType::TrianglesList));
        }
    }

    pub fn ellipse_arc(&mut self, x: f32, y: f32, width: f32, height: f32, start: f32, stop: f32) {
        if let Some(fill) = self.stroke_settings.fill {
            let arc = EllipseArc::new((x, y), (width, height), start, stop)
                .tessellate_fill(20)
                .color(fill);
            self.shapes
                .push(LazyGlShape::new(arc, PrimitiveType::TrianglesList));
        }
        if let Some(stroke) = self.stroke_settings.stroke {
            let arc = EllipseArc::new((x, y), (width, height), start, stop)
                .tessellate_stroke(self.stroke_settings.stroke_weight, 20)
                .color(stroke);
            self.shapes
                .push(LazyGlShape::new(arc, PrimitiveType::TrianglesList));
        }
    }

    pub fn circle(&mut self, x: f32, y: f32, diameter: f32) {
        self.ellipse(x, y, diameter, diameter);
    }

    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start: f32, stop: f32) {
        self.ellipse_arc(x, y, radius, radius, start, stop);
    }

    pub fn square(&mut self, x: f32, y: f32, size: f32) {
        self.rect(x, y, size, size);
    }
}

impl BeginShape for GraphicsP2D {
    type Item = (f32, f32);

    fn begin_shape(&mut self, kind: GeometryKind) {
        if self.geometry.is_some() {
            panic!("begin_shape() has already been called");
        }
        self.geometry = Some(Geometry::new(kind));
    }

    fn vertex(&mut self, vertex: Self::Item) {
        let Some(ref mut geometry) = self.geometry else {
            panic!("begin_shape() has not been called");
        };
        let StrokeSettings {
            fill,
            stroke,
            stroke_weight,
            ..
        } = self.stroke_settings;

        geometry.push_vertex(GeometryVertex::new(
            vertex.0,
            vertex.1,
            0.0,
            fill,
            stroke,
            stroke_weight,
        ));
    }

    fn end_shape(&mut self) {
        let Some(geometry) = self.geometry.take() else {
            panic!("begin_shape() has not been called");
        };
        let lazy_shape = geometry.tessellate();
        self.shapes.push(lazy_shape);
    }
}

impl Renderer for GraphicsP2D {
    fn shapes(&mut self) -> Vec<LazyGlShape> {
        self.shapes.drain(..).collect()
    }
}

impl Stroke for GraphicsP2D {
    fn stroke(&mut self, color: Option<Color>) {
        self.stroke_settings.stroke = color;
    }

    fn stroke_weight(&mut self, weight: f32) {
        self.stroke_settings.stroke_weight = weight;
    }

    fn stroke_cap(&mut self, cap: StrokeCap) {
        self.stroke_settings.stroke_cap = cap;
    }

    fn stroke_join(&mut self, join: StrokeJoin) {
        self.stroke_settings.stroke_join = join;
    }

    fn fill(&mut self, color: Option<Color>) {
        self.stroke_settings.fill = color;
    }
}

#[derive(Default)]
pub struct GraphicsP3D {
    stroke_settings: StrokeSettings,

    shapes: Vec<LazyGlShape>,
}

impl GraphicsP3D {
    pub fn parallelepiped(&mut self, _width: f32, _height: f32, _depth: f32, _angle: f32) {
        unimplemented!("3D shapes are not yet implemented");
    }

    pub fn rectengular_cuboid(&mut self, _width: f32, _height: f32, _depth: f32) {
        unimplemented!("3D shapes are not yet implemented");
    }

    pub fn cube(&mut self, _size: f32) {
        unimplemented!("3D shapes are not yet implemented");
    }

    pub fn sphere(&mut self, _radius: f32) {
        unimplemented!("3D shapes are not yet implemented");
    }
}

impl Renderer for GraphicsP3D {
    fn shapes(&mut self) -> Vec<LazyGlShape> {
        self.shapes.drain(..).collect()
    }
}

impl Stroke for GraphicsP3D {
    fn stroke(&mut self, color: Option<Color>) {
        self.stroke_settings.stroke = color;
    }

    fn stroke_weight(&mut self, weight: f32) {
        self.stroke_settings.stroke_weight = weight;
    }

    fn stroke_cap(&mut self, cap: StrokeCap) {
        self.stroke_settings.stroke_cap = cap;
    }

    fn stroke_join(&mut self, join: StrokeJoin) {
        self.stroke_settings.stroke_join = join;
    }

    fn fill(&mut self, color: Option<Color>) {
        self.stroke_settings.fill = color;
    }
}
