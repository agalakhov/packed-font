use embedded_graphics::pixelcolor::{
    Bgr555, Bgr565, Bgr666, Bgr888, GrayColor, PixelColor, Rgb555, Rgb565, Rgb666, Rgb888, RgbColor,
};

use packed_font_structs::{AA_BITS, AaColor};

pub trait Blend<C> {
    type Target: PixelColor;
    fn blend(&self, c: &C, a: AaColor) -> Self::Target;
}

fn rgb_blend<C: RgbColor>(c1: &C, c2: &C, a: AaColor) -> (u8, u8, u8) {
    (
        blend(c1.r(), c2.r(), a),
        blend(c1.g(), c2.g(), a),
        blend(c1.b(), c2.b(), a),
    )
}

fn blend(v1: u8, v2: u8, a: AaColor) -> u8 {
    let a = a.luma();
    const MAX: u8 = 255 >> (8 - AA_BITS);
    ((v1 as u16 * a as u16 + v2 as u16 * (MAX - a) as u16) / MAX as u16) as u8
}

macro_rules! impl_blend_rgb {
    ($name: ident) => {
        impl Blend<$name> for $name {
            type Target = $name;
            fn blend(&self, c: &$name, a: AaColor) -> Self::Target {
                let (r, g, b) = rgb_blend(self, c, a);
                $name::new(r, g, b)
            }
        }
    };
}

impl_blend_rgb!(Rgb555);
impl_blend_rgb!(Rgb565);
impl_blend_rgb!(Rgb666);
impl_blend_rgb!(Rgb888);
impl_blend_rgb!(Bgr555);
impl_blend_rgb!(Bgr565);
impl_blend_rgb!(Bgr666);
impl_blend_rgb!(Bgr888);
