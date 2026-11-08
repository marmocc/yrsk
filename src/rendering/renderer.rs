use crate::{
    diagnostics::{measure::Measure, timings::Timings},
    rendering::{canvas::Canvas, painter::Painter},
};
use std::{io::Result, thread::sleep, time::Duration};

pub trait Renderer: Canvas + Sized {
    fn present(&mut self) -> Result<()>;

    fn run(&mut self, painter: &mut impl Painter, fps: u32) -> Result<()> {
        let wait = Duration::from_secs_f32(1.0 / (fps as f32));
        loop {
            painter.paint(self);
            self.present()?;
            sleep(wait);
        }
    }

    fn benchmark(&mut self, painter: &mut impl Painter, iterations: u32) -> Result<Timings> {
        let mut paint_sum: Duration = Duration::ZERO;
        let mut present_sum: Duration = Duration::ZERO;

        for _ in 0..iterations {
            Measure::accumulate(&mut paint_sum, || painter.paint(self));
            Measure::accumulate(&mut present_sum, || self.present())?;
        }

        Ok(Timings::new(
            paint_sum / iterations,
            Duration::ZERO / iterations,
            present_sum / iterations,
        ))
    }
}
