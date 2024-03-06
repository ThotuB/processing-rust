use std::ops::{Deref, DerefMut};

use glium::{
    backend::glutin::SimpleWindowBuilder, glutin::surface::WindowSurface, index::NoIndices,
    program, Display, Surface,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::Window,
};

use crate::{
    geometry::GeometryKind,
    graphics::{GraphicsP2D, GraphicsP3D},
    settings::{StrokeCap, StrokeJoin, WindowSettings},
    traits::{BeginShape, Renderer, Stroke},
    Color,
};

#[derive(Debug)]
pub struct Processing<S, R: Renderer> {
    pub state: S,
    g: R,

    window_settings: WindowSettings,
    is_loop: bool,

    frame_rate: u32,
    frame_count: u32,

    setup: fn(&mut Processing<S, R>),
    draw: fn(&mut Processing<S, R>),
}

impl<S, R: Renderer + Default> Processing<S, R> {
    pub fn new(
        setup: fn(&mut Processing<S, R>),
        draw: fn(&mut Processing<S, R>),
        state: S,
    ) -> Processing<S, R> {
        Processing {
            state,
            g: R::default(),
            window_settings: WindowSettings::default(),
            is_loop: true,
            frame_rate: 1,
            frame_count: 0,
            setup,
            draw,
        }
    }
}

impl<S, R: Renderer> Processing<S, R> {
    // window configuration
    pub fn size(&mut self, width: u32, height: u32) {
        self.window_settings.width = width;
        self.window_settings.height = height;
    }

    pub fn title(&mut self, title: &str) {
        self.window_settings.title = title.to_string();
    }

    pub fn width(&self) -> u32 {
        self.window_settings.width
    }

    pub fn height(&self) -> u32 {
        self.window_settings.height
    }
}

impl<S, R: Renderer + Stroke> Processing<S, R> {
    // color settings
    pub fn stroke(&mut self, color: Color) {
        self.g.stroke(Some(color));
    }

    pub fn stroke_weight(&mut self, weight: f32) {
        self.g.stroke_weight(weight);
    }

    pub fn stroke_cap(&mut self, cap: StrokeCap) {
        self.g.stroke_cap(cap);
    }

    pub fn stroke_join(&mut self, join: StrokeJoin) {
        self.g.stroke_join(join);
    }

    pub fn fill(&mut self, color: Color) {
        self.g.fill(Some(color));
    }

    pub fn no_stroke(&mut self) {
        self.g.stroke(None);
    }

    pub fn no_fill(&mut self) {
        self.g.fill(None);
    }
}

impl<S, R: Renderer> Processing<S, R> {
    // structure
    pub fn r#loop(&mut self) {
        self.is_loop = true;
    }

    pub fn no_loop(&mut self) {
        self.is_loop = false;
    }

    // run
    pub fn run(mut self) -> anyhow::Result<()> {
        (self.setup)(&mut self);

        let event_loop = EventLoop::new().unwrap();
        let (window, display) = SimpleWindowBuilder::new()
            .with_inner_size(self.width(), self.height())
            .with_title(&self.window_settings.title)
            .build(&event_loop);

        let program = glium::Program::from_source(
            &display,
            include_str!("shaders/vertex.glsl"),
            include_str!("shaders/fragment.glsl"),
            None,
        )
        .unwrap();

        self.draw_frame(&display, &program);

        let frame_time = std::time::Duration::from_secs_f32(1.0 / self.frame_rate as f32);

        let _ = event_loop.run(move |event, window_target| {
            let now = std::time::Instant::now();

            self.event_handler(event, window_target);

            self.handle_draw(&display, &program);

            let elapsed = now.elapsed();
            if elapsed < frame_time {
                std::thread::sleep(frame_time - elapsed);
            }
        });

        Ok(())
    }

    fn draw_shapes(
        &mut self,
        target: &mut glium::Frame,
        display: &glium::Display<WindowSurface>,
        program: &glium::Program,
    ) {
        let uniforms = uniform! {
            projection: [
                [2.0 / self.width() as f32, 0.0, 0.0, 0.0],
                [0.0, 2.0 / self.height() as f32, 0.0, 0.0],
                [0.0, 0.0, 2.0 / 100.0, 0.0], // TODO depth
                [-1.0, -1.0, -1.0, 1.0]
            ],
        };

        let params = glium::DrawParameters {
            blend: glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
                },
                constant_value: (0.0, 0.0, 0.0, 0.0),
            },
            ..Default::default()
        };

        let lazy_gl_shapes = self.g.shapes();
        let gl_shapes = lazy_gl_shapes
            .into_iter()
            .map(|shape| shape.run())
            .collect::<Vec<_>>();

        for gl_shape in &gl_shapes {
            let vertex_buffer = glium::VertexBuffer::new(display, &gl_shape.vertices).unwrap();

            target
                .draw(
                    &vertex_buffer,
                    NoIndices(gl_shape.index_type),
                    program,
                    &uniforms,
                    &params,
                )
                .unwrap();
        }
    }

    fn draw_frame(&mut self, display: &glium::Display<WindowSurface>, program: &glium::Program) {
        let mut target = display.draw();

        self.draw_shapes(&mut target, display, program);

        target.finish().unwrap();
    }

    fn handle_draw(&mut self, display: &glium::Display<WindowSurface>, program: &glium::Program) {
        if !self.is_loop {
            return;
        }

        (self.draw)(self);

        self.draw_frame(display, program);
    }
}

