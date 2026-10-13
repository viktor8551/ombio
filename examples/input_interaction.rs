use walzer::{
    Color, Context, Engine, Error, Frame, Game, Rectangle,
    input::{KeyCode, MouseButton},
};

struct InputInteraction {
    x: f32,
    y: f32,
    highlighted: bool,
}

impl Game for InputInteraction {
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
        let mouse = context.input().mouse();
        if mouse.is_inside_window()
            && mouse.is_down(MouseButton::Left)
            && let Some((x, y)) = mouse.position()
        {
            self.x = x as f32 - 16.0;
            self.y = y as f32 - 16.0;
        }
        if mouse.just_pressed(MouseButton::Right) {
            self.highlighted = !self.highlighted;
        }
        let (line_x, line_y) = mouse.scroll_lines();
        let (pixel_x, pixel_y) = mouse.scroll_pixels();
        self.x += (line_x * 16.0 + pixel_x) as f32;
        self.y += (line_y * 16.0 + pixel_y) as f32;
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
        .title(
            "Arrows/left drag: move | Space/right click: color | Scroll: move | Release R: reset",
        )
        .window_size(352, 256)
        .build()?
        .run(InputInteraction {
            x: 160.0,
            y: 112.0,
            highlighted: false,
        })
}
