

use glium::{
    index::NoIndices, Surface,
};
use image::Rgba;
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::{
    geometry::GeometryKind,
    graphics::{GraphicsP2D, GraphicsP3D},
    painter::Painter,
    settings::{StrokeCap, StrokeJoin, WindowSettings},
    traits::{BeginShape, Renderer, Stroke},
    Color,
};

pub type SetupFn<S, R> = Box<dyn Fn(&mut Processing<S, R>)>;
pub type DrawFn<S, R> = fn(&mut Processing<S, R>);
pub type MouseClickedFn<S, R> = fn(&mut Processing<S, R>, MouseButton);
pub type MouseMovedFn<S, R> = fn(&mut Processing<S, R>, f32, f32);
pub type KeyPressedFn<S, R> = fn(&mut Processing<S, R>, KeyCode);

pub struct Processing<S, R: Renderer> {
    pub state: S,
    g: R,

    window_settings: WindowSettings,
    is_loop: bool,

    frame_rate: u32,
    frame_count: u32,

    draw: Option<DrawFn<S, R>>,
    mouse_clicked: Option<MouseClickedFn<S, R>>,
    mouse_moved: Option<MouseMovedFn<S, R>>,
    key_pressed: Option<KeyPressedFn<S, R>>,

    painter: Painter,
}

impl<S, R: Renderer + Default> Processing<S, R> {
    pub(crate) fn new(
        state: S,
        window_settings: WindowSettings,
        painter: Painter,
        draw: Option<DrawFn<S, R>>,
        mouse_clicked: Option<MouseClickedFn<S, R>>,
        mouse_moved: Option<MouseMovedFn<S, R>>,
        key_pressed: Option<KeyPressedFn<S, R>>,
    ) -> Processing<S, R> {
        Processing {
            state,
            g: R::default(),
            window_settings,
            is_loop: true,
            frame_rate: 1,
            frame_count: 0,
            draw,
            mouse_clicked,
            mouse_moved,
            key_pressed,
            painter,
        }
    }
}

impl<S, R: Renderer> Processing<S, R> {
    // window configuration
    //     pub fn size(&mut self, width: u32, height: u32) {
    //         self.window_settings.width = width;
    //         self.window_settings.height = height;
    //     }
    //
    pub fn title(&mut self, title: &str) {
        self.window_settings.title = title.to_string();
        self.painter.window.set_title(title);
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

    pub fn redraw(&mut self) {
        self.painter.window.request_redraw();
    }

    pub fn screenshot(&self, path: &str) -> anyhow::Result<()> {
        let image = self
            .painter
            .display
            .read_front_buffer::<glium::texture::RawImage2d<'_, u8>>()?;
        let image: image::ImageBuffer<Rgba<u8>, Vec<u8>> =
            image::ImageBuffer::from_raw(image.width, image.height, image.data.into_owned())
                .ok_or(anyhow::anyhow!("Error reading image"))?;
        let image = image::DynamicImage::ImageRgba8(image).flipv();
        image.save(path)?;

        Ok(())
    }

    fn draw_shapes(&mut self, target: &mut glium::Frame) -> anyhow::Result<()> {
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
            let vertex_buffer =
                glium::VertexBuffer::new(&self.painter.display, &gl_shape.vertices)?;

            target.draw(
                &vertex_buffer,
                NoIndices(gl_shape.index_type),
                &self.painter.program,
                &uniforms,
                &params,
            )?;
        }

        Ok(())
    }

    fn draw_frame(&mut self) -> anyhow::Result<()> {
        let mut target = self.painter.display.draw();

        self.draw_shapes(&mut target)?;

        target.finish()?;

        Ok(())
    }

    fn handle_draw(&mut self) -> anyhow::Result<()> {
        if !self.is_loop {
            return Ok(());
        }

        if let Some(draw) = self.draw {
            draw(self)
        }

        self.draw_frame()
    }

    // run
    pub(crate) fn run(
        mut self,
        event_loop: EventLoop<()>,
        setup: SetupFn<S, R>,
    ) -> anyhow::Result<()> {
        setup(&mut self);

        let _ = event_loop.run(move |event, window_target| {
            self.event_handler(event, window_target);
        });

        Ok(())
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

impl<S, R: Renderer> Processing<S, R> {
    fn event_handler(&mut self, event: Event<()>, window_target: &EventLoopWindowTarget<()>) {
        let start_time = std::time::Instant::now();

        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::Resized(size) => {
                    self.window_settings.width = size.width;
                    self.window_settings.height = size.height;
                }
                WindowEvent::KeyboardInput {
                    device_id: _,
                    event,
                    is_synthetic: _,
                } => {
                    if let Some(key_pressed) = self.key_pressed {
                        if event.state == ElementState::Released {
                            if let PhysicalKey::Code(key) = event.physical_key {
                                key_pressed(self, key);
                            }
                        }
                    }
                }
                WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                } => {
                    if let Some(mouse_moved) = self.mouse_moved {
                        mouse_moved(self, position.x as f32, position.y as f32);
                    }
                }
                WindowEvent::MouseWheel {
                    device_id: _,
                    delta: _,
                    phase: _,
                } => {}
                WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                } => {
                    if let Some(mouse_clicked) = self.mouse_clicked {
                        if state == winit::event::ElementState::Released {
                            mouse_clicked(self, button);
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    println!("RedrawRequested");

                    let _frame_time =
                        std::time::Duration::from_secs_f32(1.0 / self.frame_rate as f32);

                    let _ = self.handle_draw();

                    let _elapsed = start_time.elapsed();
                }
                _ => (),
            }
        };
    }
}

// impl<T, R: Renderer> Drop for Processing<T, R> {
//     fn drop(&mut self) {
//         println!("Processing is dropped");
//     }
// }
