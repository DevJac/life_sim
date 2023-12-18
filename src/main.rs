use pollster::FutureExt as _;

struct LifeSim {
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

    fn draw_line(&self) {
        let surface_texture: wgpu::SurfaceTexture = self.surface.get_current_texture().unwrap();
        let surface_view: wgpu::TextureView = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let _render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("rennder pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
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
        self.queue.submit([command_encoder.finish()]);
        surface_texture.present();
    }
}

fn main() {
    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window: winit::window::Window = winit::window::Window::new(&event_loop).unwrap();
    let life_sim = LifeSim::new(window);
    life_sim.configure_surface();
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut fps_stats = life_sim::fps_stats::FPSStats::new(1.0, 10.0);
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
                winit::event::WindowEvent::Resized(_) => {
                    life_sim.configure_surface();
                }
                _ => {}
            },
            winit::event::Event::AboutToWait => {
                life_sim.draw_line();
                if fps_stats.update() {
                    let fps = 1.0 / fps_stats.mean();
                    let fps_std = fps_stats.std() / fps_stats.mean().powi(2);
                    let fps_99th = 1.0 / fps_stats.percentile_99();
                    log::info!("FPS: {:.0} ({:.0} ± {:.0})", fps_99th, fps, fps_std);
                }
            }
            _ => {}
        })
        .unwrap();
}
