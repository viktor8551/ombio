use std::{path::Path, sync::Arc};

use crate::{Context, Error};

use super::renderer::GpuImage;

/// A reusable PNG image shared by sprites.
#[derive(Clone)]
pub struct Image {
    pub(crate) data: Arc<ImageData>,
}

pub(crate) struct ImageData {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) gpu: GpuImage,
}

impl Image {
    /// Decodes a PNG synchronously and queues its upload to the context's GPU.
    ///
    /// Clones share the GPU image. Decoded CPU pixels are released after loading.
    /// The image can only be drawn by the context that loaded it.
    pub fn load(ctx: &Context, path: impl AsRef<Path>) -> Result<Self, Error> {
        let image = image::ImageReader::with_format(
            std::io::BufReader::new(
                std::fs::File::open(path).map_err(|error| Error::SpriteLoading(error.into()))?,
            ),
            image::ImageFormat::Png,
        )
        .decode()
        .map_err(Error::SpriteLoading)?
        .into_rgba8();
        let gpu = ctx
            .graphics()
            .upload_image(image.width(), image.height(), image.as_raw())?;
        Ok(Self {
            data: Arc::new(ImageData {
                width: image.width(),
                height: image.height(),
                gpu,
            }),
        })
    }
}
