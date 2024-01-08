use std::time::{Instant, Duration};

pub struct FrameTimer {
    previous: Instant,
    fps: f32,
}

impl FrameTimer {
    pub fn new() -> Self {
        Self {
            previous: Instant::now() ,
            fps: 60.0,
        }
    }

    pub fn frame(&mut self) -> Duration {
        let dur = self.previous.elapsed();
        self.previous = Instant::now();
        self.fps -= self.fps / 10.0;
        self.fps += (1.0/dur.as_secs_f32()) / 10.0;
        dur
    }

    pub fn fps(&self) -> f32 {
        self.fps
    }
}