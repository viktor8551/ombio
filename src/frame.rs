/// An opaque color with red, green and blue sRGB components from 0 to 255.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub(crate) fn linear(self) -> wgpu::Color {
        fn component(value: u8) -> f64 {
            let value = f64::from(value) / 255.0;
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        }
        wgpu::Color {
            r: component(self.red),
            g: component(self.green),
            b: component(self.blue),
            a: 1.0,
        }
    }
}

/// One complete picture. Each redraw starts with an opaque black background.
pub struct Frame {
    pub(crate) background: Color,
    pub(crate) rectangles: Vec<Rectangle>,
}

pub(crate) struct Rectangle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) color: Color,
}

impl Frame {
    pub(crate) fn new() -> Self {
        Self {
            background: Color::rgb(0, 0, 0),
            rectangles: Vec::new(),
        }
    }

    /// Fills the picture with an opaque color, discarding earlier drawing in this frame.
    pub fn clear(&mut self, color: Color) {
        self.background = color;
        self.rectangles.clear();
    }

    /// Draws an opaque filled rectangle, covering earlier drawing where it overlaps.
    ///
    /// Position and dimensions are `f32` logical pixels from the content area's
    /// top-left: x increases rightward and y downward. Windows DPI scaling is
    /// applied by the renderer, and drawing clips at the content boundary.
    /// Color uses opaque 8-bit sRGB components, just like [`Frame::clear`].
    /// Non-finite arguments and non-positive dimensions draw nothing.
    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        if ![x, y, width, height].into_iter().all(f32::is_finite) || width <= 0.0 || height <= 0.0 {
            return;
        }
        self.rectangles.push(Rectangle {
            x,
            y,
            width,
            height,
            color,
        });
    }
}
