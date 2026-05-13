# Compile-time TTF-to-bitmap font rasterizer for embedded systems

High-quality font rendering for small microcontrollers with big displays
compatible with [https://crates.io/crates/embedded-graphics] ecosystem.

Many microcontrollers with high-resolution displays do not have sufficient
memory to render TrueType and OpenType fonts on their own. While they
theoretically can render vector fonts, the quality will be less optimal due
to lack of hinting and limited antialiasing capabilities of a integer-only
system.

`packed-font` pre-renders a subset of font glyphs in chosen quality with
full hinting and 4-bit antialiasing and compresses them with modified RLE
algorithm especially suitable for larger glyphs. It is thus possible to have
a fully-antialiased 72px high font in under 16 kB of ROM. Smaller fonts may
take as little as 4 kB depending on their glyph size. Extracting does not
require neither dynamic nor static memory allocation and is done on-the-fly.

Pre-rendering is done at compile time with a macro named `packed_font!`
which is quite similar to `include_bytes!`. Pre-rendered font is included
directly, no intermediate files are needed.
This macro uses [https://crates.io/crates/skrifa]
and [https://crates.io/crates/tiny-skia] internally.

## Usage example

```rust
use packed_font::{CharacterStyle, PackedFont, packed_font, twocolor::TwoColor};

const FONT: PackedFont = packed_font!("/usr/share/fonts/TTF/DejaVuSerif.ttf", 48);

let colors = TwoColor {
    foreground: Rgb565::GREEN,
    background: Rgb565::BLACK,
};

let style = CharacterStyle {
    font: &FONT,
    style: colors,
};

Text::new(
    "Hello, World!",
    Point::new(10, 40),
    style,
)
.draw(&mut display)?
```

See `examples/` directory in the repository for more.

## Limitations

* Only 7-bit ASCII (codes from 0x20 to 0x7e) and degree symbol (0xB0)
  are used to save space. No support for other languages (yet).

* Compression is less efficient with smaller fonts. For very small sizes
  consider using bitmap fonts instead.

## Acknowledgements

This crate is inspided by [https://crates.io/crates/minitype]
which uses similar pre-rendering with external tool and no compression,
just 4-bpp plain bitmap.
