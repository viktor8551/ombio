use walzer::{Color, Drawable, Engine, Error, Frame, Game, Rectangle};

struct Panel {
    background: Rectangle,
    foreground: Rectangle,
}

impl Drawable for Panel {
    fn draw(&self, frame: &mut Frame) {
        frame.draw(&self.background);
        frame.draw(&self.foreground);
    }
}

struct DrawableGame {
    panel: Panel,
}

impl Game for DrawableGame {
    fn render(&mut self, frame: &mut Frame) {
        frame.clear(Color::rgb(32, 40, 56));
        frame.draw(&self.panel);

        let mut marker = Rectangle::new(64.0, 88.0, 64.0, 32.0, Color::rgb(240, 176, 48));
        frame.draw(&marker);

        marker.x += 128.0;
        marker.color = Color::rgb(80, 216, 128);
        let drawable: &dyn Drawable = &marker;
        frame.draw(drawable);
    }
}

fn main() -> Result<(), Error> {
    Engine::builder()
        .title("WALZER composed drawables")
        .window_size(320, 208)
        .build()?
        .run(DrawableGame {
            panel: Panel {
                background: Rectangle::new(40.0, 40.0, 240.0, 128.0, Color::rgb(192, 64, 96)),
                foreground: Rectangle::new(56.0, 56.0, 208.0, 96.0, Color::rgb(48, 72, 96)),
            },
        })
}
