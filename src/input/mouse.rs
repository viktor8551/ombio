use std::collections::HashSet;

use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseScrollDelta, WindowEvent},
};

use super::MouseButton;

pub struct Mouse {
    down: HashSet<MouseButton>,
    pressed: HashSet<MouseButton>,
    released: HashSet<MouseButton>,
    position: Option<PhysicalPosition<f64>>,
    baseline: Option<PhysicalPosition<f64>>,
    inside: bool,
    scale_factor: f64,
    movement: (f64, f64),
    scroll_lines: (f64, f64),
    scroll_pixels: (f64, f64),
}

impl Mouse {
    pub(crate) fn new() -> Self {
        Self {
            down: HashSet::new(),
            pressed: HashSet::new(),
            released: HashSet::new(),
            position: None,
            baseline: None,
            inside: false,
            scale_factor: 1.0,
            movement: (0.0, 0.0),
            scroll_lines: (0.0, 0.0),
            scroll_pixels: (0.0, 0.0),
        }
    }

    pub fn is_down(&self, button: MouseButton) -> bool {
        self.down.contains(&button)
    }

    pub fn just_pressed(&self, button: MouseButton) -> bool {
        self.pressed.contains(&button)
    }

    pub fn just_released(&self, button: MouseButton) -> bool {
        self.released.contains(&button)
    }

    /// Last observed (x, y) in logical pixels from the content area's top-left.
    pub fn position(&self) -> Option<(f64, f64)> {
        self.position.map(|position| {
            (
                position.x / self.scale_factor,
                position.y / self.scale_factor,
            )
        })
    }

    pub fn is_inside_window(&self) -> bool {
        self.inside
    }

    /// Accumulated (x, y) movement in logical pixels for this update.
    pub fn movement(&self) -> (f64, f64) {
        self.movement
    }

    /// Accumulated horizontal and vertical scrolling in lines for this update.
    pub fn scroll_lines(&self) -> (f64, f64) {
        self.scroll_lines
    }

    /// Accumulated horizontal and vertical scrolling in logical pixels for this update.
    pub fn scroll_pixels(&self) -> (f64, f64) {
        self.scroll_pixels
    }

    pub(super) fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseInput { button, state, .. } => match state {
                ElementState::Pressed => {
                    if self.down.insert(*button) {
                        self.pressed.insert(*button);
                    }
                }
                ElementState::Released => {
                    if self.down.remove(button) {
                        self.released.insert(*button);
                    }
                }
            },
            WindowEvent::CursorEntered { .. } => {
                self.inside = true;
                self.baseline = None;
            }
            WindowEvent::CursorLeft { .. } => {
                self.inside = false;
                self.baseline = None;
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(previous) = self.baseline {
                    self.movement.0 += (position.x - previous.x) / self.scale_factor;
                    self.movement.1 += (position.y - previous.y) / self.scale_factor;
                }
                self.position = Some(*position);
                self.baseline = Some(*position);
                self.inside = true;
            }
            WindowEvent::MouseWheel { delta, .. } => match delta {
                MouseScrollDelta::LineDelta(x, y) => {
                    self.scroll_lines.0 += f64::from(*x);
                    self.scroll_lines.1 += f64::from(*y);
                }
                MouseScrollDelta::PixelDelta(position) => {
                    self.scroll_pixels.0 += position.x / self.scale_factor;
                    self.scroll_pixels.1 += position.y / self.scale_factor;
                }
            },
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.set_scale_factor(*scale_factor);
            }
            _ => {}
        }
    }

    pub(super) fn set_scale_factor(&mut self, scale_factor: f64) {
        if self.scale_factor != scale_factor {
            self.baseline = None;
        }
        self.scale_factor = scale_factor;
    }

    pub(super) fn release_all(&mut self) {
        self.released.extend(self.down.drain());
    }

    pub(super) fn finish_update(&mut self) {
        self.pressed.clear();
        self.released.clear();
        self.movement = (0.0, 0.0);
        self.scroll_lines = (0.0, 0.0);
        self.scroll_pixels = (0.0, 0.0);
    }
}
