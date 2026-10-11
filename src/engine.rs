use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

use crate::{Error, runtime};

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
        runtime::run(self.event_loop, self.window_attributes)
    }
}
