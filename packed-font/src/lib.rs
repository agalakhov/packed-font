#![no_std]

use bytemuck::from_bytes;

pub use packed_font_derive::packed_font;
pub use packed_font_structs::Metrics;

use packed_font_structs::AA_BITS;

pub trait UnpackTarget {
    fn push_color(&mut self, coverage: u8);
}

#[derive(Debug)]
pub struct PackedFont {
    pub first_char: u8,
    pub dict: &'static [u16],
    pub data: &'static [u8],
}

impl PackedFont {
    pub fn render(&self, character: char, out: &mut impl UnpackTarget) -> Option<&Metrics> {
        let character: u8 = character.try_into().ok()?;
        if character < self.first_char {
            return None
        }
        let idx = (character - self.first_char) as usize;
        let Some(offset) = self.dict.get(idx).map(|x| (*x) as usize) else {
            return None
        };
        let end_offset = self.dict.get(idx + 1).map(|x| (*x) as usize).unwrap_or(self.data.len());
        let raw = &self.data[offset .. end_offset];
        let metrics = from_bytes(raw);
        let packed = &raw[size_of::<PackedFont>()..];

        unpack(packed, out);

        Some(metrics)
    }
}

fn unpack(packed: &[u8], out: &mut impl UnpackTarget) {
    const MAX: u8 = 255 >> (8 - AA_BITS);
    let mut covered = false;
    let mut tail = 0;
    for byte in packed {
        let mut count = *byte - tail;
        while count > MAX {
            out.push_color(if covered {
                MAX
            } else {
                0
            });
            count -= MAX;
        }
        if count > 0 {
            tail = MAX - count;
            out.push_color(if covered {
                count
            } else {
                tail
            });
            covered = !covered;
        }
    }
}

