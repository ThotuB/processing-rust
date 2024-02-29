use std::marker::PhantomData;

use glium::{glutin::surface::WindowSurface, index::NoIndices, Display, Surface};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    core::vertex::Vertex,
    renderer::Renderer,
    settings::StrokeJoin,
    shapes,
    tess::{self, tessellator::Tessellator},
    Color, Processing, StrokeCap,
};

#[derive(Debug)]
pub struct Graphics {
    fill: Option<Color>,

    stroke: Option<Color>,
    stroke_weight: f32,
    stroke_cap: StrokeCap,
    stroke_join: StrokeJoin,

    shapes: Vec<Vertex>,
}

impl Default for Graphics {
    fn default() -> Self {
        Graphics {
            fill: Some(Color::rgb(255, 255, 255)),
            stroke: Some(Color::rgb(0, 0, 0)),
            stroke_weight: 1.0,
            stroke_cap: StrokeCap::Butt,
            stroke_join: StrokeJoin::Miter,

            shapes: Vec::new(),
        }
    }
}

impl Graphics {
    pub fn depth(&self) -> f32 {
        100.0
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    pub fn extend(&mut self, vertices: Vec<Vertex>) {
        self.shapes.extend(vertices);
    }
}

impl Renderer for Graphics {
    fn shapes(&self) -> &Vec<Vertex> {
        &self.shapes
    }

    fn stroke(&mut self, color: Option<Color>) {
        self.stroke = color;
    }

    fn stroke_weight(&mut self, weight: f32) {
        self.stroke_weight = weight;
    }

    fn stroke_cap(&mut self, cap: StrokeCap) {
        self.stroke_cap = cap;
    }

    fn stroke_join(&mut self, join: StrokeJoin) {
        self.stroke_join = join;
    }

    fn fill(&mut self, color: Option<Color>) {
        self.fill = color;
    }
}

pub struct GraphicsP2D {
    graphics: Graphics,
}

impl Default for GraphicsP2D {
    fn default() -> Self {
        GraphicsP2D {
            graphics: Graphics::default(),
        }
    }
}

impl GraphicsP2D {
    pub fn background(&mut self, color: Color, width: u32, height: u32) {
        self.graphics.clear();

        let rect = shapes::rect(0.0, 0.0, width as f32, height as f32)
            .tessellate(tess::fns::gl_triangle::quad())
            .color(color);

        self.graphics.extend(rect);
    }

    pub fn point(&mut self, vertex: (f32, f32)) {
        if let Some(stroke) = self.graphics.stroke {
            let point = shapes::point(vertex)
                .tessellate(tess::fns::gl_triangle::point(
                    self.graphics.stroke_weight,
                    self.graphics.stroke_cap,
                ))
                .color(stroke);
            self.graphics.extend(point);
        }
    }

    pub fn line(&mut self, a: (f32, f32), b: (f32, f32)) {
        if let Some(stroke) = self.graphics.stroke {
            let line = shapes::line(a, b)
                .tessellate(tess::fns::gl_triangle::line(
                    self.graphics.stroke_weight,
                    self.graphics.stroke_cap,
                ))
                .color(stroke);
            self.graphics.extend(line);
        }
    }

    pub fn triangle(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32)) {
        if let Some(fill) = self.graphics.fill {
            let triangle = shapes::triangle(a, b, c)
                .tessellate(tess::fns::gl_triangle::triangle())
                .color(fill);
            self.graphics.extend(triangle);
        }
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(fill) = self.graphics.fill {
            let rect = shapes::rect(x, y, width, height)
                .tessellate(tess::fns::gl_triangle::quad())
                .color(fill);
            self.graphics.extend(rect);
        }
    }

    pub fn ellipse(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(fill) = self.graphics.fill {
            let ellipse = shapes::ellipse((x, y), (width, height))
                .tessellate(tess::fns::gl_triangle::ellipse(20))
                .color(fill);
            self.graphics.extend(ellipse);
        }
    }

    pub fn ellipse_arc(&mut self, x: f32, y: f32, width: f32, height: f32, start: f32, stop: f32) {
        if let Some(fill) = self.graphics.fill {
            let arc = shapes::ellipse_arc((x, y), (width, height), start, stop)
                .tessellate(tess::fns::gl_triangle::ellipse_arc(20))
                .color(fill);
            self.graphics.extend(arc);
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

impl Renderer for GraphicsP2D {
    fn shapes(&self) -> &Vec<Vertex> {
        &self.graphics.shapes
    }

    fn stroke(&mut self, color: Option<Color>) {
        self.graphics.stroke(color);
    }

    fn stroke_weight(&mut self, weight: f32) {
        self.graphics.stroke_weight(weight);
    }

    fn stroke_cap(&mut self, cap: StrokeCap) {
        self.graphics.stroke_cap(cap);
    }

    fn stroke_join(&mut self, join: StrokeJoin) {
        self.graphics.stroke_join(join);
    }

    fn fill(&mut self, color: Option<Color>) {
        self.graphics.fill(color);
    }
}

pub struct GraphicsP3D {
    graphics: Graphics,
}

impl Default for GraphicsP3D {
    fn default() -> Self {
        GraphicsP3D {
            graphics: Graphics::default(),
        }
    }
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
        &self.graphics.shapes
    }

    fn stroke(&mut self, color: Option<Color>) {
        self.graphics.stroke(color);
    }

    fn stroke_weight(&mut self, weight: f32) {
        self.graphics.stroke_weight(weight);
    }

    fn stroke_cap(&mut self, cap: StrokeCap) {
        self.graphics.stroke_cap(cap);
    }

    fn stroke_join(&mut self, join: StrokeJoin) {
        self.graphics.stroke_join(join);
    }

    fn fill(&mut self, color: Option<Color>) {
        self.graphics.fill(color);
    }
}
