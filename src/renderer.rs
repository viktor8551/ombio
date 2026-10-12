use std::sync::{Arc, mpsc};

use winit::{event_loop::EventLoopProxy, window::Window};

use crate::{Error, Frame, Game};

pub(crate) struct Renderer {
    // The surface retains its own Arc to the window and is dropped first.
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    errors: mpsc::Receiver<Error>,
}

impl Renderer {
    pub(crate) async fn new(window: Arc<Window>, proxy: EventLoopProxy<()>) -> Result<Self, Error> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let surface = instance
            .create_surface(window.clone())
            .map_err(Error::SurfaceCreation)?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .map_err(Error::AdapterRequest)?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .map_err(Error::DeviceRequest)?;

        let (sender, errors) = mpsc::channel();
        let error_sender = sender.clone();
        let error_proxy = proxy.clone();
        device.on_uncaptured_error(Arc::new(move |error: wgpu::Error| {
            let _ = error_sender.send(Error::Rendering(error.to_string()));
            let _ = error_proxy.send_event(());
        }));
        device.set_device_lost_callback(move |reason, message| {
            if reason != wgpu::DeviceLostReason::Destroyed {
                let _ = sender.send(Error::Rendering(format!("device lost: {message}")));
                let _ = proxy.send_event(());
            }
        });

        let size = window.inner_size();
        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .ok_or_else(|| {
                Error::GpuInitialization("surface is unsupported by the adapter".into())
            })?;
        config.format = surface
            .get_capabilities(&adapter)
            .formats
            .into_iter()
            .find(wgpu::TextureFormat::is_srgb)
            .ok_or_else(|| Error::GpuInitialization("surface has no sRGB format".into()))?;
        let renderer = Self {
            surface,
            device,
            queue,
            config,
            errors,
        };
        renderer
            .surface
            .configure(&renderer.device, &renderer.config);
        renderer.check_error()?;
        Ok(renderer)
    }

    pub(crate) fn check_error(&self) -> Result<(), Error> {
        match self.errors.try_recv() {
            Ok(error) => Err(error),
            Err(_) => Ok(()),
        }
    }

    pub(crate) fn render(&mut self, window: &Window, game: &mut impl Game) -> Result<(), Error> {
        self.check_error()?;
        let size = window.inner_size();
        if size.width == 0 || size.height == 0 {
            return Ok(());
        }
        if (self.config.width, self.config.height) != (size.width, size.height) {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
            self.check_error()?;
        }
        let (texture, suboptimal) = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => (texture, false),
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => (texture, true),
            wgpu::CurrentSurfaceTexture::Timeout => {
                window.request_redraw();
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Occluded => return Ok(()),
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                self.check_error()?;
                window.request_redraw();
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                return Err(Error::Rendering("surface lost".into()));
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                self.check_error()?;
                return Err(Error::Rendering("surface texture validation failed".into()));
            }
        };
        let mut frame = Frame::new();
        game.render(&mut frame);
        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("WALZER background"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(frame.background.linear()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }
        self.queue.submit([encoder.finish()]);
        self.check_error()?;
        window.pre_present_notify();
        texture.present();
        if suboptimal {
            self.surface.configure(&self.device, &self.config);
        }
        self.check_error()
    }
}
