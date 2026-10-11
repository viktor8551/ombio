use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::Error;

pub(crate) fn run(
    event_loop: EventLoop<()>,
    window_attributes: WindowAttributes,
) -> Result<(), Error> {
    let mut application = Application {
        window_attributes,
        window: None,
        error: None,
    };
    let result = event_loop.run_app(&mut application);
    match application.error {
        Some(error) => Err(error),
        None => result.map_err(Error::Runtime),
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
