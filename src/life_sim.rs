use pollster::FutureExt as _;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Line {
    a: glam::Vec2,
    b: glam::Vec2,
    /// RGB color channels. Each channel should be between 0.0 and 1.0.
    color: glam::Vec4,
}

/// Encodes commands to draw the given lines to the given texture. Returns a CommandBuffer.
pub fn draw_lines(
    device: &wgpu::Device,
    preferred_texture_format: wgpu::TextureFormat,
    lines: &[Line],
    texture_view: &wgpu::TextureView,
) -> wgpu::CommandBuffer {
    let shader_module = device.create_shader_module(wgpu::include_wgsl!("shaders/line.wgsl"));
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("lines render pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: "vertex_main",
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::LineList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: "fragment_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: preferred_texture_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
    });
    let lines_bytes: &[u8] = bytemuck::cast_slice(lines);
    let line_storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("line instance buffer"),
        size: lines_bytes.len() as u64,
        usage: wgpu::BufferUsages::STORAGE,
        mapped_at_creation: true,
    });
    line_storage_buffer
        .slice(..)
        .get_mapped_range_mut()
        .copy_from_slice(lines_bytes);
    line_storage_buffer.unmap();
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("line bind group"),
        layout: &render_pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &line_storage_buffer,
                offset: 0,
                size: None,
            }),
        }],
    });
    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("lines render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
    }
    command_encoder.finish()
}

pub struct LifeSim {
    // WGPU Stuff
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    preferred_texture_format: wgpu::TextureFormat,
    // Safety: The window must life longer than its surface. Drop window last.
    window: winit::window::Window,
}

impl LifeSim {
    pub fn new(window: winit::window::Window) -> Self {
        let instance: wgpu::Instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        // Safety: The window must live longer than its surface.
        let surface: wgpu::Surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter: wgpu::Adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .block_on()
            .unwrap();
        let (device, queue): (wgpu::Device, wgpu::Queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .block_on()
            .unwrap();
        let preferred_texture_format: wgpu::TextureFormat =
            *surface.get_capabilities(&adapter).formats.get(0).unwrap();
        log::debug!("Preferred texture format: {:?}", &preferred_texture_format);
        Self {
            device,
            queue,
            surface,
            preferred_texture_format,
            window,
        }
    }

    pub fn configure_surface(&self) {
        let window_inner_size = self.window.inner_size();
        self.surface.configure(
            &self.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.preferred_texture_format,
                width: window_inner_size.width,
                height: window_inner_size.height,
                present_mode: wgpu::PresentMode::AutoNoVsync,
                // The window surface does not support alpha
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
            },
        );
    }

    pub fn draw_line(&self) {
        let surface_texture: wgpu::SurfaceTexture = self.surface.get_current_texture().unwrap();
        let surface_texture_view: wgpu::TextureView = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let draw_lines_command_buffer = draw_lines(
            &self.device,
            self.preferred_texture_format,
            &[],
            &surface_texture_view,
        );
        self.queue.submit([draw_lines_command_buffer]);
        surface_texture.present();
    }
}
