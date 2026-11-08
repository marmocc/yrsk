use std::time::Duration;

pub struct Timings {
    pub paint: Duration,
    pub play: Duration,
    pub present: Duration,
}

impl Timings {
    pub fn new(paint: Duration, play: Duration, present: Duration) -> Self {
        Self {
            paint,
            play,
            present,
        }
    }
}
