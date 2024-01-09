use rand::Rng as _;
use rand_distr::{Distribution, Normal};

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
    pub total: f32,
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
    pub momentum: glam::Vec2,
    pub energy: f32,
    pub dead: bool,
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
            momentum: glam::Vec2::new(100.0, 0.0),
            energy: 0.0,
            dead: false,
        }
    }
}

pub fn random_normal_vec2() -> glam::Vec2 {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 1.0).unwrap();
    let normal_vec2 = glam::Vec2::new(normal.sample(&mut rng), normal.sample(&mut rng));
    normal_vec2.normalize_or_zero()
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
            total: 0.0,
        };
        for segment in self.segments.iter() {
            segment_lengths.total += segment.length();
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

    pub fn maybe_move(&mut self, delta_time: f32) {
        let segment_lengths = self.segment_lengths();
        let movement_chance = segment_lengths.move_ / segment_lengths.total;
        let movement_force = movement_chance * 100.0;
        if rand::thread_rng().gen::<f32>() < movement_chance * 10.0 * delta_time {
            if rand::thread_rng().gen::<bool>() {
                self.momentum += random_normal_vec2() * movement_force;
            } else {
                let mut r = rand::thread_rng().gen::<f32>() * segment_lengths.move_;
                for segment in self.segments.iter() {
                    if segment.length() < r {
                        r -= segment.length();
                    } else {
                        assert!(segment.length() >= r);
                        let move_direction =
                            (segment.b - segment.a) * (r / segment.length()) + segment.a;
                        if rand::thread_rng().gen::<bool>() {
                            self.momentum += move_direction.normalize() * movement_force;
                        } else {
                            self.momentum += -move_direction.normalize() * movement_force;
                        }
                        break;
                    }
                }
            }
        }
    }

    pub fn check_wall_collision(&mut self, delta_time: f32, world_size: glam::Vec2) {
        let mut bounced = (false, false);
        for segment in self.segments.iter() {
            for endpoint in [segment.a, segment.b] {
                if !bounced.0 && (endpoint.x < -world_size.x || world_size.x < endpoint.x) {
                    self.dead = true;
                }
                if !bounced.1 && (endpoint.y < -world_size.y || world_size.y < endpoint.y) {
                    self.dead = true;
                }
                let next_endpoint = self.position + endpoint + self.momentum * delta_time;
                if !bounced.0 && (next_endpoint.x < -world_size.x || world_size.x < next_endpoint.x)
                {
                    bounced.0 = true;
                    self.momentum = glam::Vec2::new(-self.momentum.x, self.momentum.y);
                }
                if !bounced.1 && (next_endpoint.y < -world_size.y || world_size.y < next_endpoint.y)
                {
                    bounced.1 = true;
                    self.momentum = glam::Vec2::new(self.momentum.x, -self.momentum.y);
                }
            }
        }
        if bounced.0 || bounced.1 {
            self.momentum *= 0.5;
        }
    }

    pub fn update(&mut self, delta_time: f32, world_size: glam::Vec2) {
        self.maybe_move(delta_time);
        self.check_wall_collision(delta_time, world_size);
        self.position += self.momentum * delta_time;
        // We use the continuous time exponential growth function: P = P0 e^(kt)
        self.momentum *= f32::exp(f32::ln(0.5) * delta_time);
    }
}
