use std::time::SystemTime;

pub struct Timer {
    start: SystemTime,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: SystemTime::now(),
        }
    }

    pub fn get_time(&self) -> f64 {
        let elapsed = self.start.elapsed().expect("Time elapsed failed");
        let elapsed_secs = elapsed.as_secs() as f64;
        let elapsed_micros = elapsed.subsec_micros() as f64;
        elapsed_secs + elapsed_micros / 1_000_000.0
    }
}

pub const TL: f64 = 2.8;
