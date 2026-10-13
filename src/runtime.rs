use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes, WindowId},
};

use crate::{Context, Error, Game, renderer::Renderer};

// Target roughly 60 updates per second
const UPDATE_INTERVAL: Duration = Duration::from_secs(1).checked_div(60).unwrap();

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
        context: Context::new(),
        occluded: false,
        next_update: None,
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
    context: Context,
    occluded: bool,
    next_update: Option<Instant>,
}

impl<G: Game> Application<G> {
    fn can_render(&self) -> bool {
        !self.occluded
            && self.error.is_none()
            && self.window.as_ref().is_some_and(|window| {
                let size = window.inner_size();
                window.is_visible() != Some(false)
                    && window.is_minimized() != Some(true)
                    && size.width > 0
                    && size.height > 0
            })
    }

    fn schedule_redraw(&mut self) {
        if self.can_render()
            && let Some(window) = &self.window
        {
            window.request_redraw();
        }
    }

    fn update_if_due(&mut self, now: Instant) -> bool {
        if self.error.is_some()
            || self.window.is_none()
            || self.next_update.is_some_and(|deadline| now < deadline)
        {
            return false;
        }
        self.context.advance(now);
        self.game.update(&self.context);
        self.context.input_mut().finish_update();
        self.next_update = Some(Instant::now() + UPDATE_INTERVAL);
        true
    }

    fn fail(&mut self, event_loop: &ActiveEventLoop, error: Error) {
        self.error = Some(error);
        event_loop.exit();
    }
}

impl<G: Game> ApplicationHandler for Application<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() || self.error.is_some() {
            self.schedule_redraw();
            return;
        }
        match event_loop.create_window(self.window_attributes.clone()) {
            Ok(window) => {
                let window = Arc::new(window);
                self.context
                    .input_mut()
                    .set_scale_factor(window.scale_factor());
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

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if event_loop.exiting() {
            return;
        }
        if self.update_if_due(Instant::now()) {
            self.schedule_redraw();
        }
        event_loop.set_control_flow(match self.next_update {
            Some(deadline) if self.error.is_none() => ControlFlow::WaitUntil(deadline),
            _ => ControlFlow::Wait,
        });
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
        self.context.input_mut().handle_mouse_event(&event);
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    self.context.input_mut().handle_key(key, event.state);
                }
            }
            WindowEvent::Focused(false) => self.context.input_mut().release_all(),
            WindowEvent::RedrawRequested => {
                if !self.can_render() {
                    return;
                }
                self.update_if_due(Instant::now());
                if let (Some(window), Some(renderer)) = (&self.window, &mut self.renderer)
                    && let Err(error) = renderer.render(window, &mut self.game)
                {
                    self.fail(event_loop, error);
                }
            }
            WindowEvent::Occluded(occluded) => {
                self.occluded = occluded;
                self.schedule_redraw();
            }
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                self.schedule_redraw()
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
