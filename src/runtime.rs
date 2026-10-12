use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{Error, Game, renderer::Renderer};

pub(crate) fn run(
    event_loop: EventLoop<()>,
    window_attributes: WindowAttributes,
    game: impl Game,
) -> Result<(), Error> {
    let mut application = Application {
        proxy: event_loop.create_proxy(),
        window_attributes,
        window: None,
        renderer: None,
        game,
        error: None,
    };
    let result = event_loop.run_app(&mut application);
    match application.error {
        Some(error) => Err(error),
        None => result.map_err(Error::Runtime),
    }
}

struct Application<G> {
    proxy: EventLoopProxy<()>,
    window_attributes: WindowAttributes,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    game: G,
    error: Option<Error>,
}

impl<G: Game> Application<G> {
    fn fail(&mut self, event_loop: &ActiveEventLoop, error: Error) {
        self.error = Some(error);
        event_loop.exit();
    }
}

impl<G: Game> ApplicationHandler for Application<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() || self.error.is_some() {
            return;
        }
        match event_loop.create_window(self.window_attributes.clone()) {
            Ok(window) => {
                let window = Arc::new(window);
                match pollster::block_on(Renderer::new(window.clone(), self.proxy.clone())) {
                    Ok(renderer) => {
                        self.renderer = Some(renderer);
                        window.request_redraw();
                        self.window = Some(window);
                    }
                    Err(error) => self.fail(event_loop, error),
                }
            }
            Err(error) => self.fail(event_loop, Error::WindowCreation(error)),
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, _event: ()) {
        if let Some(renderer) = &self.renderer
            && let Err(error) = renderer.check_error()
        {
            self.fail(event_loop, error);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.error.is_some() || self.window.as_ref().map(|window| window.id()) != Some(window_id)
        {
            return;
        }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(renderer)) = (&self.window, &mut self.renderer)
                    && let Err(error) = renderer.render(window, &mut self.game)
                {
                    self.fail(event_loop, error);
                }
            }
            WindowEvent::Resized(_) => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if self.error.is_none()
            && let Some(renderer) = &self.renderer
            && let Err(error) = renderer.check_error()
        {
            self.error = Some(error);
        }
        self.renderer = None;
        self.window = None;
    }
}
