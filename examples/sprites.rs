use walzer::{Color, Engine, Frame, Game, Sprite};

struct SpriteGame {
    sprite: Sprite,
}

impl Game for SpriteGame {
    fn render(&mut self, frame: &mut Frame) {
        frame.clear(Color::rgb(32, 40, 56));
        frame.fill_rect(40.0, 40.0, 240.0, 128.0, Color::rgb(192, 64, 96));
        frame.draw_sprite(&self.sprite, 56.0, 56.0);
        frame.fill_rect(120.0, 88.0, 80.0, 32.0, Color::rgb(80, 216, 128));
        frame.draw_sprite(&self.sprite, 168.0, 56.0);
    }
}

fn main() -> Result<(), walzer::Error> {
    let sprite = Sprite::load(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/assets/sprite.png"
    ))?;
    Engine::builder()
        .title("WALZER PNG sprites")
        .window_size(320, 208)
        .build()?
        .run(SpriteGame { sprite })
}
