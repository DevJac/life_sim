use std::time::{Duration, Instant};

pub struct FPSStats {
    last_render_time: Instant,
    last_log_time: Instant,
    /// Log frequency in seconds
    log_frequency: f32,
    /// The half life (in seconds) of samples
    half_life: f32,
    mean: f32,
    variance: f32,
    percentile_99: f32,
}

impl FPSStats {
    pub fn new(half_life: f32, log_frequency: f32) -> Self {
        Self {
            last_render_time: Instant::now(),
            last_log_time: Instant::now(),
            log_frequency,
            half_life,
            mean: 1.0 / 60.0,
            variance: 0.0,
            percentile_99: 1.0 / 60.0,
        }
    }

    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        let frame_time = (now - self.last_render_time).as_secs_f32();
        self.last_render_time = now;
        let alpha: f32 = 2.0_f32.powf(-frame_time / self.half_life);
        self.mean = alpha * self.mean + (1.0 - alpha) * frame_time;
        self.variance = alpha * self.variance + (1.0 - alpha) * (self.mean - frame_time).powi(2);
        let percentile_step = self.std() / 100.0;
        if frame_time < self.percentile_99 {
            self.percentile_99 -= percentile_step / 0.99;
        }
        if frame_time > self.percentile_99 {
            self.percentile_99 += percentile_step / (1.0 - 0.99);
        }
        if (now - self.last_log_time) > Duration::from_secs_f32(self.log_frequency) {
            self.last_log_time = now;
            return true;
        }
        false
    }

    pub fn mean(&self) -> f32 {
        self.mean
    }

    pub fn variance(&self) -> f32 {
        self.variance
    }

    /// Standard deviation
    pub fn std(&self) -> f32 {
        self.variance.sqrt()
    }

    pub fn percentile_99(&self) -> f32 {
        self.percentile_99
    }
}
