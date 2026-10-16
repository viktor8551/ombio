use crate::{Camera, Image, camera::Projection};

/// A color with sRGB red, green and blue components and linear alpha from 0 to 255.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::rgba(red, green, blue, 255)
    }

    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
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
            a: f64::from(self.alpha) / 255.0,
        }
    }
}

/// One complete picture. Each redraw starts with an opaque black background.
pub struct Frame {
    pub(crate) background: Color,
    pub(crate) draws: Vec<RecordedDraw>,
    pub(crate) size: (u32, u32),
    window_projection: Option<Projection>,
}

pub(crate) struct RecordedDraw {
    pub(crate) projection: Projection,
    pub(crate) command: DrawCommand,
}

pub(crate) enum DrawCommand {
    Rectangle(Rectangle),
    Sprite { image: Image, x: f32, y: f32 },
}

/// Something that can be drawn through a Frame or View.
pub trait Drawable {
    fn draw(&self, view: &mut View<'_>);
}

/// A filled rectangle in logical pixels from the top-left.
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
    fn draw(&self, view: &mut View<'_>) {
        if ![self.x, self.y, self.width, self.height]
            .into_iter()
            .all(f32::is_finite)
            || self.width <= 0.0
            || self.height <= 0.0
        {
            return;
        }
        view.submit(DrawCommand::Rectangle(*self));
    }
}

impl Frame {
    pub(crate) fn new(size: (u32, u32), dpi_scale: f64) -> Self {
        Self {
            background: Color::rgb(0, 0, 0),
            draws: Vec::new(),
            size,
            window_projection: Projection::window(size, dpi_scale),
        }
    }

    /// Fills the picture with a color, discarding earlier drawing in this frame.
    pub fn clear(&mut self, color: Color) {
        self.background = color;
        self.draws.clear();
    }

    /// Draws in window coordinates, after earlier submissions.
    pub fn draw(&mut self, drawable: &(impl Drawable + ?Sized)) {
        self.drawing(self.window_projection).draw(drawable);
    }

    /// Full-window View centered on a camera snapshot, without clearing.
    /// Zoom 1 preserves logical pixels; x points right, y down.
    /// Non-finite camera values, zoom <= 0, or zero-sized windows draw nothing.
    pub fn view(&mut self, camera: &Camera) -> View<'_> {
        let projection = self
            .window_projection
            .and_then(|window| camera.projection(self.size, window.scale));
        self.drawing(projection)
    }

    fn drawing(&mut self, projection: Option<Projection>) -> View<'_> {
        View {
            draws: &mut self.draws,
            projection,
        }
    }
}

/// A temporary drawing handle borrowing a Frame's submission list.
pub struct View<'a> {
    draws: &'a mut Vec<RecordedDraw>,
    projection: Option<Projection>,
}

impl View<'_> {
    /// Draws an object or composition using this View's coordinates.
    pub fn draw(&mut self, drawable: &(impl Drawable + ?Sized)) {
        if self.projection.is_some() {
            drawable.draw(self);
        }
    }

    pub(crate) fn submit(&mut self, command: DrawCommand) {
        if let Some(projection) = self.projection {
            self.draws.push(RecordedDraw {
                projection,
                command,
            });
        }
    }
}
