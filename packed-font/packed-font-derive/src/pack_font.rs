use anyhow::{anyhow, Error};
use fontdue::{Font, Metrics};
use std::{ops::RangeInclusive, };
use bytemuck::bytes_of;

use packed_font_structs::{AA_BITS, FontMetrics, Metrics as PackedMetrics};

#[derive(Debug)]
pub struct CompressedFont {
    pub metrics: FontMetrics,
    pub dict: Vec<u16>,
    pub font_data: Vec<u8>,
}

fn compress(data: Vec<u8>, bits: u8) -> Vec<u8> {
    let max = 255_u8 >> (8 - bits);
    let total_count = data.len() as u32 * max as u32;
    let mut out = Vec::new();
    let mut covered = false;
    let mut count: u32 = 0;
    let mut sum_count: u32 = 0;
    for byte in data {
        let byte = byte >> (8 - bits);
        let addition = if covered {
            byte
        } else {
            max - byte
        };
        if count + addition as u32 > u8::MAX as u32 {
            sum_count += count;
            out.push(count.try_into().unwrap());
            out.push(0);
            count = 0;
        }
        count += addition as u32;
        if addition < max {
            sum_count += count;
            out.push(count.try_into().unwrap());
            count = max as u32 - addition as u32;
            covered = !covered;
        }
    }
    sum_count += count as u32;
    assert_eq!(sum_count, total_count);
    out
}

fn pack_metrics(m: Metrics) -> Result<PackedMetrics, Error> {
    Ok(PackedMetrics {
        xmin: m.xmin.try_into()?,
        ymin: m.ymin.try_into()?,
        width: m.width.try_into()?,
        advance: m.advance_width.round() as u8,
    })
}

impl CompressedFont {
    pub fn compress(font: impl AsRef<[u8]>, chars: RangeInclusive<u8>, size: f32) -> Result<Self, Error> {
        let font = Font::from_bytes(font.as_ref(), Default::default())
            .map_err(|e| anyhow!(e))?;

        let mut dict = Vec::<u16>::new();
        let mut font_data = Vec::new();
        for chr in chars {
            dict.push(font_data.len().try_into()?);
            let (metrics, bitmap) = font.rasterize(chr as char, size);
            let metrics = pack_metrics(metrics)?;
            font_data.extend_from_slice(bytes_of(&metrics));
            if ! bitmap.is_empty() {
                let mut bitmap = compress(bitmap, AA_BITS);
                font_data.append(&mut bitmap);
            }
        }

        let line_metrics = font.horizontal_line_metrics(size)
            .ok_or_else(|| anyhow!("This font does not support horizontal lines"))?;

        Ok(Self {
            metrics: FontMetrics {
                line_height: (line_metrics.new_line_size.ceil() as u32).try_into()?,
            },
            dict,
            font_data,
        })
    }
}
