use packed_font::{CharacterStyle, PackedFont, packed_font, twocolor::TwoColor};

use embedded_graphics::{
    Drawable,
    geometry::{Point, Size},
    pixelcolor::{Rgb565, RgbColor},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

const FONT: PackedFont = packed_font!("/home/agalakhov/.local/share/fonts/din1451alt.ttf", 76);

fn main() {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(284, 76));

    let colors = TwoColor {
        foreground: Rgb565::GREEN,
        background: Rgb565::BLACK,
    };

    let style = CharacterStyle {
        font: &FONT,
        style: colors,
    };

    Text::with_text_style(
        "{A}{B} 27°C",
        Point::new(142, 38),
        style,
        TextStyleBuilder::new()
            .alignment(Alignment::Center)
            .baseline(Baseline::Middle)
            .build(),
    )
    .draw(&mut display)
    .expect("Error rendering text");

    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    Window::new("Two color demo", &output_settings).show_static(&display);
}
