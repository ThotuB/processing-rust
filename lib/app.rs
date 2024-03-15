use glium::backend::glutin::SimpleWindowBuilder;
use winit::{event::MouseButton, event_loop::EventLoop, keyboard::KeyCode};

use crate::{
    painter::Painter,
    processing::{DrawFn, KeyPressedFn, MouseClickedFn, MouseMovedFn, Processing, SetupFn},
    settings::WindowSettings,
    traits::Renderer,
    Vector2D,
};

#[derive(Debug)]
pub struct App<S, R: Renderer + Default> {
    state: S,

    window_settings: WindowSettings,

    setup: SetupFn<S, R>,
    draw: Option<DrawFn<S, R>>,

    mouse_clicked: Option<MouseClickedFn<S, R>>,
    mouse_moved: Option<MouseMovedFn<S, R>>,

    key_pressed: Option<KeyPressedFn<S, R>>,
}

impl<S, R: Renderer + Default> App<S, R> {
    pub(crate) fn new(state: S) -> App<S, R> {
        App {
            state,
            window_settings: WindowSettings::default(),
            setup: |_| {},
            draw: None,
            mouse_clicked: None,
            mouse_moved: None,
            key_pressed: None,
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> App<S, R> {
        self.window_settings.width = width;
        self.window_settings.height = height;
        self
    }

    pub fn with_title(mut self, title: &str) -> App<S, R> {
        self.window_settings.title = title.to_string();
        self
    }

    pub fn setup(mut self, f: SetupFn<S, R>) -> App<S, R> {
        self.setup = f;
        self
    }

    pub fn draw(mut self, f: DrawFn<S, R>) -> App<S, R> {
        self.draw = Some(f);
        self
    }

    pub fn mouse_clicked(mut self, f: MouseClickedFn<S, R>) -> App<S, R> {
        self.mouse_clicked = Some(f);
        self
    }

    pub fn mouse_moved(mut self, f: MouseMovedFn<S, R>) -> App<S, R> {
        self.mouse_moved = Some(f);
        self
    }

    pub fn key_pressed(mut self, f: KeyPressedFn<S, R>) -> App<S, R> {
        self.key_pressed = Some(f);
        self
    }

    pub fn run(self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;
        let (window, display) = SimpleWindowBuilder::new()
            .with_inner_size(self.window_settings.width, self.window_settings.height)
            .with_title(&self.window_settings.title)
            .build(&event_loop);

        let program = glium::Program::from_source(
            &display,
            include_str!("shaders/vertex.glsl"),
            include_str!("shaders/fragment.glsl"),
            None,
        )?;

        let processing = Processing::new(
            self.state,
            self.window_settings,
            Painter::new(window, display, program),
            self.setup,
            self.draw,
            self.mouse_clicked,
            self.mouse_moved,
            self.key_pressed,
        );

        processing.run(event_loop)
    }
}
