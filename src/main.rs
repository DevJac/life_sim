use pollster::FutureExt as _;

fn main() {
    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window: winit::window::Window = winit::window::Window::new(&event_loop).unwrap();
    let mut renderer = Renderer::new(window);
    renderer.configure_surface();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut loop_count = 0;
    event_loop
        .run(move |event, event_loop_window_target| match event {
            winit::event::Event::WindowEvent {
                window_id: _,
                event: window_event,
            } => match window_event {
                winit::event::WindowEvent::CloseRequested
                | winit::event::WindowEvent::KeyboardInput {
                    device_id: _,
                    is_synthetic: _,
                    event:
                        winit::event::KeyEvent {
                            physical_key: _,
                            logical_key:
                                winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                            text: _,
                            location: _,
                            state: _,
                            repeat: _,
                            ..
                        },
                } => {
                    event_loop_window_target.exit();
                }
                _ => {}
            },
            winit::event::Event::AboutToWait => {
                loop_count += 1;
                renderer.draw_line(Line::new(
                    glam::Vec2::new(0.0, 0.0),
                    glam::Vec2::new(10.0, 10.0),
                    Color(glam::Vec3::new(1.0, 1.0, 1.0)),
                ));
                if loop_count % 10_123 == 0 {
                    let wgpu_report = renderer.instance.generate_report();
                    println!("Instance report: {:?}", wgpu_report);
                    event_loop_window_target.exit();
                }
            }
            _ => {}
        })
        .unwrap();
}

pub struct Color(pub glam::Vec3);

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Line {
    a: glam::Vec2,
    b: glam::Vec2,
    /// RGB color channels. Each channel should be between 0.0 and 1.0.
    color: glam::Vec4,
}

impl Line {
    pub fn new(a: glam::Vec2, b: glam::Vec2, color: Color) -> Self {
        let color = glam::Vec4::new(color.0.x, color.0.y, color.0.z, 1.0);
        Self { a, b, color }
    }
}

fn empty_command_buffer(
    device: &wgpu::Device,
    texture_view: &wgpu::TextureView,
) -> wgpu::CommandBuffer {
    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let _render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("empty render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: texture_view,
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
    }
    command_encoder.finish()
}

/// Encodes commands to draw the given lines to the given texture. Returns a CommandBuffer.
fn draw_lines(
    device: &wgpu::Device,
    preferred_texture_format: wgpu::TextureFormat,
    lines: &[Line],
    texture: &wgpu::Texture,
) -> wgpu::CommandBuffer {
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    if lines.is_empty() {
        return empty_command_buffer(device, &texture_view);
    }
    let sample_count = 4;
    let multisample_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("line multisample texture"),
        size: texture.size(),
        mip_level_count: 1,
        sample_count,
        dimension: wgpu::TextureDimension::D2,
        format: preferred_texture_format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let multisample_texture_view =
        multisample_texture.create_view(&wgpu::TextureViewDescriptor::default());
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
        multisample: wgpu::MultisampleState {
            count: sample_count,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
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
    let world_space_size =
        glam::Vec2::new(texture.width() as f32 / 2.0, texture.height() as f32 / 2.0);
    let world_space_size_bytes: &[u8] = bytemuck::bytes_of(&world_space_size);
    let world_space_size_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("line world space size buffer"),
        size: 32.max(world_space_size_bytes.len() as u64),
        usage: wgpu::BufferUsages::UNIFORM,
        mapped_at_creation: true,
    });
    world_space_size_buffer
        .slice(0..world_space_size_bytes.len() as u64)
        .get_mapped_range_mut()
        .copy_from_slice(world_space_size_bytes);
    world_space_size_buffer.unmap();
    let line_bytes: &[u8] = bytemuck::cast_slice(lines);
    let line_storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("line storage buffer"),
        size: 32.max(line_bytes.len() as u64),
        usage: wgpu::BufferUsages::STORAGE,
        mapped_at_creation: true,
    });
    line_storage_buffer
        .slice(0..line_bytes.len() as u64)
        .get_mapped_range_mut()
        .copy_from_slice(line_bytes);
    line_storage_buffer.unmap();
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("line bind group"),
        layout: &render_pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &world_space_size_buffer,
                    offset: 0,
                    size: None,
                }),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &line_storage_buffer,
                    offset: 0,
                    size: None,
                }),
            },
        ],
    });
    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("lines render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &multisample_texture_view,
                resolve_target: Some(&texture_view),
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
        render_pass.draw(0..2, 0..lines.len() as u32);
    }
    command_encoder.finish()
}

pub struct Renderer {
    // WGPU Stuff
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub preferred_texture_format: wgpu::TextureFormat,
    // Safety: The window must life longer than its surface. Drop window last.
    pub window: winit::window::Window,
}

impl Renderer {
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
            *surface.get_capabilities(&adapter).formats.first().unwrap();
        log::debug!("Preferred texture format: {:?}", &preferred_texture_format);
        Self {
            instance,
            device,
            queue,
            surface,
            preferred_texture_format,
            window,
        }
    }

    /// The window surface must be reconfigured when the window changes size.
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

    pub fn draw_line(&mut self, line: Line) {
        let surface_texture: wgpu::SurfaceTexture = self.surface.get_current_texture().unwrap();
        let draw_lines_command_buffer = draw_lines(
            &self.device,
            self.preferred_texture_format,
            &[line],
            &surface_texture.texture,
        );
        self.queue.submit([draw_lines_command_buffer]);
        surface_texture.present();
    }
}
