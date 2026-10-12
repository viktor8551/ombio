use crate::Frame;

/// An application and the state that determines what it draws.
pub trait Game {
    /// Draws a complete picture when the window needs a redraw.
    fn render(&mut self, frame: &mut Frame);
}
