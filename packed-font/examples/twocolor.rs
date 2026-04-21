use packed_font::{PackedFont, twocolor::TwoColor, CharacterStyle, packed_font};

use embedded_graphics::{geometry::{Size, Point}, pixelcolor::{RgbColor, Rgb565}, text::Text, Drawable};
use embedded_graphics_simulator::{SimulatorDisplay, OutputSettingsBuilder, Window};

const FONT: PackedFont = packed_font!("/usr/share/fonts/TTF/DejaVuSans.ttf", 16);

fn main() {
    let mut display = SimulatorDisplay::<Rgb565>::new(Size::new(284, 76));

    let colors = TwoColor {
        foreground: Rgb565::YELLOW,
        background: Rgb565::BLUE,
    };

    let style = CharacterStyle {
        font: &FONT,
        style: colors,
    };

    Text::new("Hello World", Point::zero(), style).draw(&mut display)
        .expect("Error rendering text");

    let output_settings = OutputSettingsBuilder::new()
        .scale(8)
        .build();
    Window::new("Two color demo", &output_settings).show_static(&display);
}
