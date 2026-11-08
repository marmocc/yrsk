use std::time::{Duration, Instant};

pub fn measure<T>(function: impl FnOnce() -> T) -> (T, Duration) {
    let origin = Instant::now();
    let result = function();
    (result, origin.elapsed())
}
