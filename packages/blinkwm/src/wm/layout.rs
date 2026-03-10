#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Layout {
    Vertical,   // Master on left, stack on right
    #[allow(dead_code)]
    Horizontal, // Master on top, stack on bottom
    #[allow(dead_code)]
    Stacking,   // All windows fullscreen
}

impl Default for Layout {
    fn default() -> Self {
        Self::Vertical
    }
}
