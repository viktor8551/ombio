use std::time::Instant;

use crate::Time;

pub struct Context {
    time: Time,
}

impl Context {
    pub(crate) fn new() -> Self {
        Self { time: Time::new() }
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

    pub(crate) fn advance(&mut self, now: Instant) {
        self.time.advance(now);
    }
}
