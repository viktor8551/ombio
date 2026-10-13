use ombio::{Color, Context, Engine, Error, Frame, Game, Rectangle};

struct MovingRectangle {
    phase: f64,
}

impl Game for MovingRectangle {
    fn update(&mut self, context: &Context) {
        // Move at 120 logical pixels per second, bouncing between the edges.
        self.phase = (self.phase + 120.0 * context.time().delta_time().as_secs_f64()) % 512.0;
    }

    fn render(&mut self, frame: &mut Frame) {
        frame.clear(Color::rgb(32, 40, 56));
        let x = 32.0 + self.phase.min(512.0 - self.phase) as f32;
        frame.draw(&Rectangle::new(
            x,
            72.0,
            32.0,
            32.0,
            Color::rgb(80, 216, 128),
        ));
    }
}

fn main() -> Result<(), Error> {
    Engine::builder()
        .title("OMBIO moving rectangle")
        .window_size(352, 176)
        .build()?
        .run(MovingRectangle { phase: 0.0 })
}
