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

fn dist_from_origin_squared(v: glam::Vec2) -> f32 {
    v.x.powi(2) + v.y.powi(2)
}

impl Creature {
    pub fn radius(&self) -> f32 {
        let mut radius_squared = 0.0;
        for segment in self.segments.iter() {
            let dfos = dist_from_origin_squared(segment.a);
            if dfos > radius_squared {
                radius_squared = dfos;
            }
            let dfos = dist_from_origin_squared(segment.b);
            if dfos > radius_squared {
                radius_squared = dfos;
            }
        }
        radius_squared.sqrt()
    }
}
