use std::{io::Result, time::Duration};

use crate::{
    diagnostics::measure::Measure,
    rendering::{canvas::Canvas, painter::Painter},
};

pub trait Renderer: Canvas + Sized {
    fn present(&mut self) -> Result<()>;

    fn run(&mut self, painter: &mut impl Painter) -> Result<()> {
        loop {
            painter.paint(self);
            self.present()?;
        }
    }

    fn benchmark(
        &mut self,
        painter: &mut impl Painter,
        iterations: u32,
    ) -> Result<(Duration, Duration)> {
        let mut paint_sum: Duration = Duration::ZERO;
        let mut present_sum: Duration = Duration::ZERO;

        for _ in 0..iterations {
            Measure::accumulate(&mut paint_sum, || painter.paint(self));
            Measure::accumulate(&mut present_sum, || self.present())?;
        }

        Ok((paint_sum / iterations, present_sum / iterations))
    }
}
