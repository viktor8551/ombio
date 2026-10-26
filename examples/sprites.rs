use ombio::{Color, Context, Engine, Error, Frame, Game, Image, Rectangle, Sprite};

struct SpriteGame {
    first: Sprite,
    second: Sprite,
    clipped: Sprite,
}

impl SpriteGame {
    fn new(ctx: &Context) -> Result<Self, Error> {
        let image = Image::load(ctx, "sprite.png")?;
        Ok(Self {
            first: Sprite::new(image.clone(), 56.0, 56.0),
            second: Sprite::new(image.clone(), 168.0, 56.0),
            clipped: Sprite::new(image, -24.0, 176.0),
        })
    }
}

impl Game for SpriteGame {
    fn render(&mut self, frame: &mut Frame) {
        frame.clear(Color::rgb(32, 40, 56));
        frame.draw(&Rectangle::new(
            40.0,
            40.0,
            240.0,
            128.0,
            Color::rgb(192, 64, 96),
        ));
        frame.draw(&self.first);
        frame.draw(&Rectangle::new(
            120.0,
            88.0,
            80.0,
            32.0,
            Color::rgb(80, 216, 128),
        ));
        frame.draw(&self.second);
        frame.draw(&self.clipped);
    }
}

fn main() -> Result<(), Error> {
    Engine::builder()
        .asset_directory("examples/assets")
        .title("OMBIO PNG sprites")
        .window_size(320, 208)
        .build()?
        .run(SpriteGame::new)
}
