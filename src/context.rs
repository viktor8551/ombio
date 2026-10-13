use std::time::Instant;

use crate::{Time, input::Input};

pub struct Context {
    time: Time,
    input: Input,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self {
            time: Time::new(),
            input: Input::new(),
        }
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub(crate) fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub(crate) fn advance(&mut self, now: Instant) {
        self.time.advance(now);
    }
}
