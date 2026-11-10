use crate::{common::size::Size, rendering::command::Command};
use std::time::Duration;

pub trait Canvas {
    fn size(&self) -> Size;
    fn elapsed(&self) -> Duration;
    fn execute(&mut self, command: Command);
}
