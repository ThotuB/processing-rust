use std::marker::PhantomData;

use glium::{glutin::surface::WindowSurface, index::NoIndices, Display, Surface};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    core::vertex::Vertex,
    renderer::Renderer,
    settings::StrokeJoin,
    shapes,
    tess::{self, tessellator::Tessellator},
    Color, Processing, StrokeCap, P2D, P3D,
};

#[derive(Debug)]
pub struct Graphics<R: Renderer> {
    _renderer: PhantomData<R>,
    fill: Option<Color>,

    stroke: Option<Color>,
    stroke_weight: f32,
    stroke_cap: StrokeCap,
    stroke_join: StrokeJoin,

    shapes: Vec<Vertex>,
}

impl<R: Renderer> Graphics<R> {
    pub fn new() -> Self {
        Graphics {
            _renderer: PhantomData,
            fill: Some(Color::rgb(255, 255, 255)),
            stroke: Some(Color::rgb(0, 0, 0)),
            stroke_weight: 1.0,
            stroke_cap: StrokeCap::Butt,
            stroke_join: StrokeJoin::Miter,

            shapes: Vec::new(),
        }
    }

    pub fn shapes(&self) -> &Vec<Vertex> {
        &self.shapes
    }

    pub fn depth(&self) -> f32 {
        100.0
    }

    pub fn stroke(&mut self, color: Option<Color>) {
        self.stroke = color;
    }

    pub fn stroke_weight(&mut self, weight: f32) {
        self.stroke_weight = weight;
    }

    pub fn stroke_cap(&mut self, cap: StrokeCap) {
        self.stroke_cap = cap;
    }

    pub fn stroke_join(&mut self, join: StrokeJoin) {
        self.stroke_join = join;
    }

    pub fn fill(&mut self, color: Option<Color>) {
        self.fill = color;
    }
}

impl Graphics<P2D> {
    pub fn background(&mut self, color: Color, width: u32, height: u32) {
        self.shapes.clear();

        let rect = shapes::rect(0.0, 0.0, width as f32, height as f32)
            .tessellate(tess::fns::gl_triangle::quad())
            .color(color);

        self.shapes.extend(rect);
    }

    pub fn point(&mut self, vertex: (f32, f32)) {
        if let Some(stroke) = self.stroke {
            let point = shapes::point(vertex)
                .tessellate(tess::fns::gl_triangle::point(
                    self.stroke_weight,
                    self.stroke_cap,
                ))
                .color(stroke);
            self.shapes.extend(point);
        }
    }

    pub fn line(&mut self, a: (f32, f32), b: (f32, f32)) {
        if let Some(stroke) = self.stroke {
            let line = shapes::line(a, b)
                .tessellate(tess::fns::gl_triangle::line(
                    self.stroke_weight,
                    self.stroke_cap,
                ))
                .color(stroke);
            self.shapes.extend(line);
        }
    }

    pub fn triangle(&mut self, a: (f32, f32), b: (f32, f32), c: (f32, f32)) {
        if let Some(fill) = self.fill {
            let triangle = shapes::triangle(a, b, c)
                .tessellate(tess::fns::gl_triangle::triangle())
                .color(fill);
            self.shapes.extend(triangle);
        }
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(fill) = self.fill {
            let rect = shapes::rect(x, y, width, height)
                .tessellate(tess::fns::gl_triangle::quad())
                .color(fill);
            self.shapes.extend(rect);
        }
    }

    pub fn ellipse(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(fill) = self.fill {
            let ellipse = shapes::ellipse((x, y), (width, height))
                .tessellate(tess::fns::gl_triangle::ellipse(20))
                .color(fill);
            self.shapes.extend(ellipse);
        }
    }

    pub fn ellipse_arc(&mut self, x: f32, y: f32, width: f32, height: f32, start: f32, stop: f32) {
        if let Some(fill) = self.fill {
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

impl Graphics<P3D> {
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
