#![no_std]

mod blend;
mod unpack;

pub mod twocolor;

use bytemuck::from_bytes;
use embedded_graphics_core::{Pixel, geometry::Point};

use self::unpack::Unpacker;

pub use packed_font_derive::packed_font;
pub use packed_font_structs::{AaColor, Metrics};

pub trait UnpackTarget {
    type Error;
    fn draw_iter(
        &mut self,
        metrics: &Metrics,
        iter: impl Iterator<Item = Pixel<AaColor>>,
    ) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub struct PackedFont {
    pub first_char: u8,
    pub dict: &'static [u16],
    pub data: &'static [u8],
}

impl PackedFont {
    pub fn render<T: UnpackTarget>(
        &self,
        character: char,
        out: &mut T,
    ) -> Result<Option<&Metrics>, T::Error> {
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
        let metrics: &Metrics = from_bytes(raw);
        let packed = &raw[size_of::<PackedFont>()..];

        let mut x = 0;
        let mut y = 0;
        let w = metrics.width as i32;
        let pixels = Unpacker::new(packed.iter().cloned()).map(|color| {
            x += 1;
            if x >= w {
                y += 1;
                x = 0;
            }
            let pt = Point::new(x, y);
            Pixel(pt, color)
        });

        out.draw_iter(metrics, pixels)?;

        Ok(Some(metrics))
    }
}
