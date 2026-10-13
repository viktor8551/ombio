use ombio::{Color, Engine, Error, Frame, Game, Rectangle};

struct RectangleGame {
    background: Color,
    rectangle: Rectangle,
}

impl Game for RectangleGame {
    fn render(&mut self, frame: &mut Frame) {
        frame.clear(self.background);
        frame.draw(&self.rectangle);
    }
}

fn main() -> Result<(), Error> {
    Engine::builder()
        .title("My OMBIO game")
        .window_size(640, 480)
        .build()?
        .run(RectangleGame {
            background: Color::rgb(48, 144, 72),
            rectangle: Rectangle::new(50.0, 50.0, 200.0, 100.0, Color::rgb(240, 176, 48)),
        })
}
