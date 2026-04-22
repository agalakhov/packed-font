#![no_std]

use bytemuck::{NoUninit, AnyBitPattern};
use embedded_graphics_core::pixelcolor::Gray4;

pub const AA_BITS: u8 = 4;
pub type AaColor = Gray4;

#[derive(Debug, Clone)]
pub struct FontMetrics {
    pub line_height: u8,
}

#[derive(Debug, NoUninit, AnyBitPattern, Clone, Copy)]
#[repr(C)]
pub struct Metrics {
    pub left_bearing: i8,
    pub top_bearing: i8,
    pub width: u8,
    pub advance: u8,
}

