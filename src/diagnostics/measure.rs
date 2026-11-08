use std::time::{Duration, Instant};

pub struct Measure;
impl Measure {
    pub fn into<T>(elapsed: &mut Duration, function: impl FnOnce() -> T) -> T {
        let origin = Instant::now();
        let result = function();
        *elapsed = origin.elapsed();
        result
    }

    pub fn accumulate<T>(elapsed: &mut Duration, function: impl FnOnce() -> T) -> T {
        let origin = Instant::now();
        let result = function();
        *elapsed += origin.elapsed();
        result
    }
}
