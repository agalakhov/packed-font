use packed_font::{CharacterStyle, PackedFont, packed_font, twocolor::TwoColor};

use embedded_graphics::{
    Drawable,
    geometry::{Point, Size},
    pixelcolor::{Rgb565, RgbColor},
    text::{Baseline, Text},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

const FONT: PackedFont = packed_font!("din1451alt.ttf", 36);

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

    Text::with_baseline("Hello World", Point::zero(), style, Baseline::Top)
        .draw(&mut display)
        .expect("Error rendering text");

    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    Window::new("Two color demo", &output_settings).show_static(&display);
}
