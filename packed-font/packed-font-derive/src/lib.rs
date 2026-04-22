use derive_syn_parse::Parse;
use proc_macro::{self, TokenStream};
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use std::{fs::read, ops::RangeInclusive, path::PathBuf};
use syn::{LitInt, LitStr, Token, parse_macro_input};

use packed_font_structs::FontMetrics;

mod pack_font;
mod render;
use pack_font::CompressedFont;

const ALL_CHARS: RangeInclusive<u8> = 0x20..=0x7e;

#[derive(Parse)]
struct Input {
    file: LitStr,
    _comma: Token![,],
    size: LitInt,
}

#[proc_macro]
#[proc_macro_error]
pub fn packed_font(tokens: TokenStream) -> TokenStream {
    let Input { file, size, .. } = parse_macro_input!(tokens);
    let src_file = file.span().file();
    let file = file.value();
    let size: u8 = size.base10_parse().expect("Size must be `u8`");

    let src_file = PathBuf::from(src_file);
    let file = PathBuf::from(file);
    let file = if file.is_relative() {
        if let Some(path) = src_file.parent() {
            let mut path = path.to_owned();
            path.push(file);
            path
        } else {
            file
        }
    } else {
        file
    };
    let bytes = match read(&file) {
        Ok(file) => file,
        Err(e) => abort_call_site!("Can't read file '{}': {}", &file.to_string_lossy(), e),
    };

    let CompressedFont {
        metrics,
        dict,
        font_data,
    } = match CompressedFont::compress(bytes, ALL_CHARS, size as f32, &[][..]) {
        Ok(packed) => packed,
        Err(e) => abort_call_site!("Can't compress file '{}': {}", &file.to_string_lossy(), e),
    };

    let first_char = ALL_CHARS.start();

    let FontMetrics { line_height } = metrics;

    quote! {
        ::packed_font::PackedFont {
            metrics: ::packed_font::FontMetrics {
                line_height: #line_height,
            },
            first_char: #first_char,
            dict: &[
                #(#dict),*
            ],
            data: &[
                #(#font_data),*
            ],
        }
    }
    .into()
}
