use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

use crate::{Error, Game, runtime};

/// Configures an engine, defaulting to WALZER and an 800 by 600 logical pixel content area.
pub struct EngineBuilder {
    window_attributes: WindowAttributes,
}

impl EngineBuilder {
    /// Sets the window title. Defaults to "WALZER" when omitted.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.window_attributes = self.window_attributes.with_title(title);
        self
    }

    /// Sets the initial inner content width and height in logical pixels.
    ///
    /// Excludes the title bar and borders; Windows DPI scaling is applied at creation.
    /// Defaults to 800 by 600 when omitted.
    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.window_attributes = self
            .window_attributes
            .with_inner_size(LogicalSize::new(width, height));
        self
    }

    /// Prepares the event loop without creating or showing a window.
    ///
    /// Call on the main thread. Only one event loop may be built per process.
    pub fn build(self) -> Result<Engine, Error> {
        let event_loop = EventLoop::new().map_err(Error::Preparation)?;
        event_loop.set_control_flow(ControlFlow::Wait);
        Ok(Engine {
            event_loop,
            window_attributes: self.window_attributes,
        })
    }
}

/// A prepared engine. Its window is created when execution starts.
pub struct Engine {
    event_loop: EventLoop<()>,
    window_attributes: WindowAttributes,
}

impl Engine {
    pub fn builder() -> EngineBuilder {
        EngineBuilder {
            window_attributes: Window::default_attributes()
                .with_title("WALZER")
                .with_inner_size(LogicalSize::new(800.0, 600.0)),
        }
    }

    /// Takes ownership of the game, opens one window and renders on redraw requests.
    /// The same mutable game is used for every redraw, with no update loop.
    /// Window and event-loop resources are released before returning.
    pub fn run(self, game: impl Game) -> Result<(), Error> {
        runtime::run(self.event_loop, self.window_attributes, game)
    }
}
