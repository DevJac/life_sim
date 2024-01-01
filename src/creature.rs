#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentType {
    Energy,
    Attack,
    Defend,
    Move,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SegmentLengths {
    pub energy: f32,
    pub attack: f32,
    pub defend: f32,
    pub move_: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    pub a: glam::Vec2,
    pub b: glam::Vec2,
    pub t: SegmentType,
}

impl Segment {
    pub fn length(&self) -> f32 {
        self.b.distance(self.a)
    }

    pub fn max_dist_from_origin_squared(&self) -> f32 {
        let a_dist = self.a.distance_squared(glam::Vec2::ZERO);
        let b_dist = self.b.distance_squared(glam::Vec2::ZERO);
        a_dist.max(b_dist)
    }

    pub fn midpoint(&self) -> glam::Vec2 {
        (self.b - self.a) / 2.0
    }
}

pub struct Creature {
    pub segments: Vec<Segment>,
    pub position: glam::Vec2,
    pub energy: f32,
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
            t: SegmentType::Move,
        });
        segments.push(Segment {
            a: glam::Vec2::new(30.0, 30.0),
            b: glam::Vec2::new(30.0, 0.0),
            t: SegmentType::Attack,
        });
        Self {
            segments,
            position: glam::Vec2::ZERO,
            energy: 0.0,
        }
    }
}

impl Creature {
    pub fn radius(&self) -> f32 {
        let mut radius_squared = 0.0;
        for segment in self.segments.iter() {
            let dfos = segment.max_dist_from_origin_squared();
            if dfos > radius_squared {
                radius_squared = dfos;
            }
        }
        radius_squared.sqrt()
    }

    pub fn segment_lengths(&self) -> SegmentLengths {
        let mut segment_lengths = SegmentLengths {
            energy: 0.0,
            attack: 0.0,
            defend: 0.0,
            move_: 0.0,
        };
        for segment in self.segments.iter() {
            match segment.t {
                SegmentType::Energy => segment_lengths.energy += segment.length(),
                SegmentType::Attack => segment_lengths.attack += segment.length(),
                SegmentType::Defend => segment_lengths.defend += segment.length(),
                SegmentType::Move => segment_lengths.move_ += segment.length(),
            }
        }
        segment_lengths
    }

    pub fn energy_income(&self) -> f32 {
        self.segment_lengths().energy
    }

    pub fn energy_requirement(&self) -> f32 {
        let segment_lengths = self.segment_lengths();
        segment_lengths.attack + segment_lengths.defend + segment_lengths.move_ + self.radius()
    }
}
