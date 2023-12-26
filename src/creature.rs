#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentType {
    Energy,
    Attack,
    Defend,
    Move,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    pub a: glam::Vec2,
    pub b: glam::Vec2,
    pub t: SegmentType,
}

pub struct Creature {
    segments: Vec<Segment>,
}

impl Creature {
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }
}

impl Default for Creature {
    fn default() -> Self {
        let mut segments = Vec::with_capacity(15);
        segments.push(Segment {
            a: glam::Vec2::new(0.0, 0.0),
            b: glam::Vec2::new(0.0, 30.0),
            t: SegmentType::Energy,
        });
        segments.push(Segment {
            a: glam::Vec2::new(0.0, 30.0),
            b: glam::Vec2::new(30.0, 30.0),
            t: SegmentType::Attack,
        });
        Self { segments }
    }
}
