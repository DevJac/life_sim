use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let _window = Window::new(&event_loop);
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, event_loop_window_target| match event {
            winit::event::Event::WindowEvent {
                window_id: _,
                event: window_event,
            } => match window_event {
                winit::event::WindowEvent::CloseRequested
                | winit::event::WindowEvent::KeyboardInput {
                    device_id: _,
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
                    is_synthetic: _,
                } => {
                    event_loop_window_target.exit();
                }
                _ => {}
            },
            _ => {}
        })
        .unwrap();
}
