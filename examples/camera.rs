use ombio::{
    Camera, Color, Context, Drawable, Engine, Error, Frame, Game, Image, Rectangle, Sprite, View,
    input::KeyCode,
};

struct Scenery {
    marker: Sprite,
}

impl Drawable for Scenery {
    fn draw(&self, view: &mut View<'_>) {
        let color = Color::rgb(56, 64, 80);
        view.draw(&Rectangle::new(-2000.0, -1.0, 4000.0, 2.0, color));
        view.draw(&Rectangle::new(-1.0, -2000.0, 2.0, 4000.0, color));
        view.draw(&self.marker);
    }
}

struct CameraGame {
    camera: Camera,
    player: Rectangle,
    scenery: Scenery,
    following: bool,
}

impl Game for CameraGame {
    fn update(&mut self, context: &Context) {
        let keyboard = context.input().keyboard();
        let distance = 240.0 * context.time().delta_time().as_secs_f32();
        if keyboard.is_down(KeyCode::KeyA) {
            self.player.x -= distance;
        }
        if keyboard.is_down(KeyCode::KeyD) {
            self.player.x += distance;
        }
        if keyboard.is_down(KeyCode::KeyW) {
            self.player.y -= distance;
        }
        if keyboard.is_down(KeyCode::KeyS) {
            self.player.y += distance;
        }
        if keyboard.just_pressed(KeyCode::Space) {
            self.following = !self.following;
        }
        if self.following {
            self.camera.x = self.player.x + self.player.width / 2.0;
            self.camera.y = self.player.y + self.player.height / 2.0;
        }
        let mouse = context.input().mouse();
        let scroll = (mouse.scroll_lines().1 + mouse.scroll_pixels().1 / 40.0) as f32;
        self.camera.zoom = (self.camera.zoom * 1.1_f32.powf(scroll)).clamp(0.25, 4.0);
    }

    fn render(&mut self, frame: &mut Frame) {
        frame.clear(Color::rgb(24, 32, 48));

        let mut world = frame.view(&self.camera);
        world.draw(&self.scenery);
        world.draw(&self.player);

        let status = if self.following {
            Color::rgb(80, 216, 128)
        } else {
            Color::rgb(248, 192, 64)
        };
        frame.draw(&Rectangle::new(
            12.0,
            12.0,
            192.0,
            56.0,
            Color::rgba(0, 0, 0, 208),
        ));
        frame.draw(&Rectangle::new(24.0, 24.0, 12.0, 12.0, status));
        frame.draw(&Rectangle::new(
            24.0,
            48.0,
            160.0 * self.camera.zoom / 4.0,
            8.0,
            Color::rgb(96, 176, 248),
        ));
    }
}

fn main() -> Result<(), Error> {
    let image = Image::load(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/assets/sprite.png"
    ))?;
    Engine::builder()
        .title("WASD: move | Space: follow on/off | Scroll: zoom")
        .window_size(800, 600)
        .build()?
        .run(CameraGame {
            camera: Camera::default(),
            player: Rectangle::new(-20.0, -14.0, 28.0, 28.0, Color::rgb(248, 192, 64)),
            scenery: Scenery {
                marker: Sprite::new(image, 160.0, 96.0),
            },
            following: true,
        })
}
