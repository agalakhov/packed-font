use embedded_graphics_core::{Pixel, draw_target::DrawTarget, pixelcolor::RgbColor};

use super::{UnpackStyle, blend::Blend};
use packed_font_structs::{AaColor, Metrics};

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
    fn draw_iter<D: DrawTarget<Color = C>>(
        &self,
        _metrics: &Metrics,
        target: &mut D,
        pixels: impl Iterator<Item = Pixel<AaColor>>,
    ) -> Result<(), D::Error> {
        let pixels = pixels.map(|Pixel(pt, grade)| {
            let color = self.foreground.blend(&self.background, grade);
            Pixel(pt, color)
        });
        target.draw_iter(pixels)
    }
}
