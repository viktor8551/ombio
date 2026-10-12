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
}

impl Frame {
    pub(crate) fn new() -> Self {
        Self {
            background: Color::rgb(0, 0, 0),
        }
    }

    /// Fills the picture with an opaque color. The last clear in a frame wins.
    pub fn clear(&mut self, color: Color) {
        self.background = color;
    }
}
