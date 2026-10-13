use std::sync::{Arc, mpsc};

use wgpu::util::DeviceExt;
use winit::{event_loop::EventLoopProxy, window::Window};

use crate::{Error, Frame, Game, Image, frame::DrawCommand};

// Two position components and four linear color components per vertex.
const VERTEX_SIZE: usize = 6 * size_of::<f32>();

pub(crate) struct GpuImage {
    pub(crate) device: wgpu::Device,
    pub(crate) bind_group: wgpu::BindGroup,
}

pub(crate) struct Renderer {
    // The surface retains its own Arc to the window and is dropped first.
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    surface_configured: bool,
    rectangle_pipeline: wgpu::RenderPipeline,
    sprite_pipeline: wgpu::RenderPipeline,
    sprite_layout: wgpu::BindGroupLayout,
    sprite_sampler: wgpu::Sampler,
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
            label: Some("WALZER rectangles"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: VERTEX_SIZE as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4],
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
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let sprite_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("WALZER sprite"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/sprite.wgsl").into()),
        });
        let sprite_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("WALZER sprite texture"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let sprite_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("WALZER sprite pipeline layout"),
                bind_group_layouts: &[Some(&sprite_layout)],
                immediate_size: 0,
            });
        let sprite_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("WALZER sprites"),
            layout: Some(&sprite_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &sprite_shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: VERTEX_SIZE as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
                }],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &sprite_shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let sprite_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("WALZER sprite sampler"),
            ..Default::default()
        });
        let renderer = Self {
            surface,
            device,
            queue,
            config,
            surface_configured: false,
            rectangle_pipeline,
            sprite_pipeline,
            sprite_layout,
            sprite_sampler,
            errors,
        };
        renderer.check_error()?;
        Ok(renderer)
    }

    pub(crate) fn check_error(&self) -> Result<(), Error> {
        match self.errors.try_recv() {
            Ok(error) => Err(error),
            Err(_) => Ok(()),
        }
    }

    fn image_bind_group(&self, image: &Image) -> Result<wgpu::BindGroup, Error> {
        let data = &image.data;
        let mut cached = data
            .gpu
            .lock()
            .map_err(|_| Error::Rendering("sprite GPU cache lock was poisoned".into()))?;
        if let Some(gpu) = cached.as_ref()
            && gpu.device == self.device
        {
            return Ok(gpu.bind_group.clone());
        }
        let limit = self.device.limits().max_texture_dimension_2d;
        if data.width > limit || data.height > limit {
            return Err(Error::Rendering(format!(
                "sprite dimensions {}x{} exceed the GPU texture limit {limit}",
                data.width, data.height,
            )));
        }
        let size = wgpu::Extent3d {
            width: data.width,
            height: data.height,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("WALZER sprite image"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.queue.write_texture(
            texture.as_image_copy(),
            &data.pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(data.width * 4),
                rows_per_image: Some(data.height),
            },
            size,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("WALZER sprite"),
            layout: &self.sprite_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sprite_sampler),
                },
            ],
        });
        self.check_error()?;
        *cached = Some(GpuImage {
            device: self.device.clone(),
            bind_group: bind_group.clone(),
        });
        Ok(bind_group)
    }

    pub(crate) fn render(&mut self, window: &Window, game: &mut impl Game) -> Result<(), Error> {
        self.check_error()?;
        let size = window.inner_size();
        if window.is_minimized() == Some(true) || size.width == 0 || size.height == 0 {
            return Ok(());
        }
        // Configure only when a usable drawable area is available, including the first frame.
        if !self.surface_configured
            || (self.config.width, self.config.height) != (size.width, size.height)
        {
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
            self.check_error()?;
            self.surface_configured = true;
        }
        let (texture, suboptimal) = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => (texture, false),
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => (texture, true),
            wgpu::CurrentSurfaceTexture::Timeout => return Ok(()),
            wgpu::CurrentSurfaceTexture::Occluded => return Ok(()),
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                self.check_error()?;
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
        let mut draws = Vec::new();
        let scale = window.scale_factor();
        for command in &frame.commands {
            let (x, y, width, height) = match command {
                DrawCommand::Rectangle(rectangle) => {
                    (rectangle.x, rectangle.y, rectangle.width, rectangle.height)
                }
                DrawCommand::Sprite { image, x, y } => {
                    (*x, *y, image.data.width as f32, image.data.height as f32)
                }
            };
            // Clip in physical pixels before converting to GPU clip coordinates.
            let origin_x = f64::from(x) * scale;
            let origin_y = f64::from(y) * scale;
            let physical_width = f64::from(width) * scale;
            let physical_height = f64::from(height) * scale;
            let left = origin_x.clamp(0.0, f64::from(size.width));
            let top = origin_y.clamp(0.0, f64::from(size.height));
            let right = (origin_x + physical_width).clamp(0.0, f64::from(size.width));
            let bottom = (origin_y + physical_height).clamp(0.0, f64::from(size.height));
            if left >= right || top >= bottom {
                continue;
            }
            // Clipping crops a sprite's UVs instead of stretching its remaining image.
            let u_left = ((left - origin_x) / physical_width) as f32;
            let u_right = ((right - origin_x) / physical_width) as f32;
            let v_top = ((top - origin_y) / physical_height) as f32;
            let v_bottom = ((bottom - origin_y) / physical_height) as f32;
            let left = (2.0 * left / f64::from(size.width) - 1.0) as f32;
            let right = (2.0 * right / f64::from(size.width) - 1.0) as f32;
            let top = (1.0 - 2.0 * top / f64::from(size.height)) as f32;
            let bottom = (1.0 - 2.0 * bottom / f64::from(size.height)) as f32;
            let bind_group = match command {
                DrawCommand::Sprite { image, .. } => Some(self.image_bind_group(image)?),
                DrawCommand::Rectangle(_) => None,
            };
            let start = (vertices.len() / VERTEX_SIZE) as u32;
            for (x, y, u, v) in [
                (left, top, u_left, v_top),
                (left, bottom, u_left, v_bottom),
                (right, top, u_right, v_top),
                (right, top, u_right, v_top),
                (left, bottom, u_left, v_bottom),
                (right, bottom, u_right, v_bottom),
            ] {
                let attributes = match command {
                    DrawCommand::Rectangle(rectangle) => {
                        let color = rectangle.color.linear();
                        [
                            color.r as f32,
                            color.g as f32,
                            color.b as f32,
                            color.a as f32,
                        ]
                    }
                    DrawCommand::Sprite { .. } => [u, v, 0.0, 0.0],
                };
                for value in [
                    x,
                    y,
                    attributes[0],
                    attributes[1],
                    attributes[2],
                    attributes[3],
                ] {
                    vertices.extend_from_slice(&value.to_ne_bytes());
                }
            }
            draws.push((start..start + 6, bind_group));
        }
        let vertex_buffer = (!vertices.is_empty()).then(|| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("WALZER frame vertices"),
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
                pass.set_vertex_buffer(0, buffer.slice(..));
                for (range, bind_group) in &draws {
                    if let Some(bind_group) = bind_group {
                        pass.set_pipeline(&self.sprite_pipeline);
                        pass.set_bind_group(0, bind_group, &[]);
                    } else {
                        pass.set_pipeline(&self.rectangle_pipeline);
                    }
                    pass.draw(range.clone(), 0..1);
                }
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