impl<S, R: Renderer + BeginShape> Processing<S, R> {
    pub fn begin_shape(&mut self, kind: GeometryKind) {
        self.g.begin_shape(kind);
    }

    pub fn vertex(&mut self, vertex: <R as BeginShape>::Item) {
        self.g.vertex(vertex);
    }

    pub fn end_shape(&mut self) {
        self.g.end_shape();
    }
}

impl<S> Processing<S, GraphicsP2D> {
    // shapes
    pub fn background(&mut self, color: Color) {
        self.g.background(color, self.width(), self.height());
    }

    pub fn point(&mut self, x: f32, y: f32) {
        self.g.point((x, y));
    }

    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.g.line((x1, y1), (x2, y2));
    }

    pub fn triangle(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.g.triangle((x1, y1), (x2, y2), (x3, y3));
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.g.rect(x, y, width, height);
    }

    pub fn ellipse(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.g.ellipse(x, y, width, height);
    }

    pub fn circle(&mut self, x: f32, y: f32, diameter: f32) {
        self.g.circle(x, y, diameter);
    }

    pub fn arc(&mut self, x: f32, y: f32, radius: f32, start: f32, stop: f32) {
        self.g.arc(x, y, radius, start, stop);
    }

    pub fn square(&mut self, x: f32, y: f32, size: f32) {
        self.g.square(x, y, size);
    }
}

impl<S> Processing<S, GraphicsP3D> {
    pub fn parallelepiped(&mut self, width: f32, height: f32, depth: f32, angle: f32) {
        self.g.parallelepiped(width, height, depth, angle);
    }

    pub fn rectengular_cuboid(&mut self, width: f32, height: f32, depth: f32) {
        self.g.rectengular_cuboid(width, height, depth);
    }

    pub fn cube(&mut self, size: f32) {
        self.g.cube(size);
    }

    pub fn sphere(&mut self, radius: f32) {
        self.g.sphere(radius);
    }
}

// impl<S, R: Renderer> Deref for Processing<S, R> {
//     type Target = R;
//
//     fn deref(&self) -> &Self::Target {
//         &self.g
//     }
// }
//
// impl<S, R: Renderer> DerefMut for Processing<S, R> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.g
//     }
// }

impl<S, R: Renderer> Processing<S, R> {
    fn event_handler(&mut self, event: Event<()>, window_target: &EventLoopWindowTarget<()>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                // WindowEvent::ActivationTokenDone { serial, token } => todo!(),
                // WindowEvent::Resized(_) => todo!(),
                // WindowEvent::Moved(_) => todo!(),
                // WindowEvent::Destroyed => todo!(),
                // WindowEvent::DroppedFile(_) => todo!(),
                // WindowEvent::HoveredFile(_) => todo!(),
                // WindowEvent::HoveredFileCancelled => todo!(),
                // WindowEvent::Focused(_) => todo!(),
                // WindowEvent::KeyboardInput {
                //     device_id,
                //     event,
                //     is_synthetic,
                // } => todo!(),
                // WindowEvent::ModifiersChanged(_) => todo!(),
                // WindowEvent::Ime(_) => todo!(),
                // WindowEvent::CursorMoved {
                //     device_id,
                //     position,
                // } => todo!(),
                // WindowEvent::CursorEntered { device_id } => todo!(),
                // WindowEvent::CursorLeft { device_id } => todo!(),
                // WindowEvent::MouseWheel {
                //     device_id,
                //     delta,
                //     phase,
                // } => todo!(),
                // WindowEvent::MouseInput {
                //     device_id,
                //     state,
                //     button,
                // } => todo!(),
                // WindowEvent::TouchpadMagnify {
                //     device_id,
                //     delta,
                //     phase,
                // } => todo!(),
                // WindowEvent::SmartMagnify { device_id } => todo!(),
                // WindowEvent::TouchpadRotate {
                //     device_id,
                //     delta,
                //     phase,
                // } => todo!(),
                // WindowEvent::TouchpadPressure {
                //     device_id,
                //     pressure,
                //     stage,
                // } => todo!(),
                // WindowEvent::AxisMotion {
                //     device_id,
                //     axis,
                //     value,
                // } => todo!(),
                // WindowEvent::Touch(_) => todo!(),
                // WindowEvent::ScaleFactorChanged {
                //     scale_factor,
                //     inner_size_writer,
                // } => todo!(),
                // WindowEvent::ThemeChanged(_) => todo!(),
                // WindowEvent::Occluded(_) => todo!(),
                // WindowEvent::RedrawRequested => todo!(),
                _ => (),
            },
            // Event::NewEvents(_) => todo!(),
            // Event::DeviceEvent { device_id, event } => todo!(),
            // Event::UserEvent(_) => todo!(),
            // Event::Suspended => todo!(),
            // Event::Resumed => todo!(),
            // Event::AboutToWait => todo!(),
            // Event::LoopExiting => todo!(),
            // Event::MemoryWarning => todo!(),
            _ => (),
        };
    }
}

impl<T, R: Renderer> Drop for Processing<T, R> {
    fn drop(&mut self) {
        println!("Processing is dropped");
    }
}
