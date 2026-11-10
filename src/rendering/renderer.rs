use std::{
    io,
    time::{Duration, Instant},
};

use crate::{diagnostics::measure::Measure, painters::chaos, rendering::canvas::Canvas};

pub trait Renderer: Canvas + Sized {
    fn present(&mut self) -> io::Result<()>;

    fn benchmark(&mut self, iterations: u32) -> io::Result<(Duration, Duration)> {
        let mut paint_sum: Duration = Duration::ZERO;
        let mut present_sum: Duration = Duration::ZERO;

        let origin = Instant::now();
        for _ in 0..iterations {
            Measure::accumulate(&mut paint_sum, || chaos(origin, self));
            Measure::accumulate(&mut present_sum, || self.present())?;
        }

        Ok((paint_sum / iterations, present_sum / iterations))
    }
}
