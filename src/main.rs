use life_sim::life_sim::LifeSim;

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
