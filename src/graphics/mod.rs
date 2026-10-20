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

use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoopProxy,
    window::{Window, WindowId},
};

use crate::{Error, Game};

use self::renderer::Renderer;

pub struct Graphics {
    renderer: Renderer,
    window: Arc<Window>,
    occluded: bool,
}

impl Graphics {
    pub(crate) async fn new(window: Window, proxy: EventLoopProxy<()>) -> Result<Self, Error> {
        let window = Arc::new(window);
        let renderer = Renderer::new(window.clone(), proxy).await?;
        Ok(Self {
            renderer,
            window,
            occluded: false,
        })
    }

    /// Current window dimensions in logical pixels.
    pub fn window_size(&self) -> (f64, f64) {
        logical_size(self.window.inner_size(), self.window.scale_factor())
    }

    pub(crate) fn window_id(&self) -> WindowId {
        self.window.id()
    }

    pub(crate) fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    pub(crate) fn set_occluded(&mut self, occluded: bool) {
        self.occluded = occluded;
    }

    pub(crate) fn can_render(&self) -> bool {
        let size = self.window.inner_size();
        !self.occluded
            && self.window.is_visible() != Some(false)
            && self.window.is_minimized() != Some(true)
            && size.width > 0
            && size.height > 0
    }

    pub(crate) fn request_redraw(&self) {
        if self.can_render() {
            self.window.request_redraw();
        }
    }

    pub(crate) fn render(&mut self, game: &mut impl Game) -> Result<(), Error> {
        if !self.can_render() {
            return Ok(());
        }
        self.renderer.render(&self.window, game)
    }

    pub(crate) fn check_error(&self) -> Result<(), Error> {
        self.renderer.check_error()
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
