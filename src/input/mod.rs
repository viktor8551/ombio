mod keyboard;
mod mouse;

pub use keyboard::Keyboard;
pub use mouse::Mouse;
pub use winit::event::MouseButton;
pub use winit::keyboard::KeyCode;

use winit::event::{ElementState, WindowEvent};

pub struct Input {
    keyboard: Keyboard,
    mouse: Mouse,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
        }
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }

    pub(crate) fn handle_mouse_event(&mut self, event: &WindowEvent) {
        self.mouse.handle_event(event);
    }

    pub(crate) fn set_scale_factor(&mut self, scale_factor: f64) {
        self.mouse.set_scale_factor(scale_factor);
    }

    pub(crate) fn handle_key(&mut self, key: KeyCode, state: ElementState) {
        self.keyboard.handle_key(key, state);
    }

    pub(crate) fn release_all(&mut self) {
        self.keyboard.release_all();
        self.mouse.release_all();
    }

    pub(crate) fn finish_update(&mut self) {
        self.keyboard.finish_update();
        self.mouse.finish_update();
    }
}
