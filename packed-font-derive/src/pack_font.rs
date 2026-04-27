use anyhow::{Error, anyhow};
use bytemuck::bytes_of;
use skrifa::{
    metrics::{BoundingBox, GlyphMetrics},
    outline::{HintingInstance, HintingOptions},
    prelude::*,
};
use std::ops::RangeInclusive;

use crate::render::Bitmap;
use packed_font_structs::{AA_BITS, FontMetrics, Metrics as PackedMetrics};

#[derive(Debug)]
pub struct CompressedFont {
    pub metrics: FontMetrics,
    pub dict: Vec<u16>,
    pub font_data: Vec<u8>,
}

fn compress(data: impl ExactSizeIterator<Item = u8>, bits: u8) -> Vec<u8> {
    let max = 255_u8 >> (8 - bits);
    let total_count = data.len() as u32 * max as u32;
    let mut out = Vec::new();
    let mut covered = false;
    let mut count: u32 = 0;
    let mut sum_count: u32 = 0;
    for byte in data {
        let byte = byte >> (8 - bits);
        let addition = if covered { byte } else { max - byte };
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
    if count > 0 {
        out.push(count.try_into().unwrap());
    }
    sum_count += count;
    assert_eq!(sum_count, total_count);
    out
}

fn get_metrics(metrics: &GlyphMetrics, id: GlyphId) -> Result<(PackedMetrics, BoundingBox), Error> {
    let advance = (metrics
        .advance_width(id)
        .expect("Glyph has no advance width")
        .ceil() as i32)
        .try_into()?;
    let bbox = metrics.bounds(id).expect("Glyph has no bounds");
    let left_bearing = (bbox.x_min.floor() as i32).try_into()?;
    let top_bearing = (bbox.y_max.ceil() as i32).try_into()?;
    let x = bbox.x_max.ceil() as i32;
    let width = (x - left_bearing as i32).try_into()?;

    Ok((
        PackedMetrics {
            left_bearing,
            top_bearing,
            width,
            advance,
        },
        bbox,
    ))
}

impl CompressedFont {
    pub fn compress(
        font: impl AsRef<[u8]>,
        chars: RangeInclusive<u8>,
        size: u32,
        location: &[NormalizedCoord],
    ) -> Result<Self, Error> {
        let font = FontRef::new(font.as_ref())?;

        // Determine font size to match exact pixel size.
        let size = {
            let mut s = 1.0;
            let mut iter = 0;
            loop {
                iter += 1;
                if iter > 100 {
                    Err(anyhow!("Failed to compute font size"))?;
                }
                let metrics = font.metrics(Size::new(s), location);
                let pixel_height =
                    (metrics.ascent.ceil() as i32 - metrics.descent.floor() as i32) as u32;
                if pixel_height == size {
                    break s;
                }
                s = s * size as f32 / pixel_height as f32;
            }
        };

        let size = Size::new(size);
        let outlines = font.outline_glyphs();
        let glyph_metrics = font.glyph_metrics(size, location);
        let hinting = HintingInstance::new(&outlines, size, location, HintingOptions::default())?;
        let charmap = font.charmap();

        let mut dict = Vec::<u16>::new();
        let mut font_data = Vec::new();
        for chr in chars {
            dict.push(font_data.len().try_into()?);

            let Some(id) = charmap.map(chr) else {
                Err(anyhow!("Character '{}' not in the font", chr))?
            };

            let (metrics, bbox) = get_metrics(&glyph_metrics, id)?;
            let height = (bbox.y_max.ceil() - bbox.y_min.floor()) as u32;
            let bitmap = if metrics.width > 0 && height > 0 {
                let mut bitmap = Bitmap::new(-bbox.x_min, bbox.y_max, metrics.width as u32, height);
                let glyph = outlines.get(id).expect("Glyph has no outlines");
                bitmap.draw_glyph(&hinting, &glyph);
                Some(bitmap)
            } else {
                None
            };

            font_data.extend_from_slice(bytes_of(&metrics));
            if let Some(bitmap) = bitmap {
                let mut bitmap = compress(bitmap.pixels(), AA_BITS);
                font_data.append(&mut bitmap);
            }
        }

        let metrics = font.metrics(size, location);
        let ascent = (metrics.ascent.ceil() as i32).try_into()?;
        let descent = (metrics.descent.floor() as i32).try_into()?;
        let leading = (metrics.leading.ceil() as i32).try_into()?;

        Ok(Self {
            metrics: FontMetrics {
                ascent,
                descent,
                leading,
            },
            dict,
            font_data,
        })
    }
}
