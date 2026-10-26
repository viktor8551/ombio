use std::time::Instant;

use crate::{Filesystem, Graphics, Time, input::Input};

pub struct Context {
    time: Time,
    input: Input,
    graphics: Graphics,
    filesystem: Filesystem,
}

impl Context {
    pub(crate) fn new(graphics: Graphics, filesystem: Filesystem) -> Self {
        let mut input = Input::new();
        input.set_scale_factor(graphics.scale_factor());
        Self {
            time: Time::new(),
            input,
            graphics,
            filesystem,
        }
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn graphics(&self) -> &Graphics {
        &self.graphics
    }

    pub fn filesystem(&self) -> &Filesystem {
        &self.filesystem
    }

    pub(crate) fn graphics_mut(&mut self) -> &mut Graphics {
        &mut self.graphics
    }

    pub(crate) fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub(crate) fn advance(&mut self, now: Instant) {
        self.input.set_scale_factor(self.graphics.scale_factor());
        self.time.advance(now);
    }
}
