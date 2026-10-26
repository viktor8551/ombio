use std::{fmt, io, path::PathBuf};

use winit::error::{EventLoopError, OsError};

/// A failure while loading assets, preparing, or running the engine.
#[derive(Debug)]
pub enum Error {
    AssetDirectory(PathBuf, io::Error),
    AssetOpen(PathBuf, io::Error),
    SpriteLoading(image::ImageError),
    Preparation(EventLoopError),
    WindowCreation(OsError),
    Runtime(EventLoopError),
    SurfaceCreation(wgpu::CreateSurfaceError),
    AdapterRequest(wgpu::RequestAdapterError),
    DeviceRequest(wgpu::RequestDeviceError),
    GpuInitialization(String),
    Rendering(String),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AssetDirectory(path, error) => write!(
                formatter,
                "asset directory resolution failed for '{}': {error}",
                path.display()
            ),
            Self::AssetOpen(path, error) => write!(
                formatter,
                "asset opening failed for '{}': {error}",
                path.display()
            ),
            Self::SpriteLoading(error) => write!(formatter, "sprite loading failed: {error}"),
            Self::Preparation(error) => write!(formatter, "engine preparation failed: {error}"),
            Self::WindowCreation(error) => write!(formatter, "window creation failed: {error}"),
            Self::Runtime(error) => write!(formatter, "engine runtime failed: {error}"),
            Self::SurfaceCreation(error) => {
                write!(formatter, "GPU surface creation failed: {error}")
            }
            Self::AdapterRequest(error) => write!(formatter, "GPU adapter request failed: {error}"),
            Self::DeviceRequest(error) => write!(formatter, "GPU device request failed: {error}"),
            Self::GpuInitialization(error) => {
                write!(formatter, "GPU initialization failed: {error}")
            }
            Self::Rendering(error) => write!(formatter, "GPU rendering failed: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::AssetDirectory(_, error) | Self::AssetOpen(_, error) => Some(error),
            Self::SpriteLoading(error) => Some(error),
            Self::Preparation(error) | Self::Runtime(error) => Some(error),
            Self::WindowCreation(error) => Some(error),
            Self::SurfaceCreation(error) => Some(error),
            Self::AdapterRequest(error) => Some(error),
            Self::DeviceRequest(error) => Some(error),
            Self::GpuInitialization(_) | Self::Rendering(_) => None,
        }
    }
}
