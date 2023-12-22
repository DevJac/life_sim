#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SegmentType {
    Energy,
    Attack,
    Defend,
    Move,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Segment {
    a: glam::Vec2,
    b: glam::Vec2,
    t: SegmentType,
}

pub struct Creature {
    segments: Vec<Segment>,
}

impl Creature {
    pub fn new() -> Self {
        let mut segments = Vec::with_capacity(15);
        segments.push(Segment {
            a: glam::Vec2::new(0.0, 0.0),
            b: glam::Vec2::new(1.0, 0.0),
            t: SegmentType::Energy,
        });
        Self { segments }
    }
}
