use walzer::{Color, Context, Engine, Error, Frame, Game, Rectangle, input::KeyCode};

struct KeyboardMovement {
    x: f32,
    y: f32,
    highlighted: bool,
}

impl Game for KeyboardMovement {
    fn update(&mut self, context: &Context) {
        let keyboard = context.input().keyboard();
        let distance = 120.0 * context.time().delta_time().as_secs_f32();
        if keyboard.is_down(KeyCode::ArrowLeft) {
            self.x -= distance;
        }
        if keyboard.is_down(KeyCode::ArrowRight) {
            self.x += distance;
        }
        if keyboard.is_down(KeyCode::ArrowUp) {
            self.y -= distance;
        }
        if keyboard.is_down(KeyCode::ArrowDown) {
            self.y += distance;
        }
        if keyboard.just_pressed(KeyCode::Space) {
            self.highlighted = !self.highlighted;
        }
        if keyboard.just_released(KeyCode::KeyR) {
            self.x = 160.0;
            self.y = 112.0;
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        frame.clear(Color::rgb(32, 40, 56));
        let color = if self.highlighted {
            Color::rgb(248, 208, 80)
        } else {
            Color::rgb(80, 216, 128)
        };
        frame.draw(&Rectangle::new(self.x, self.y, 32.0, 32.0, color));
    }
}

fn main() -> Result<(), Error> {
    Engine::builder()
        .title("Keyboard Example. Arrows to move, Space changes color, release R to reset")
        .window_size(352, 256)
        .build()?
        .run(KeyboardMovement {
            x: 160.0,
            y: 112.0,
            highlighted: false,
        })
}
