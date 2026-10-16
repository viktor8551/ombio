#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Projection {
    pub(crate) origin: (f64, f64),
    pub(crate) offset: (f64, f64),
    pub(crate) scale: f64,
}

impl Projection {
    pub(crate) fn window(size: (u32, u32), scale: f64) -> Option<Self> {
        if size.0 == 0 || size.1 == 0 || !scale.is_finite() || scale <= 0.0 {
            return None;
        }
        Some(Self {
            origin: (0.0, 0.0),
            offset: (0.0, 0.0),
            scale,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Camera {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y, zoom: 1.0 }
    }

    pub(crate) fn projection(self, view_size: (u32, u32), dpi_scale: f64) -> Option<Projection> {
        if !self.x.is_finite() || !self.y.is_finite() || !self.zoom.is_finite() || self.zoom <= 0.0
        {
            return None;
        }
        let mut projection = Projection::window(view_size, dpi_scale)?;
        projection.origin = (f64::from(self.x), f64::from(self.y));
        projection.offset = (f64::from(view_size.0) / 2.0, f64::from(view_size.1) / 2.0);
        projection.scale *= f64::from(self.zoom);
        if !projection.scale.is_finite() || projection.scale <= 0.0 {
            return None;
        }
        Some(projection)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}
