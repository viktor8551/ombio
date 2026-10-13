mod keyboard;

pub use keyboard::Keyboard;
pub use winit::keyboard::KeyCode;

use winit::event::ElementState;

pub struct Input {
    keyboard: Keyboard,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            keyboard: Keyboard::new(),
        }
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub(crate) fn handle_key(&mut self, key: KeyCode, state: ElementState) {
        self.keyboard.handle_key(key, state);
    }

    pub(crate) fn release_all(&mut self) {
        self.keyboard.release_all();
    }

    pub(crate) fn finish_update(&mut self) {
        self.keyboard.finish_update();
    }
}
