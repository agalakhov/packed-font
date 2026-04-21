use embedded_graphics_core::{Pixel, draw_target::DrawTarget, pixelcolor::RgbColor};

use super::{UnpackTarget, blend::Blend};
use packed_font_structs::{AaColor, Metrics};

pub struct TwoColor<D: DrawTarget> {
    target: D,
    foreground: D::Color,
    background: D::Color,
}

impl<D: DrawTarget> UnpackTarget for TwoColor<D>
where
    D::Color: RgbColor + Blend<D::Color, Target = D::Color>,
{
    type Error = D::Error;
    fn draw_iter(
        &mut self,
        _metrics: &Metrics,
        pixels: impl Iterator<Item = Pixel<AaColor>>,
    ) -> Result<(), Self::Error> {
        let pixels = pixels.map(|Pixel(pt, grade)| {
            let color = self.foreground.blend(&self.background, grade);
            Pixel(pt, color)
        });
        self.target.draw_iter(pixels)
    }
}
