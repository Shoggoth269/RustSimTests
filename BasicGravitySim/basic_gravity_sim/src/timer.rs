pub struct Timer {
    pub now: std::time::SystemTime,
}

pub trait Timing {
    fn mark(&mut self) -> f64;
}

impl Timing for Timer {
    fn mark(&mut self) -> f64 {
        let last = self.now;
        self.now = std::time::SystemTime::now();
        let duration = self.now.duration_since(last).unwrap();
        duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
    }
}