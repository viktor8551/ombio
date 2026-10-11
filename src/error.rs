use std::fmt;

use winit::error::{EventLoopError, OsError};

/// A failure while preparing or running the engine.
#[derive(Debug)]
pub enum Error {
    Preparation(EventLoopError),
    WindowCreation(OsError),
    Runtime(EventLoopError),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preparation(error) => write!(formatter, "engine preparation failed: {error}"),
            Self::WindowCreation(error) => write!(formatter, "window creation failed: {error}"),
            Self::Runtime(error) => write!(formatter, "engine runtime failed: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Preparation(error) | Self::Runtime(error) => Some(error),
            Self::WindowCreation(error) => Some(error),
        }
    }
}
