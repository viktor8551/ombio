use std::path::PathBuf;

use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

use crate::{Context, Error, Filesystem, Game, runtime};

/// Configures an engine, defaulting to OMBIO and an 800 by 600 logical pixel content area.
pub struct EngineBuilder {
    window_attributes: WindowAttributes,
    asset_directory: PathBuf,
}

impl EngineBuilder {
    /// Sets the window title. Defaults to "OMBIO" when omitted.
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

    /// Sets the asset directory, resolved at build time. Defaults to `assets`.
    pub fn asset_directory(mut self, directory: impl Into<PathBuf>) -> Self {
        self.asset_directory = directory.into();
        self
    }

    /// Prepares the event loop without creating or showing a window.
    ///
    /// Call on the main thread. Only one event loop may be built per process.
    pub fn build(self) -> Result<Engine, Error> {
        let filesystem = Filesystem::new(self.asset_directory)?;
        let event_loop = EventLoop::new().map_err(Error::Preparation)?;
        event_loop.set_control_flow(ControlFlow::Wait);
        Ok(Engine {
            event_loop,
            window_attributes: self.window_attributes,
            filesystem,
        })
    }
}

/// A prepared engine. Its window is created when execution starts.
pub struct Engine {
    event_loop: EventLoop<()>,
    window_attributes: WindowAttributes,
    filesystem: Filesystem,
}

impl Engine {
    pub fn builder() -> EngineBuilder {
        EngineBuilder {
            window_attributes: Window::default_attributes()
                .with_title("OMBIO")
                .with_inner_size(LogicalSize::new(800.0, 600.0)),
            asset_directory: PathBuf::from("assets"),
        }
    }

    /// Opens one window and constructs the game with its ready Context.
    ///
    /// Calls the constructor once, before any updates or rendering. Constructor
    /// errors are returned without starting the game.
    /// Window and event-loop resources are released before returning.
    pub fn run<G: Game>(
        self,
        create_game: impl FnOnce(&Context) -> Result<G, Error>,
    ) -> Result<(), Error> {
        runtime::run(
            self.event_loop,
            self.window_attributes,
            self.filesystem,
            create_game,
        )
    }
}
