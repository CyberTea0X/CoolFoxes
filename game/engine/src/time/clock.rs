use std::time::{Duration, Instant};

pub struct Clock {
    instant: Instant,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {instant: Instant::now()}
    }
    pub fn get_time(&mut self) -> Duration {
        let dt = self.instant.elapsed();
        self.instant = Instant::now();
        return dt;
    }
}