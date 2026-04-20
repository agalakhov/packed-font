#![no_std]

use bytemuck::{NoUninit, AnyBitPattern};

pub const AA_BITS: u8 = 4;

#[derive(Debug, NoUninit, AnyBitPattern, Clone, Copy)]
#[repr(C)]
pub struct Metrics {
    pub xmin: i8,
    pub ymin: i8,
    pub width: u8,
    pub advance: u8,
}

