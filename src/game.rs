use crate::{Context, Frame};

/// An application and the state that determines what it draws.
pub trait Game {
    /// Advances game state independently of rendering.
    fn update(&mut self, _context: &Context) {}

    /// Draws a complete picture when the window needs a redraw.
    fn render(&mut self, frame: &mut Frame);
}
