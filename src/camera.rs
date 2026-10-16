/// A requested rectangle in logical window pixels, independent of window resizing.
///
/// The Camera maps to this rectangle's center, including when it extends outside
/// the window. Drawing is clipped to the intersection with the window.
/// Non-finite coordinates or dimensions, and dimensions <= 0, draw nothing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Clip {
    pub(crate) left: f64,
    pub(crate) top: f64,
    pub(crate) right: f64,
    pub(crate) bottom: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Projection {
    pub(crate) origin: (f64, f64),
    pub(crate) offset: (f64, f64),
    pub(crate) scale: f64,
    pub(crate) clip: Clip,
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
            clip: Clip {
                left: 0.0,
                top: 0.0,
                right: f64::from(size.0),
                bottom: f64::from(size.1),
            },
        })
    }

    pub(crate) fn in_viewport(mut self, viewport: Viewport, dpi_scale: f64) -> Option<Self> {
        if ![viewport.x, viewport.y, viewport.width, viewport.height]
            .into_iter()
            .all(f32::is_finite)
            || viewport.width <= 0.0
            || viewport.height <= 0.0
        {
            return None;
        }
        let left = f64::from(viewport.x) * dpi_scale;
        let top = f64::from(viewport.y) * dpi_scale;
        let width = f64::from(viewport.width) * dpi_scale;
        let height = f64::from(viewport.height) * dpi_scale;
        let right = left + width;
        let bottom = top + height;
        let center = (left + width / 2.0, top + height / 2.0);
        if ![left, top, right, bottom, center.0, center.1]
            .into_iter()
            .all(f64::is_finite)
        {
            return None;
        }
        self.clip = Clip {
            left: left.max(self.clip.left),
            top: top.max(self.clip.top),
            right: right.min(self.clip.right),
            bottom: bottom.min(self.clip.bottom),
        };
        if self.clip.left >= self.clip.right || self.clip.top >= self.clip.bottom {
            return None;
        }
        self.offset = center;
        Some(self)
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
