use crate::common::Size;
use crate::rendering::ansi::Ansi;

#[cfg(windows)]
pub type Terminal = Ansi<std::fs::File>;
#[cfg(windows)]
impl Terminal {
    pub fn with_size(size: Size) -> Self {
        use std::os::windows::io::AsHandle;
        let handle = std::io::stdout()
            .as_handle()
            .try_clone_to_owned()
            .expect("could not duplicate stdout handle");
        Self::with_output(size, std::fs::File::from(handle))
    }
}

#[cfg(not(windows))]
pub type Terminal = Ansi<std::io::Stdout>;
#[cfg(not(windows))]
impl Terminal {
    pub fn with_size(size: Size) -> Self {
        Self::with_output(size, std::io::stdout())
    }
}
