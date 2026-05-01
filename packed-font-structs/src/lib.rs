#![no_std]

use bytemuck::{AnyBitPattern, NoUninit};
use embedded_graphics_core::pixelcolor::Gray4;

pub const AA_BITS: u8 = 4;
pub type AaColor = Gray4;

#[derive(Debug, Clone)]
pub struct FontMetrics {
    pub ascent: i8,
    pub descent: i8,
    pub leading: u8,
}

#[derive(Debug, NoUninit, AnyBitPattern, Clone, Copy)]
#[repr(C)]
pub struct Metrics {
    pub left_bearing: i8,
    pub top_bearing: i8,
    pub width: u8,
    pub advance: u8,
}

pub fn all_chars() -> impl Iterator<Item = char> {
    ('\x20'..='\x7e').into_iter().chain(['°'])
}

pub const fn map_character(chr: char) -> Option<u8> {
    const BASE: u8 = 0x20;
    match chr {
        '\x20'..='\x7e' => Some(chr as u8 - BASE),
        '°' => Some(0x7f - BASE),
        _ => None,
    }
}
