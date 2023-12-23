use crate::renderer::{Line, Renderer};

pub struct LifeSim {
    renderer: Renderer,
}

impl LifeSim {
    pub fn new(window: winit::window::Window) -> Self {
        Self {
            renderer: Renderer::new(window),
        }
    }

    /// The underlying Renderer must be told when the window surface is resized.
    pub fn configure_surface(&self) {
        self.renderer.configure_surface();
    }

    pub fn draw_line(&mut self) {
        let lines = [
            Line::new(
                glam::Vec2::new(-0.5, -0.5),
                glam::Vec2::new(-0.5, 0.5),
                glam::Vec3::new(1.0, 0.0, 0.0),
            ),
            Line::new(
                glam::Vec2::new(-0.5, 0.5),
                glam::Vec2::new(0.5, 0.5),
                glam::Vec3::new(0.0, 1.0, 0.0),
            ),
            Line::new(
                glam::Vec2::new(0.5, 0.5),
                glam::Vec2::new(0.5, -0.5),
                glam::Vec3::new(0.0, 0.0, 1.0),
            ),
            Line::new(
                glam::Vec2::new(0.5, -0.5),
                glam::Vec2::new(-0.5, -0.5),
                glam::Vec3::new(1.0, 1.0, 1.0),
            ),
            Line::new(
                glam::Vec2::new(0.0, 0.0),
                glam::Vec2::new(0.5, 0.5),
                glam::Vec3::new(1.0, 1.0, 0.0),
            ),
        ];
        self.renderer.draw_lines(&lines);
        self.renderer.render();
    }
}
