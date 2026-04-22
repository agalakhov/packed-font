//#![no_std]

mod blend;
mod textrenderer;
mod unpack;

pub mod twocolor;

use bytemuck::from_bytes;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    pixelcolor::PixelColor,
    primitives::Rectangle,
};

use self::unpack::Unpacker;

pub use packed_font_derive::packed_font;
pub use packed_font_structs::{AaColor, FontMetrics, Metrics};

pub use textrenderer::CharacterStyle;

pub trait UnpackStyle {
    type Color: PixelColor;
    fn map_color(&self, grade: AaColor) -> Self::Color;
    fn background_color(&self) -> Option<Self::Color>;
}

#[derive(Debug)]
pub struct PackedFont {
    pub metrics: FontMetrics,
    pub first_char: u8,
    pub dict: &'static [u16],
    pub data: &'static [u8],
}

impl PackedFont {
    pub fn render<S, D>(
        &self,
        character: char,
        style: &S,
        origin: Point,
        target: &mut D,
    ) -> Result<Option<&Metrics>, D::Error>
    where
        S: UnpackStyle,
        D: DrawTarget<Color = S::Color>,
    {
        let Ok(character) = TryInto::<u8>::try_into(character) else {
            return Ok(None);
        };
        if character < self.first_char {
            return Ok(None);
        }
        let idx = (character - self.first_char) as usize;
        let Some(offset) = self.dict.get(idx).map(|x| (*x) as usize) else {
            return Ok(None);
        };
        let end_offset = self
            .dict
            .get(idx + 1)
            .map(|x| (*x) as usize)
            .unwrap_or(self.data.len());
        let raw = &self.data[offset..end_offset];
        let (metrics, packed) = raw.split_at(size_of::<Metrics>());
        let metrics: &Metrics = from_bytes(metrics);

        let w = metrics.width as u32;
        let h = 100;
        let lsb = metrics.left_bearing as i32;
        let tsb = metrics.top_bearing as i32;
        let origin = Point::new(origin.x + lsb, origin.y - tsb);

        let rect = Rectangle::new(origin, Size::new(w, h));

        let pixels = Unpacker::new(packed.iter().cloned()).map(|a| style.map_color(a));

        target.fill_contiguous(&rect, pixels)?;

        Ok(Some(metrics))
    }
}
