use life_sim::renderer::{Color, Line, Renderer};

fn main() {
    env_logger::init();
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window: winit::window::Window = winit::window::Window::new(&event_loop).unwrap();
    let mut renderer = Renderer::new(window);
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
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
                renderer.draw_line(Line::new(
                    glam::Vec2::new(0.0, 0.0),
                    glam::Vec2::new(10.0, 10.0),
                    Color(glam::Vec3::new(1.0, 1.0, 1.0)),
                ));
            }
            _ => {}
        })
        .unwrap();
}
