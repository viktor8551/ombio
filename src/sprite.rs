use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{Error, renderer::GpuSprite};

/// A PNG image that can be retained and drawn across frames.
/// Clones share the decoded image and its lazily uploaded GPU resources.
#[derive(Clone)]
pub struct Sprite {
    pub(crate) data: Arc<SpriteData>,
}

pub(crate) struct SpriteData {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixels: Vec<u8>,
    pub(crate) gpu: Mutex<Option<GpuSprite>>,
}

impl Sprite {
    /// Synchronously loads a PNG without requiring an engine, window, or GPU.
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
            data: Arc::new(SpriteData {
                width: image.width(),
                height: image.height(),
                pixels: image.into_raw(),
                gpu: Mutex::new(None),
            }),
        })
    }
}
