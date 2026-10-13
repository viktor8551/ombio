use std::time::{Duration, Instant};

/// Elapsed time for the current game update.
pub struct Time {
    delta_time: Duration,
    previous_update: Option<Instant>,
}

impl Time {
    pub(crate) fn new() -> Self {
        Self {
            delta_time: Duration::ZERO,
            previous_update: None,
        }
    }

    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    pub(crate) fn advance(&mut self, now: Instant) {
        self.delta_time = self
            .previous_update
            .map_or(Duration::ZERO, |previous| now.duration_since(previous));
        self.previous_update = Some(now);
    }
}
