//! Prepare and run a Windows window through the engine's public API.

#![cfg(windows)]

use std::fmt;

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    error::{EventLoopError, OsError},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

/// A failure while preparing or running the engine.
#[derive(Debug)]
pub enum Error {
    Preparation(EventLoopError),
    WindowCreation(OsError),
    Runtime(EventLoopError),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preparation(error) => write!(formatter, "engine preparation failed: {error}"),
            Self::WindowCreation(error) => write!(formatter, "window creation failed: {error}"),
            Self::Runtime(error) => write!(formatter, "engine runtime failed: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Preparation(error) | Self::Runtime(error) => Some(error),
            Self::WindowCreation(error) => Some(error),
        }
    }
}

/// Prepares an engine with a WALZER window and an 800 by 600 logical pixel content area.
pub struct EngineBuilder {
    window_attributes: WindowAttributes,
}

impl EngineBuilder {
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

    /// Opens one window and processes events until a normal Windows close action.
    /// Window and event-loop resources are released before returning.
    pub fn run(self) -> Result<(), Error> {
        let mut application = Application {
            window_attributes: self.window_attributes,
            window: None,
            error: None,
        };
        let result = self.event_loop.run_app(&mut application);
        match application.error {
            Some(error) => Err(error),
            None => result.map_err(Error::Runtime),
        }
    }
}

struct Application {
    window_attributes: WindowAttributes,
    window: Option<Window>,
    error: Option<Error>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() || self.error.is_some() {
            return;
        }
        match event_loop.create_window(self.window_attributes.clone()) {
            Ok(window) => self.window = Some(window),
            Err(error) => {
                self.error = Some(Error::WindowCreation(error));
                event_loop.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.window.as_ref().map(Window::id) == Some(window_id)
            && matches!(event, WindowEvent::CloseRequested)
        {
            event_loop.exit();
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.window = None;
    }
}
