use std::time::{Duration, Instant};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    keyboard::PhysicalKey,
    window::{WindowAttributes, WindowId},
};

use crate::{Context, Error, Game, Graphics};

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
        game,
        error: None,
        context: None,
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
    game: G,
    error: Option<Error>,
    context: Option<Context>,
    next_update: Option<Instant>,
}

impl<G: Game> Application<G> {
    fn schedule_redraw(&mut self) {
        if self.error.is_none()
            && let Some(context) = &self.context
        {
            context.graphics().request_redraw();
        }
    }

    fn update_if_due(&mut self, now: Instant) -> bool {
        if self.error.is_some() || self.next_update.is_some_and(|deadline| now < deadline) {
            return false;
        }
        let Some(context) = &mut self.context else {
            return false;
        };
        context.advance(now);
        self.game.update(context);
        context.input_mut().finish_update();
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
        if self.context.is_some() || self.error.is_some() {
            self.schedule_redraw();
            return;
        }
        match event_loop.create_window(self.window_attributes.clone()) {
            Ok(window) => match pollster::block_on(Graphics::new(window, self.proxy.clone())) {
                Ok(graphics) => {
                    self.context = Some(Context::new(graphics));
                    self.schedule_redraw();
                }
                Err(error) => self.fail(event_loop, error),
            },
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
        if let Some(context) = &self.context
            && let Err(error) = context.graphics().check_error()
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
        if self.error.is_some() {
            return;
        }
        let Some(context) = &mut self.context else {
            return;
        };
        if context.graphics().window_id() != window_id {
            return;
        }
        context.input_mut().handle_mouse_event(&event);
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(key) = event.physical_key {
                    context.input_mut().handle_key(key, event.state);
                }
            }
            WindowEvent::Focused(false) => context.input_mut().release_all(),
            WindowEvent::RedrawRequested => {
                if !context.graphics().can_render() {
                    return;
                }
                self.update_if_due(Instant::now());
                if let Some(context) = &mut self.context
                    && let Err(error) = context.graphics_mut().render(&mut self.game)
                {
                    self.fail(event_loop, error);
                }
            }
            WindowEvent::Occluded(occluded) => {
                context.graphics_mut().set_occluded(occluded);
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
            && let Some(context) = &self.context
            && let Err(error) = context.graphics().check_error()
        {
            self.error = Some(error);
        }
        self.context = None;
    }
}
