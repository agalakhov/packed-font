use skrifa::{OutlineGlyph, outline::{OutlinePen, DrawSettings, HintingInstance}};
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Transform};

pub struct Bitmap {
    x0: f32,
    y0: f32,
    pixmap: Pixmap,
    path: Option<PathBuilder>,
}

impl Bitmap {
    pub fn new(x0: f32, y0: f32, width: u32, height: u32) -> Self {
        let pixmap = Pixmap::new(width, height)
            .unwrap();
        println!("Pixmap x0={x0} y0={y0}");
        Self {
            x0,
            y0,
            pixmap,
            path: None,
        }
    }

    pub fn draw_glyph(&mut self, hinter: &HintingInstance, glyph: &OutlineGlyph) {
        let settings = DrawSettings::hinted(hinter, false);
        glyph.draw(settings, self).unwrap();
        self.draw_path();
    }

    fn path(&mut self) -> &mut PathBuilder {
        self.path.get_or_insert_with(PathBuilder::new)
    }

    fn draw_path(&mut self) {
        if let Some(path) = self.path.take().and_then(PathBuilder::finish) {
            let mut paint = Paint::default();
            paint.set_color(Color::WHITE);
            self.pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
    }

    pub fn pixels(&self) -> impl ExactSizeIterator<Item = u8> {
        self.pixmap.pixels().iter().map(|p| p.alpha())
    }
}

impl OutlinePen for Bitmap {
    fn move_to(&mut self, x: f32, y: f32) {
        println!("move_to {x} {y} -> {} {}", self.x0 + x, self.y0 - y);
        let x0 = self.x0;
        let y0 = self.y0;
        self.path().move_to(x0 + x, y0 - y)
    }

    fn line_to(&mut self, x: f32, y: f32) {
        println!("line_to {x} {y} -> {} {}", self.x0 + x, self.y0 - y);
        let x0 = self.x0;
        let y0 = self.y0;
        self.path().line_to(x0 + x, y0 - y)
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        let x0 = self.x0;
        let y0 = self.y0;
        self.path().quad_to(x0 + cx0, y0 - cy0, x0 + x, y0 - y)
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        let x0 = self.x0;
        let y0 = self.y0;
        self.path().cubic_to(x0 + cx0, y0 - cy0, x0 + cx1, y0 - cy1, x0 + x, y0 - y);
    }

    fn close(&mut self) {
        self.path().close();
    }
}
