use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    primitives::Rectangle,
    text::{
        Baseline,
        renderer::{TextMetrics, TextRenderer},
    },
};

use super::{PackedFont, UnpackStyle};

pub struct CharacterStyle<'t, S> {
    pub font: &'t PackedFont,
    pub style: S,
}

impl<'t, S: UnpackStyle> CharacterStyle<'t, S> {
    pub fn new(font: &'t PackedFont, style: S) -> Self {
        Self { font, style }
    }

    fn apply_baseline(&self, position: Point, baseline: Baseline) -> Point {
        let Point { x, mut y } = position;
        let metrics = &self.font.metrics;
        y += match baseline {
            Baseline::Top => metrics.ascent as i32,
            Baseline::Bottom => metrics.descent as i32,
            Baseline::Middle => (self.line_height() / 2) as i32,
            Baseline::Alphabetic => 0,
        };
        Point::new(x, y)
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
        D: DrawTarget<Color = Self::Color>,
    {
        let pos = self.apply_baseline(position, baseline);
        let mut x = pos.x;
        let y = pos.y;

        for chr in text.chars() {
            let origin = Point::new(x, y);
            if let Some(metrics) = self.font.render(chr, &self.style, origin, target)? {
                x += metrics.advance as i32;
            }
        }

        Ok(Point::new(x, position.y))
    }

    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let position = self.apply_baseline(position, baseline);
        let height = self.line_height();
        if let Some(color) = self.style.background_color() {
            target.fill_solid(&Rectangle::new(position, Size::new(width, height)), color)?;
        }
        Ok(Point::new(position.x + width as i32, position.y))
    }

    fn measure_string(&self, text: &str, position: Point, baseline: Baseline) -> TextMetrics {
        let position = self.apply_baseline(position, baseline);
        todo!()
    }

    fn line_height(&self) -> u32 {
        (self.font.metrics.ascent as i32 - self.font.metrics.descent as i32
            + self.font.metrics.leading as i32) as u32
    }
}
