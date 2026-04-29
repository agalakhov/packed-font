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

impl<'t, S> CharacterStyle<'t, S> {
    pub fn new(font: &'t PackedFont, style: S) -> Self {
        Self { font, style }
    }

    pub fn apply_baseline(&self, position: Point, baseline: Baseline) -> Point {
        let Point { x, mut y } = position;
        let metrics = &self.font.metrics;
        y += match baseline {
            Baseline::Top => metrics.ascent as i32,
            Baseline::Bottom => metrics.descent as i32,
            Baseline::Middle => (metrics.ascent as i32 + metrics.descent as i32) / 2,
            Baseline::Alphabetic => 0,
        };
        Point::new(x, y)
    }
}

impl<'t, S: UnpackStyle> CharacterStyle<'t, S> {
    pub fn draw_character<D>(
        &self,
        chr: char,
        origin: Point,
        target: &mut D,
    ) -> Result<TextMetrics, D::Error>
    where
        D: DrawTarget<Color = S::Color>,
    {
        if let Some((metrics, height)) = self.font.render(chr, &self.style, origin, target)? {
            let top_left = Point::new(origin.x, origin.y - self.font.metrics.ascent as i32);
            let full_height = (self.font.metrics.ascent - self.font.metrics.descent) as u32;
            if let Some(color) = self.style.background_color() {
                if let Ok(left_bearing) = metrics.left_bearing.try_into() {
                    target.fill_solid(
                        &Rectangle::new(top_left, Size::new(left_bearing, full_height)),
                        color,
                    )?;
                }
                if self.font.metrics.ascent > metrics.top_bearing {
                    target.fill_solid(
                        &Rectangle::new(
                            Point::new(top_left.x + metrics.left_bearing as i32, top_left.y),
                            Size::new(
                                metrics.width as u32,
                                (self.font.metrics.ascent - metrics.top_bearing) as u32,
                            ),
                        ),
                        color,
                    )?;
                }
                let bottom_y_offset = height as i32 - metrics.top_bearing as i32;
                let bottom_rest = -bottom_y_offset - self.font.metrics.descent as i32;
                if bottom_rest > 0 {
                    target.fill_solid(
                        &Rectangle::new(
                            Point::new(
                                top_left.x + metrics.left_bearing as i32,
                                origin.y + bottom_y_offset,
                            ),
                            Size::new(metrics.width as u32, bottom_rest as u32),
                        ),
                        color,
                    )?;
                }
                let right_rest =
                    metrics.advance as i32 - metrics.left_bearing as i32 - metrics.width as i32;
                if right_rest > 0 {
                    target.fill_solid(
                        &Rectangle::new(
                            Point::new(
                                top_left.x + metrics.left_bearing as i32 + metrics.width as i32,
                                top_left.y,
                            ),
                            Size::new(right_rest as u32, full_height),
                        ),
                        color,
                    )?;
                }
            }

            let next_position = Point::new(origin.x + metrics.advance as i32, origin.y);
            let size = Size::new(metrics.advance as u32, full_height);
            let bounding_box = Rectangle::new(top_left, size);
            Ok(TextMetrics {
                bounding_box,
                next_position,
            })
        } else {
            let bounding_box = Rectangle::new(origin, Size::new(0, 0));
            Ok(TextMetrics {
                bounding_box,
                next_position: origin,
            })
        }
    }

    pub fn measure_character(&self, chr: char) -> Size {
        let full_height = (self.font.metrics.ascent - self.font.metrics.descent) as u32;
        let Some((metrics, _)) = self.font.get_metrics_and_data(chr) else {
            return Size::zero();
        };
        let width = metrics.advance.max(0) as u32;
        Size::new(width, full_height)
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
        let mut pos = self.apply_baseline(position, baseline);

        for chr in text.chars() {
            pos = self.draw_character(chr, pos, target)?.next_position;
        }

        Ok(Point::new(pos.x, position.y))
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
        let pos = self.apply_baseline(position, baseline);
        let height = self.line_height();
        if let Some(color) = self.style.background_color() {
            target.fill_solid(&Rectangle::new(pos, Size::new(width, height)), color)?;
        }
        Ok(Point::new(pos.x + width as i32, pos.y))
    }

    fn measure_string(&self, text: &str, position: Point, baseline: Baseline) -> TextMetrics {
        let pos = self.apply_baseline(position, baseline);
        let full_height = (self.font.metrics.ascent - self.font.metrics.descent) as u32;
        let mut total_width = 0;

        let top_left = Point::new(pos.x, pos.y - self.font.metrics.ascent as i32);

        for chr in text.chars() {
            if let Some((metrics, _)) = self.font.get_metrics_and_data(chr) {
                total_width += metrics.advance as i32;
            }
        }

        let width = total_width.max(0) as u32;

        TextMetrics {
            next_position: Point::new(position.x + total_width, position.y),
            bounding_box: Rectangle::new(top_left, Size::new(width, full_height)),
        }
    }

    fn line_height(&self) -> u32 {
        (self.font.metrics.ascent as i32 - self.font.metrics.descent as i32
            + self.font.metrics.leading as i32) as u32
    }
}
