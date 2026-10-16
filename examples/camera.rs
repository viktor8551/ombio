use ombio::{
    Camera, Color, Context, Drawable, Engine, Error, Frame, Game, Image, Rectangle, Sprite, View,
    Viewport, input::KeyCode,
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

// The same nested composition is submitted through both Cameras.
struct World<'a> {
    scenery: &'a Scenery,
    player: &'a Rectangle,
}

impl Drawable for World<'_> {
    fn draw(&self, view: &mut View<'_>) {
        view.draw(self.scenery);
        view.draw(self.player);
    }
}

struct CameraGame {
    camera: Camera,
    minimap_camera: Camera,
    player: Rectangle,
    scenery: Scenery,
    following: bool,
    split_screen: bool,
}

impl CameraGame {
    fn new() -> Result<Self, Error> {
        let image = Image::load(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/assets/sprite.png"
        ))?;

        Ok(Self {
            camera: Camera::default(),
            minimap_camera: Camera {
                zoom: 0.25,
                ..Camera::default()
            },
            player: Rectangle::new(-20.0, -14.0, 28.0, 28.0, Color::rgb(248, 192, 64)),
            scenery: Scenery {
                marker: Sprite::new(image, 160.0, 96.0),
            },
            following: true,
            split_screen: false,
        })
    }
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
        if keyboard.just_pressed(KeyCode::Tab) {
            self.split_screen = !self.split_screen;
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

        let world = World {
            scenery: &self.scenery,
            player: &self.player,
        };
        let (width, height) = frame.window_size();
        let mut main = frame.view(&self.camera);
        if self.split_screen {
            main.set_viewport(Some(Viewport::new(
                0.0,
                0.0,
                width as f32 / 2.0,
                height as f32,
            )));
        }
        main.draw(&world);

        // The inset keeps its explicit logical layout on resize. Tab switches to
        // two adjacent rectangles through the very same View API.
        let viewport = if self.split_screen {
            Viewport::new(width as f32 / 2.0, 0.0, width as f32 / 2.0, height as f32)
        } else {
            Viewport::new(564.0, 20.0, 216.0, 160.0)
        };
        let mut minimap = frame.view(&self.minimap_camera).with_viewport(viewport);
        // Views never clear automatically, so draw an explicit opaque background.
        let world_width = viewport.width / self.minimap_camera.zoom;
        let world_height = viewport.height / self.minimap_camera.zoom;
        minimap.draw(&Rectangle::new(
            self.minimap_camera.x - world_width / 2.0,
            self.minimap_camera.y - world_height / 2.0,
            world_width,
            world_height,
            Color::rgb(40, 52, 64),
        ));
        minimap.draw(&world);

        // Fixed UI is submitted last and can cross either Viewport.
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
    let engine = Engine::builder()
        .title("WASD: move | Space: follow | Scroll: zoom | Tab: split")
        .window_size(800, 600)
        .build()?;

    engine.run(CameraGame::new()?)
}
