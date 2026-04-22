use core::{iter::Fuse, mem::replace};

use packed_font_structs::{AA_BITS, AaColor};

pub struct Unpacker<I> {
    packed: Fuse<I>,
    covered: bool,
    count: i16,
}

impl<I> Unpacker<I>
where
    I: Iterator<Item = u8>,
{
    pub fn new(packed: I) -> Self {
        let mut packed = packed.fuse();
        let count = packed.next().unwrap_or(0) as i16;
        Self {
            packed,
            covered: false,
            count,
        }
    }
}

impl<I> Iterator for Unpacker<I>
where
    I: Iterator<Item = u8>,
{
    type Item = AaColor;
    fn next(&mut self) -> Option<Self::Item> {
        const MAX: u8 = 255 >> (8 - AA_BITS);
        loop {
            if self.count > 0 {
                let coverage = if self.count < MAX as i16 {
                    self.count.try_into().unwrap()
                } else {
                    MAX
                };
                self.count -= MAX as i16;
                break Some(AaColor::new(if self.covered {
                    coverage
                } else {
                    MAX - coverage
                }));
            } else {
                self.covered = !self.covered;
                let Some(count) = self.packed.next() else {
                    break None;
                };
                self.count += count as i16;
            }
        }
    }
}
