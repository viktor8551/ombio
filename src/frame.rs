use crate::Sprite;

/// An opaque color with red, green and blue sRGB components from 0 to 255.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub(crate) fn linear(self) -> wgpu::Color {
        fn component(value: u8) -> f64 {
            let value = f64::from(value) / 255.0;
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        }
        wgpu::Color {
            r: component(self.red),
            g: component(self.green),
            b: component(self.blue),
            a: 1.0,
        }
    }
}

/// One complete picture. Each redraw starts with an opaque black background.
pub struct Frame {
    pub(crate) background: Color,
    pub(crate) commands: Vec<DrawCommand>,
}

pub(crate) enum DrawCommand {
    Rectangle(Rectangle),
    Sprite { sprite: Sprite, x: f32, y: f32 },
}

/// An object that describes a picture within a frame.
///
/// Compose other drawables through [`Frame::draw`], as in the `drawables` example.
pub trait Drawable {
    /// Records drawing commands. Implementations must not submit themselves recursively.
    fn draw(&self, frame: &mut Frame);
}

/// An editable, opaque filled rectangle drawn through [`Frame::draw`].
///
/// Coordinates use logical pixels from the top-left, with x rightward and y downward.
/// Drawing applies DPI scaling and clips at the content boundary.
/// Non-finite geometry and non-positive dimensions draw nothing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Rectangle {
    pub const fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, frame: &mut Frame) {
        if ![self.x, self.y, self.width, self.height]
            .into_iter()
            .all(f32::is_finite)
            || self.width <= 0.0
            || self.height <= 0.0
        {
            return;
        }
        frame.commands.push(DrawCommand::Rectangle(*self));
    }
}

impl Frame {
    pub(crate) fn new() -> Self {
        Self {
            background: Color::rgb(0, 0, 0),
            commands: Vec::new(),
        }
    }

    /// Fills the picture with an opaque color, discarding earlier drawing in this frame.
    pub fn clear(&mut self, color: Color) {
        self.background = color;
        self.commands.clear();
    }

    /// Records drawing commands in submission order for later presentation.
    ///
    /// Accepts concrete drawables and borrowed trait objects (`&dyn Drawable`).
    /// Rectangles capture their current geometry and color; later edits or dropping
    /// the object do not affect earlier submissions.
    pub fn draw(&mut self, drawable: &(impl Drawable + ?Sized)) {
        drawable.draw(self);
    }

    /// Draws a sprite at its natural dimensions in logical pixels.
    ///
    /// Position follows [`Rectangle`]: top-left anchor, x rightward, y
    /// downward, with DPI scaling and clipping at the content boundary.
    /// Transparent pixels reveal earlier drawing; partial transparency blends
    /// with it. Non-finite positions draw nothing.
    pub fn draw_sprite(&mut self, sprite: &Sprite, x: f32, y: f32) {
        if !x.is_finite() || !y.is_finite() {
            return;
        }
        self.commands.push(DrawCommand::Sprite {
            sprite: sprite.clone(),
            x,
            y,
        });
    }
}
