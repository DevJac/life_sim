use pollster::FutureExt as _;

/// We will use normalized device coordinates (NDC) as our world coordinates,
/// because NDC is as good a world coordinate system as any other.
/// This means coordinates between -1 and 1 on both the x and y axis.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Line {
    a: glam::Vec2,
    b: glam::Vec2,
    /// RGB color channels. Each channel should be between 0.0 and 1.0.
    color: glam::Vec3,
}

/// Encodes commands to draw the given lines to the given texture. Returns a CommandBuffer.
pub fn draw_lines(
    device: &wgpu::Device,
    lines: &[Line],
    texture_view: &wgpu::TextureView,
) -> wgpu::CommandBuffer {
    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let _render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("rennder pass"),
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
        let draw_lines_command_buffer = draw_lines(&self.device, &[], &surface_texture_view);
        self.queue.submit([draw_lines_command_buffer]);
        surface_texture.present();
    }
}
