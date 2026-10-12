use walzer::{Color, Engine, Error, Frame, Game};

struct Background {
    color: Color,
}

impl Game for Background {
    fn render(&mut self, frame: &mut Frame) {
        frame.clear(self.color);
    }
}

fn main() -> Result<(), Error> {
    Engine::builder()
        .title("My WALZER game")
        .window_size(640, 480)
        .build()?
        .run(Background {
            color: Color::rgb(48, 144, 72),
        })
}
