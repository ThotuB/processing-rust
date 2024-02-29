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
    graphics::{Graphics, GraphicsP2D, GraphicsP3D},
    renderer::{Renderer, Stroke},
    settings::{StrokeCap, StrokeJoin, WindowSettings},
    Color,
};

#[derive(Debug)]
pub struct Processing<T, R: Renderer> {
    pub state: T,
    g: R,

    window_settings: WindowSettings,
    is_loop: bool,

    pub setup: fn(&mut Processing<T, R>),
    pub draw: fn(&mut Processing<T, R>),
}

impl<T, R: Renderer + Default> Processing<T, R> {
    pub fn new(
        setup: fn(&mut Processing<T, R>),
        draw: fn(&mut Processing<T, R>),
        state: T,
    ) -> Processing<T, R> {
        Processing {
            state,
            g: R::default(),
            window_settings: WindowSettings::default(),
            is_loop: true,
            setup,
            draw,
        }
    }
}

impl<T, R: Renderer> Processing<T, R> {
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

impl<T, R: Renderer + Stroke> Processing<T, R> {
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

        let _ = event_loop.run(move |event, window_target| {
            self.event_handler(event, window_target);

            self.handle_draw(&display, &program);
        });

        Ok(())
    }

    fn draw_shapes(
        &self,
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

        let shapes = self.g.shapes();
        let vertex_buffer = glium::VertexBuffer::new(display, shapes).unwrap();

        target
            .draw(
                &vertex_buffer,
                NoIndices(glium::index::PrimitiveType::TrianglesList),
                program,
                &uniforms,
                &params,
            )
            .unwrap();
    }

    fn draw_frame(&self, display: &glium::Display<WindowSurface>, program: &glium::Program) {
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

impl<T> Processing<T, GraphicsP2D> {
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

impl<T> Processing<T, GraphicsP3D> {
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

impl<T, R: Renderer> Processing<T, R> {
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
