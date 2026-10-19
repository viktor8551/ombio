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
    pub(crate) fn window(size: (f64, f64)) -> Option<Self> {
        if ![size.0, size.1].into_iter().all(f64::is_finite) || size.0 <= 0.0 || size.1 <= 0.0 {
            return None;
        }
        Some(Self {
            origin: (0.0, 0.0),
            offset: (0.0, 0.0),
            scale: 1.0,
            clip: Clip {
                left: 0.0,
                top: 0.0,
                right: size.0,
                bottom: size.1,
            },
        })
    }

    pub(crate) fn in_viewport(mut self, viewport: Viewport) -> Option<Self> {
        if ![viewport.x, viewport.y, viewport.width, viewport.height]
            .into_iter()
            .all(f32::is_finite)
            || viewport.width <= 0.0
            || viewport.height <= 0.0
        {
            return None;
        }
        let left = f64::from(viewport.x);
        let top = f64::from(viewport.y);
        let width = f64::from(viewport.width);
        let height = f64::from(viewport.height);
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

    fn unproject(self, position: (f64, f64)) -> Option<(f64, f64)> {
        let (x, y) = position;
        if !x.is_finite()
            || !y.is_finite()
            || x < self.clip.left
            || x >= self.clip.right
            || y < self.clip.top
            || y >= self.clip.bottom
        {
            return None;
        }
        let world = (
            (x - self.offset.0) / self.scale + self.origin.0,
            (y - self.offset.1) / self.scale + self.origin.1,
        );
        (world.0.is_finite() && world.1.is_finite()).then_some(world)
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

    /// Logical window pixels to world coordinates; `None` viewport uses the full window.
    /// Includes left/top edges, excludes right/bottom. Invalid inputs, outside points,
    /// and non-finite results return `None`.
    pub fn window_to_world(
        &self,
        position: (f64, f64),
        window_size: (f64, f64),
        viewport: Option<Viewport>,
    ) -> Option<(f64, f64)> {
        let projection = self.projection(window_size)?;
        let projection = match viewport {
            Some(viewport) => projection.in_viewport(viewport)?,
            None => projection,
        };
        projection.unproject(position)
    }

    pub(crate) fn projection(self, view_size: (f64, f64)) -> Option<Projection> {
        if !self.x.is_finite() || !self.y.is_finite() || !self.zoom.is_finite() || self.zoom <= 0.0
        {
            return None;
        }
        let mut projection = Projection::window(view_size)?;
        projection.origin = (f64::from(self.x), f64::from(self.y));
        projection.offset = (view_size.0 / 2.0, view_size.1 / 2.0);
        projection.scale = f64::from(self.zoom);
        Some(projection)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}
