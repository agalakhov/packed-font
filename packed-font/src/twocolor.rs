use embedded_graphics_core::pixelcolor::RgbColor;

use super::{UnpackStyle, blend::Blend};
use packed_font_structs::AaColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwoColor<C> {
    pub foreground: C,
    pub background: C,
}

impl<C> UnpackStyle for TwoColor<C>
where
    C: RgbColor + Blend<C, Target = C>,
{
    type Color = C;
    fn map_color(&self, grade: AaColor) -> Self::Color {
        self.foreground.blend(&self.background, grade)
    }
    fn background_color(&self) -> Option<Self::Color> {
        Some(self.background)
    }
}
