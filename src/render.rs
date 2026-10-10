use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::Frame;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct RectangleInstance {
    position: [f32; 2],
    size: [f32; 2],
    color: [f32; 4],
}

pub(crate) struct Renderer {
    pub(crate) device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub(crate) fn present(&self, texture: wgpu::SurfaceTexture) {
        self.queue.present(texture);
    }

    pub(crate) fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("rectangle shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("rectangle.wgsl").into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("rectangle pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<RectangleInstance>() as u64,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4],
                })],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        Self {
            device,
            queue,
            pipeline,
        }
    }

    pub(crate) fn draw(&self, frame: &Frame, target: &wgpu::TextureView) {
        let viewport = [frame.viewport[0], frame.viewport[1], 0.0, 0.0];
        let viewport_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("logical viewport"),
                contents: bytemuck::cast_slice(&viewport),
                usage: wgpu::BufferUsages::UNIFORM,
            });
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rectangle viewport"),
            layout: &self.pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: viewport_buffer.as_entire_binding(),
            }],
        });
        let instances: Vec<_> = frame
            .rectangles
            .iter()
            .map(|rectangle| RectangleInstance {
                position: rectangle.position,
                size: rectangle.size,
                color: rectangle.color,
            })
            .collect();
        let instance_buffer = (!instances.is_empty()).then(|| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("rectangles"),
                    contents: bytemuck::cast_slice(&instances),
                    usage: wgpu::BufferUsages::VERTEX,
                })
        });
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("rectangle commands"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("rectangle pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            if let Some(buffer) = &instance_buffer {
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &bind_group, &[]);
                pass.set_vertex_buffer(0, buffer.slice(..));
                pass.draw(0..6, 0..instances.len() as u32);
            }
        }
        self.queue.submit([encoder.finish()]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Frame, Game, Rectangle};

    #[test]
    fn game_draws_a_colored_rectangle_in_logical_pixels() {
        struct Demo;
        impl Game for Demo {
            fn draw(&mut self, frame: &mut Frame) {
                assert_eq!(frame.viewport(), [32.0, 24.0]);
                frame.draw_rectangle(Rectangle {
                    position: [4.0, 3.0],
                    size: [8.0, 6.0],
                    color: [0.0, 1.0, 0.0, 1.0],
                });
            }
        }
        let instance = wgpu::Instance::default();
        let adapter = pollster::block_on(instance.request_adapter(&Default::default()))
            .expect("GPU adapter required for the rendering test");
        let (device, queue) = pollster::block_on(adapter.request_device(&Default::default()))
            .expect("GPU device required for the rendering test");
        let target = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("rectangle test target"),
            size: wgpu::Extent3d {
                width: 64,
                height: 48,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rectangle test pixels"),
            size: 256 * 48,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let renderer = Renderer::new(device, queue, wgpu::TextureFormat::Rgba8Unorm);
        let mut frame = Frame::new([32.0, 24.0]);
        Demo.draw(&mut frame);
        renderer.draw(&frame, &target.create_view(&Default::default()));
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            target.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(48),
                },
            },
            wgpu::Extent3d {
                width: 64,
                height: 48,
                depth_or_array_layers: 1,
            },
        );
        renderer.queue.submit([encoder.finish()]);
        let (sender, receiver) = std::sync::mpsc::channel();
        readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                sender.send(result).unwrap();
            });
        renderer
            .device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
        receiver.recv().unwrap().unwrap();
        let pixels = readback.slice(..).get_mapped_range().unwrap();
        for (x, y, expected) in [
            (8, 6, [0, 255, 0, 255]),
            (23, 17, [0, 255, 0, 255]),
            (7, 6, [0, 0, 0, 255]),
            (24, 17, [0, 0, 0, 255]),
            (8, 5, [0, 0, 0, 255]),
            (23, 18, [0, 0, 0, 255]),
        ] {
            let offset = y * 256 + x * 4;
            assert_eq!(&pixels[offset..offset + 4], &expected, "pixel ({x}, {y})");
        }
    }
}
