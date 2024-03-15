use glium::{glutin::surface::WindowSurface, Display, Program};
use winit::{event_loop::EventLoop, window::Window};

#[derive(Debug)]
pub struct Painter {
    pub(crate) window: Window,
    pub(crate) display: Display<WindowSurface>,
    pub(crate) program: Program,
}

impl Painter {
    pub fn new(window: Window, display: Display<WindowSurface>, program: Program) -> Painter {
        Painter {
            window,
            display,
            program,
        }
    }
}
