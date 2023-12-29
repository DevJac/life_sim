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
        let x_diff = self.a.x - self.b.x;
        let y_diff = self.a.y - self.b.y;
        (x_diff.powi(2) + y_diff.powi(2)).sqrt()
    }

    fn max_dist_from_origin_squared(&self) -> f32 {
        let a_dist = self.a.x.powi(2) + self.a.y.powi(2);
        let b_dist = self.b.x.powi(2) + self.b.y.powi(2);
        a_dist.max(b_dist)
    }
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
