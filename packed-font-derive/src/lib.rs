use derive_syn_parse::Parse;
use proc_macro::{self, TokenStream};
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;
use std::{fs::read, path::PathBuf};
use syn::{LitInt, LitStr, Token, parse_macro_input};

use packed_font_structs::{FontMetrics, all_chars};

mod pack_font;
mod render;
use pack_font::CompressedFont;

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
    } = match CompressedFont::compress(bytes, all_chars(), size as u32, &[][..]) {
        Ok(packed) => packed,
        Err(e) => abort_call_site!("Can't compress file '{}': {}", &file.to_string_lossy(), e),
    };

    let FontMetrics {
        ascent,
        descent,
        leading,
    } = metrics;

    quote! {
        ::packed_font::PackedFont {
            metrics: ::packed_font::FontMetrics {
                ascent: #ascent,
                descent: #descent,
                leading: #leading,
            },
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
