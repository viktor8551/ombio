use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use crate::Error;

use super::renderer::GpuImage;

/// A reusable PNG image shared by sprites.
#[derive(Clone)]
pub struct Image {
    pub(crate) data: Arc<ImageData>,
}

pub(crate) struct ImageData {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixels: Vec<u8>,
    pub(crate) gpu: Mutex<Option<GpuImage>>,
}

impl Image {
    /// Loads a PNG synchronously without an engine.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let image = image::ImageReader::with_format(
            std::io::BufReader::new(
                std::fs::File::open(path).map_err(|error| Error::SpriteLoading(error.into()))?,
            ),
            image::ImageFormat::Png,
        )
        .decode()
        .map_err(Error::SpriteLoading)?
        .into_rgba8();
        Ok(Self {
            data: Arc::new(ImageData {
                width: image.width(),
                height: image.height(),
                pixels: image.into_raw(),
                gpu: Mutex::new(None),
            }),
        })
    }
}
