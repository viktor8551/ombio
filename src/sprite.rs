use crate::{Drawable, Image, View, frame::DrawCommand};

/// An [`Image`] placed at x and y, drawn through a Frame or View.
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
    fn draw(&self, view: &mut View<'_>) {
        if !self.x.is_finite() || !self.y.is_finite() {
            return;
        }
        view.submit(DrawCommand::Sprite {
            image: self.image.clone(),
            x: self.x,
            y: self.y,
        });
    }
}
