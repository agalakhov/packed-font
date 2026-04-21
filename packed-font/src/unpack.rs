use core::{iter::Fuse, mem::replace};

use packed_font_structs::{AA_BITS, AaColor};

pub struct Unpacker<I> {
    packed: Fuse<I>,
    covered: bool,
    tail: u8,
    count: u8,
}

impl<I> Unpacker<I>
where
    I: Iterator<Item = u8>,
{
    pub fn new(packed: I) -> Self {
        let mut packed = packed.fuse();
        let count = packed.next().unwrap_or(0);
        Self {
            packed,
            covered: false,
            tail: 0,
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
            if self.count >= MAX {
                self.count -= MAX;
                break Some(AaColor::new(if self.covered { MAX } else { 0 }));
            } else if self.count > 0 {
                let count = replace(&mut self.count, 0);
                self.tail = MAX - count;
                let covered = self.covered;
                let covered = replace(&mut self.covered, !covered);
                break Some(AaColor::new(if covered { count } else { self.tail }));
            } else {
                let Some(count) = self.packed.next() else {
                    break None;
                };
                self.count = count - replace(&mut self.tail, 0);
            }
        }
    }
}
