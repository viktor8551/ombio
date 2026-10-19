mod camera;
mod frame;
mod geometry;
mod image;
mod renderer;
mod sprite;

pub use camera::{Camera, Viewport};
pub use frame::{Color, Drawable, Frame, Rectangle, View};
pub use image::Image;
pub use sprite::Sprite;

use std::sync::Arc;

use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy, window::Window};

use crate::Error;

use self::renderer::Renderer;

pub struct Graphics {
    pub(crate) renderer: Renderer,
    pub(crate) window: Arc<Window>,
}

impl Graphics {
    pub(crate) async fn new(window: Window, proxy: EventLoopProxy<()>) -> Result<Self, Error> {
        let window = Arc::new(window);
        let renderer = Renderer::new(window.clone(), proxy).await?;
        Ok(Self { renderer, window })
    }

    /// Current window dimensions in logical pixels.
    pub fn window_size(&self) -> (f64, f64) {
        logical_size(self.window.inner_size(), self.window.scale_factor())
    }
}

pub(crate) fn logical_size(size: PhysicalSize<u32>, scale_factor: f64) -> (f64, f64) {
    if !scale_factor.is_finite() || scale_factor <= 0.0 {
        return (0.0, 0.0);
    }
    let size = (
        f64::from(size.width) / scale_factor,
        f64::from(size.height) / scale_factor,
    );
    if !size.0.is_finite() || !size.1.is_finite() {
        return (0.0, 0.0);
    }
    size
}
