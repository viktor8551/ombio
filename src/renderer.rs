use std::sync::{Arc, mpsc};

use wgpu::util::DeviceExt;
use winit::{event_loop::EventLoopProxy, window::Window};

use crate::{Error, Frame, Game};

// Two position components and three linear color components per vertex.
const RECTANGLE_VERTEX_SIZE: usize = 5 * size_of::<f32>();

pub(crate) struct Renderer {
    // The surface retains its own Arc to the window and is dropped first.
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    rectangle_pipeline: wgpu::RenderPipeline,
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
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("WALZER solid color"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/solid.wgsl").into()),
        });
        let rectangle_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("WALZER opaque rectangles"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: RECTANGLE_VERTEX_SIZE as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x3],
                }],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let renderer = Self {
            surface,
            device,
            queue,
            config,
            rectangle_pipeline,
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
        let mut vertices = Vec::new();
        let scale = window.scale_factor();
        for rectangle in &frame.rectangles {
            // Clip in physical pixels before converting to GPU clip coordinates.
            let left = (f64::from(rectangle.x) * scale).clamp(0.0, f64::from(size.width));
            let top = (f64::from(rectangle.y) * scale).clamp(0.0, f64::from(size.height));
            let right = ((f64::from(rectangle.x) + f64::from(rectangle.width)) * scale)
                .clamp(0.0, f64::from(size.width));
            let bottom = ((f64::from(rectangle.y) + f64::from(rectangle.height)) * scale)
                .clamp(0.0, f64::from(size.height));
            if left >= right || top >= bottom {
                continue;
            }
            let left = (2.0 * left / f64::from(size.width) - 1.0) as f32;
            let right = (2.0 * right / f64::from(size.width) - 1.0) as f32;
            let top = (1.0 - 2.0 * top / f64::from(size.height)) as f32;
            let bottom = (1.0 - 2.0 * bottom / f64::from(size.height)) as f32;
            let color = rectangle.color.linear();
            for (x, y) in [
                (left, top),
                (left, bottom),
                (right, top),
                (right, top),
                (left, bottom),
                (right, bottom),
            ] {
                for value in [x, y, color.r as f32, color.g as f32, color.b as f32] {
                    vertices.extend_from_slice(&value.to_ne_bytes());
                }
            }
        }
        let vertex_buffer = (!vertices.is_empty()).then(|| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("WALZER rectangle vertices"),
                    contents: &vertices,
                    usage: wgpu::BufferUsages::VERTEX,
                })
        });
        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("WALZER frame"),
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
            if let Some(buffer) = &vertex_buffer {
                pass.set_pipeline(&self.rectangle_pipeline);
                pass.set_vertex_buffer(0, buffer.slice(..));
                pass.draw(0..(vertices.len() / RECTANGLE_VERTEX_SIZE) as u32, 0..1);
            }
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
