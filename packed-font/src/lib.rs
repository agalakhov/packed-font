#![no_std]

mod blend;
mod unpack;
mod textrenderer;

pub mod twocolor;

use bytemuck::from_bytes;
use embedded_graphics_core::{Pixel, draw_target::DrawTarget, pixelcolor::PixelColor, geometry::{Point, Size}};

use self::unpack::Unpacker;

pub use packed_font_derive::packed_font;
pub use packed_font_structs::{AaColor, Metrics, FontMetrics};

pub use textrenderer::CharacterStyle;

pub trait UnpackStyle {
    type Color: PixelColor;
    fn draw_iter<D: DrawTarget<Color = Self::Color>>(
        &self,
        metrics: &Metrics,
        target: &mut D,
        iter: impl Iterator<Item = Pixel<AaColor>>,
    ) -> Result<(), D::Error>;
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
            return Ok(None)
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

        let mut x = 0;
        let mut y = 0;
        let w = metrics.width as u32;
        let pixels = Unpacker::new(packed.iter().cloned()).map(|color| {
            let pt = origin + Size::new(x, y);
            x += 1;
            if x >= w {
                y += 1;
                x = 0;
            }
            Pixel(pt, color)
        });

        style.draw_iter(metrics, target, pixels)?;

        Ok(Some(metrics))
    }
}
