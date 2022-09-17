use std::time::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Timer {
    pub instant: Instant,
    pub current_frame: f32,
}

impl Timer {
    pub fn new() -> Self {
        return Self {
            instant: Instant::now(),
            current_frame: 0.0f32,
        };
    }

    pub fn dt(&mut self) -> f32 {
        let current_frame = self.time();
        let dt = current_frame - self.current_frame;
        self.current_frame = current_frame;
        return dt;
    }

    pub fn current_frame(&self) -> f32 {
        return self.current_frame;
    }

    pub fn time(&self) -> f32 {
        return self.instant.elapsed().as_secs_f32();
    }
}
