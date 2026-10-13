use std::collections::HashSet;

use winit::event::ElementState;

use super::KeyCode;

pub struct Keyboard {
    down: HashSet<KeyCode>,
    pressed: HashSet<KeyCode>,
    released: HashSet<KeyCode>,
}

impl Keyboard {
    pub(crate) fn new() -> Self {
        Self {
            down: HashSet::new(),
            pressed: HashSet::new(),
            released: HashSet::new(),
        }
    }

    pub fn is_down(&self, key: KeyCode) -> bool {
        self.down.contains(&key)
    }

    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn just_released(&self, key: KeyCode) -> bool {
        self.released.contains(&key)
    }

    pub(super) fn handle_key(&mut self, key: KeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => {
                if self.down.insert(key) {
                    self.pressed.insert(key);
                }
            }
            ElementState::Released => {
                if self.down.remove(&key) {
                    self.released.insert(key);
                }
            }
        }
    }

    pub(super) fn release_all(&mut self) {
        self.released.extend(self.down.drain());
    }

    pub(super) fn finish_update(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }
}
