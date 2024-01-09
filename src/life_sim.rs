use crate::{
    creature::{Creature, SegmentType},
    renderer::{Color, Line, Renderer},
};

impl From<SegmentType> for Color {
    fn from(segment_type: SegmentType) -> Self {
        match segment_type {
            SegmentType::Energy => Color(glam::Vec3::new(0.0, 0.8, 0.0)),
            SegmentType::Attack => Color(glam::Vec3::new(1.0, 0.0, 0.0)),
            SegmentType::Defend => Color(glam::Vec3::new(0.0, 0.0, 1.0)),
            SegmentType::Move => Color(glam::Vec3::new(1.0, 0.9, 0.0)),
        }
    }
}

impl From<&Creature> for Vec<Line> {
    fn from(creature: &Creature) -> Self {
        creature
            .segments
            .iter()
            .map(|segment| {
                Line::new(
                    creature.position + segment.a,
                    creature.position + segment.b,
                    segment.t.into(),
                )
            })
            .collect()
    }
}

pub struct LifeSim {
    renderer: Renderer,
    creature: Creature,
}

impl LifeSim {
    pub fn new(window: winit::window::Window) -> Self {
        Self {
            renderer: Renderer::new(window),
            creature: Creature::default(),
        }
    }

    /// The underlying Renderer must be told when the window surface is resized.
    pub fn configure_surface(&self) {
        self.renderer.configure_surface();
    }

    pub fn draw_creature(&mut self, delta_time: f32) {
        self.creature.update(delta_time, self.renderer.world_size());
        self.renderer
            .draw_lines(&Into::<Vec<Line>>::into(&self.creature));
        self.renderer.present();
    }
}
