use walzer::{Color, Engine, Error, Frame, Game};

struct RectangleGame {
    background: Color,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

impl Game for RectangleGame {
    fn render(&mut self, frame: &mut Frame) {
        frame.clear(self.background);
        frame.fill_rect(self.x, self.y, self.width, self.height, self.color);
    }
}

fn main() -> Result<(), Error> {
    Engine::builder()
        .title("My WALZER game")
        .window_size(640, 480)
        .build()?
        .run(RectangleGame {
            background: Color::rgb(48, 144, 72),
            x: 50.0,
            y: 50.0,
            width: 200.0,
            height: 100.0,
            color: Color::rgb(240, 176, 48),
        })
}
