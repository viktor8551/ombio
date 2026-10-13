use crate::{Drawable, Frame, Image, frame::DrawCommand};

/// An [`Image`] placed at x and y, drawn through [`Frame::draw`].
#[derive(Clone)]
pub struct Sprite {
    pub image: Image,
    pub x: f32,
    pub y: f32,
}

impl Sprite {
    pub fn new(image: Image, x: f32, y: f32) -> Self {
        Self { image, x, y }
    }
}

impl Drawable for Sprite {
    fn draw(&self, frame: &mut Frame) {
        if !self.x.is_finite() || !self.y.is_finite() {
            return;
        }
        frame.commands.push(DrawCommand::Sprite {
            image: self.image.clone(),
            x: self.x,
            y: self.y,
        });
    }
}
