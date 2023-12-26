use crate::{
    creature::{Creature, SegmentType},
    renderer::{Line, Renderer},
};

impl From<SegmentType> for glam::Vec3 {
    fn from(segment_type: SegmentType) -> Self {
        match segment_type {
            SegmentType::Energy => glam::Vec3::new(0.0, 1.0, 0.0),
            SegmentType::Attack => glam::Vec3::new(1.0, 0.0, 0.0),
            SegmentType::Defend => glam::Vec3::new(0.0, 0.0, 1.0),
            SegmentType::Move => glam::Vec3::new(1.0, 1.0, 0.0),
        }
    }
}

impl From<Creature> for Vec<Line> {
    fn from(creature: Creature) -> Self {
        creature
            .segments()
            .iter()
            .map(|segment| Line::new(segment.a, segment.b, segment.t.into()))
            .collect()
    }
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
        self.renderer.draw_lines(&Into::<Vec<Line>>::into(creature));
        self.renderer.present();
    }
}
