use embedded_graphics::{geometry::Point, draw_target::DrawTarget, text::{Baseline, renderer::{TextRenderer, TextMetrics}}};

use super::{PackedFont, UnpackStyle};

pub struct CharacterStyle<'t, S> {
    pub font: &'t PackedFont,
    pub style: S,
}

impl<'t, S: UnpackStyle> CharacterStyle<'t, S> {
    pub fn new(font: &'t PackedFont, style: S) -> Self {
        Self { font, style }
    }
}

impl<S> TextRenderer for CharacterStyle<'_, S>
where
    S: UnpackStyle,
{
    type Color = S::Color;

    fn draw_string<D>(
        &self,
        text: &str,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>
    {
        let mut x = position.x;
        let y = position.y;

        for chr in text.chars() {
            let origin = Point::new(x, y);
            if let Some(metrics) = self.font.render(chr, &self.style, origin, target)? {
                x += metrics.advance as i32;
            }
        }

        Ok(Point::new(x, y))
    }

    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
       where D: DrawTarget<Color = Self::Color>
    {
        todo!()
    }

    fn measure_string(
        &self,
        text: &str,
        position: Point,
        baseline: Baseline,
    ) -> TextMetrics
    {
        todo!()
    }

    fn line_height(&self) -> u32 {
        self.font.metrics.line_height as u32
    }
}
