use std::marker::PhantomData;

use glium::{glutin::surface::WindowSurface, index::NoIndices, Display, Surface};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    core::vertex::Vertex,
    geometry::{Geometry, GeometryKind, GeometryVertex},
    renderer::{Renderer, Stroke, VertexShape},
    settings::{StrokeJoin, StrokeSettings},
    shapes,
    tess::{self, tessellator::Tessellator},
    Color, Processing, StrokeCap,
};

#[derive(Default)]
pub struct GraphicsP2D {
    stroke_settings: StrokeSettings,

    vertex_shape: Option<Geometry>,
    shapes: Vec<Vertex>,
}

impl GraphicsP2D {
    pub fn background(&mut self, color: Color, width: u32, height: u32) {
        self.shapes.clear();

        let rect = shapes::rect(0.0, 0.0, width as f32, height as f32)
            .tessellate(tess::fns::gl_triangle::quad())
            .color(color);

        self.shapes.extend(rect);
    }

    pub fn point(&mut self, vertex: (f32, f32)) {
        if let Some(stroke) = self.stroke_settings.stroke {
            let point = shapes::point(vertex)
                .tessellate(tess::fns::gl_triangle::point(
                    self.stroke_settings.stroke_weight,
                    self.stroke_settings.stroke_cap,
                ))
                .color(stroke);
            self.shapes.extend(point);
        }
    }

    pub fn line(&mut self, a: (f32, f32), b: (f32, f32)) {
        if let Some(stroke) = self.stroke_settings.stroke {
            let line = shapes::line(a, b)
                .tessellate(tess::fns::gl_triangle::line(
                    self.stroke_settings.stroke_weight,
                    self.stroke_settings.stroke_cap,
                ))
                .color(stroke);
            self.shapes.extend(line);
        }
    }

    pub fn triangle(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32)) {
        if let Some(fill) = self.stroke_settings.fill {
            let triangle = shapes::triangle(a, b, c)
                .tessellate(tess::fns::gl_triangle::triangle())
                .color(fill);
            self.shapes.extend(triangle);
        }
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(fill) = self.stroke_settings.fill {
            let rect = shapes::rect(x, y, width, height)
                .tessellate(tess::fns::gl_triangle::quad())
                .color(fill);
            self.shapes.extend(rect);
        }
    }

    pub fn ellipse(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(fill) = self.stroke_settings.fill {
            let ellipse = shapes::ellipse((x, y), (width, height))
                .tessellate(tess::fns::gl_triangle::ellipse(20))
                .color(fill);
            self.shapes.extend(ellipse);
        }
    }

    pub fn ellipse_arc(&mut self, x: f32, y: f32, width: f32, height: f32, start: f32, stop: f32) {
        if let Some(fill) = self.stroke_settings.fill {
            let arc = shapes::ellipse_arc((x, y), (width, height), start, stop)
                .tessellate(tess::fns::gl_triangle::ellipse_arc(20))
                .color(fill);
            self.shapes.extend(arc);
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

impl VertexShape for GraphicsP2D {
    type Item = (f32, f32);

    fn begin_shape(&mut self, kind: GeometryKind) {
        if self.vertex_shape.is_some() {
            panic!("begin_shape() has already been called");
        }
        self.vertex_shape = Some(Geometry::new(kind));
    }

    fn vertex(&mut self, vertex: Self::Item) {
        let Some(ref mut shape) = self.vertex_shape else {
            panic!("begin_shape() has not been called");
        };
        let StrokeSettings {
            fill,
            stroke,
            stroke_weight,
            ..
        } = self.stroke_settings;

        shape.push_vertex(GeometryVertex::new(
            vertex.0,
            vertex.1,
            0.0,
            fill,
            stroke,
            stroke_weight,
        ));
    }

    fn end_shape(&mut self) {
        if self.vertex_shape.is_none() {
            panic!("begin_shape() has not been called");
        }
    }
}

impl Renderer for GraphicsP2D {
    fn shapes(&self) -> &Vec<Vertex> {
        &self.shapes
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

    shapes: Vec<Vertex>,
}

impl GraphicsP3D {
    pub fn parallelepiped(&mut self, width: f32, height: f32, depth: f32, angle: f32) {
        unimplemented!("3D shapes are not yet implemented");
    }

    pub fn rectengular_cuboid(&mut self, width: f32, height: f32, depth: f32) {
        unimplemented!("3D shapes are not yet implemented");
    }

    pub fn cube(&mut self, size: f32) {
        unimplemented!("3D shapes are not yet implemented");
    }

    pub fn sphere(&mut self, radius: f32) {
        unimplemented!("3D shapes are not yet implemented");
    }
}

impl Renderer for GraphicsP3D {
    fn shapes(&self) -> &Vec<Vertex> {
        &self.shapes
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
