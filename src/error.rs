use std::{fmt, io};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A size was requested with a width of zero.
    ZeroWidth,
    /// A size was requested with a height of zero.
    ZeroHeight,
    /// An underlying I/O operation failed.
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroWidth => f.write_str("size width must be non-zero"),
            Self::ZeroHeight => f.write_str("size height must be non-zero"),
            Self::Io(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::ZeroWidth | Self::ZeroHeight => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
