//! A small 2D drawing library. Walzer owns the window and calls [`Game::draw`].

mod render;
mod runtime;

pub use runtime::run;

/// Drawing behavior supplied by a game. Platform setup belongs to Walzer.
pub trait Game {
    fn draw(&mut self, frame: &mut Frame);
}

/// A solid rectangle in logical pixels, with the origin at the viewport's top left.
#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub position: [f32; 2],
    pub size: [f32; 2],
    /// Linear RGBA components in the range 0..=1.
    pub color: [f32; 4],
}

/// Drawing commands for one frame. Rectangles are drawn in submission order.
pub struct Frame {
    viewport: [f32; 2],
    rectangles: Vec<Rectangle>,
}

impl Frame {
    /// The visible width and height in logical pixels.
    pub fn viewport(&self) -> [f32; 2] {
        self.viewport
    }

    /// Draw a rectangle. Coordinates must be finite and size must be nonnegative.
    pub fn draw_rectangle(&mut self, rectangle: Rectangle) {
        self.rectangles.push(rectangle);
    }

    fn new(viewport: [f32; 2]) -> Self {
        Self {
            viewport,
            rectangles: Vec::new(),
        }
    }
}
