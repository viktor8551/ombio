use ombio::{Color, Drawable, Engine, Error, Frame, Game, Image, Rectangle, Sprite, View};

struct Panel {
    background: Rectangle,
    foreground: Sprite,
}

impl Drawable for Panel {
    fn draw(&self, view: &mut View<'_>) {
        view.draw(&self.background);
        view.draw(&self.foreground);
    }
}

struct DrawableGame {
    panel: Panel,
}

impl Game for DrawableGame {
    fn render(&mut self, frame: &mut Frame) {
        frame.clear(Color::rgb(32, 40, 56));
        frame.draw(&self.panel);

        let mut marker = Rectangle::new(64.0, 88.0, 64.0, 32.0, Color::rgba(240, 176, 48, 128));
        frame.draw(&marker);

        marker.x += 128.0;
        marker.color = Color::rgba(80, 216, 128, 128);
        let drawable: &dyn Drawable = &marker;
        frame.draw(drawable);
    }
}

fn main() -> Result<(), Error> {
    let image = Image::load(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/assets/sprite.png"
    ))?;
    Engine::builder()
        .title("OMBIO composed drawables")
        .window_size(320, 208)
        .build()?
        .run(DrawableGame {
            panel: Panel {
                background: Rectangle::new(40.0, 40.0, 240.0, 128.0, Color::rgb(192, 64, 96)),
                foreground: Sprite::new(image, 56.0, 56.0),
            },
        })
}
