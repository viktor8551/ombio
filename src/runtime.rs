use std::{error::Error, sync::Arc};

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

use crate::{Frame, Game, render::Renderer};

type RunError = Box<dyn Error>;

/// Open a window and draw the game until it is closed.
///
/// Call this once from the main thread. Startup and surface errors are returned
/// to the caller. The web entry point will be supplied by a later milestone.
pub fn run(game: impl Game) -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = Application {
        game,
        graphics: None,
        error: None,
    };
    event_loop.run_app(&mut app)?;
    match app.error {
        Some(error) => Err(error),
        None => Ok(()),
    }
}

struct Application<G> {
    game: G,
    graphics: Option<WindowGraphics>,
    error: Option<RunError>,
}

impl<G: Game> Application<G> {
    fn fail(&mut self, event_loop: &ActiveEventLoop, error: RunError) {
        self.error = Some(error);
        event_loop.exit();
    }
}

impl<G: Game> ApplicationHandler for Application<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.graphics.is_some() {
            return;
        }
        let result = (|| {
            let window = Arc::new(
                event_loop.create_window(
                    Window::default_attributes()
                        .with_title("Walzer - rectangle")
                        .with_inner_size(LogicalSize::new(800.0, 600.0)),
                )?,
            );
            pollster::block_on(WindowGraphics::new(window))
        })();
        match result {
            Ok(graphics) => {
                graphics.window.request_redraw();
                self.graphics = Some(graphics);
            }
            Err(error) => self.fail(event_loop, error),
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let Some(graphics) = &mut self.graphics else {
            return;
        };
        if id != graphics.window.id() {
            return;
        }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                graphics.configure();
                graphics.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if let Err(error) = graphics.draw(&mut self.game) {
                    self.fail(event_loop, error);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(graphics) = &self.graphics {
            let size = graphics.window.inner_size();
            if size.width > 0 && size.height > 0 {
                graphics.window.request_redraw();
            }
        }
    }
}

struct WindowGraphics {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    renderer: Renderer,
}

impl WindowGraphics {
    async fn new(window: Arc<Window>) -> Result<Self, RunError> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone())?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await?;
        let (device, queue) = adapter.request_device(&Default::default()).await?;
        let size = window.inner_size();
        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .ok_or("GPU adapter cannot present to this window")?;
        if let Some(format) = surface
            .get_capabilities(&adapter)
            .formats
            .into_iter()
            .find(|format| format.is_srgb())
        {
            config.format = format;
        }
        config.present_mode = wgpu::PresentMode::Fifo;
        let renderer = Renderer::new(device, queue, config.format);
        let mut graphics = Self {
            window,
            surface,
            config,
            renderer,
        };
        graphics.configure();
        Ok(graphics)
    }

    fn configure(&mut self) {
        let size = self.window.inner_size();
        if size.width == 0 || size.height == 0 {
            return;
        }
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.renderer.device, &self.config);
    }

    fn draw(&mut self, game: &mut impl Game) -> Result<(), RunError> {
        let size = self.window.inner_size();
        if size.width == 0 || size.height == 0 {
            return Ok(());
        }
        let (texture, reconfigure) = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => (texture, false),
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => (texture, true),
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure();
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => return Err("Rendering surface was lost".into()),
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err("Rendering surface validation failed".into());
            }
        };
        let logical_size = size.to_logical::<f32>(self.window.scale_factor());
        let mut frame = Frame::new([logical_size.width, logical_size.height]);
        game.draw(&mut frame);
        self.renderer
            .draw(&frame, &texture.texture.create_view(&Default::default()));
        self.window.pre_present_notify();
        self.renderer.present(texture);
        if reconfigure {
            self.configure();
        }
        Ok(())
    }
}
