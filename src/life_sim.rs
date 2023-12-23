use crate::{
    creature::{Creature, SegmentType},
    renderer::{Line, Renderer},
};

fn segment_type_to_color(segment_type: SegmentType) -> glam::Vec3 {
    match segment_type {
        SegmentType::Energy => glam::Vec3::new(0.0, 1.0, 0.0),
        SegmentType::Attack => glam::Vec3::new(1.0, 0.0, 0.0),
        SegmentType::Defend => glam::Vec3::new(0.0, 0.0, 1.0),
        SegmentType::Move => glam::Vec3::new(1.0, 1.0, 0.0),
    }
}

fn creature_to_lines(creature: &Creature) -> Vec<Line> {
    creature
        .segments()
        .iter()
        .map(|segment| Line::new(segment.a, segment.b, segment_type_to_color(segment.t)))
        .collect()
}

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

    pub fn draw_creature(&mut self) {
        let creature = Creature::default();
        self.renderer.draw_lines(&creature_to_lines(&creature));
        self.renderer.present();
    }
}
