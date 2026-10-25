use std::cell::RefCell;
use std::collections::HashMap;

use fontdue::layout::{
    CoordinateSystem, HorizontalAlign, Layout, LayoutSettings, TextStyle as FontdueStyle,
    VerticalAlign, WrapStyle,
};
use fontdue::{Font, FontSettings, Metrics};
use tiny_skia::Pixmap;

use crate::assets;
use crate::theme::{Rgb, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Face {
    Serif,
    SerifBold,
    Sans,
    SansBold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    pub face: Face,
    pub px: f32,
    pub color: Rgb,
    pub align: Align,
    pub line_height: f32,
}

impl TextStyle {
    pub const fn new(face: Face, px: f32, token: Token) -> Self {
        Self {
            face,
            px,
            color: token.rgb(),
            align: Align::Left,
            line_height: 1.15,
        }
    }

    pub const fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub const fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = line_height;
        self
    }

    pub const fn color(mut self, token: Token) -> Self {
        self.color = token.rgb();
        self
    }
}

pub struct Fonts {
    serif: Font,
    serif_bold: Font,
    sans: Font,
    sans_bold: Font,
}

impl Fonts {
    pub fn embedded() -> Self {
        let load = |bytes: &[u8]| {
            Font::from_bytes(bytes, FontSettings::default()).expect("embedded font is valid")
        };

        Self {
            serif: load(assets::FONT_EB_GARAMOND_REGULAR),
            serif_bold: load(assets::FONT_EB_GARAMOND_SEMIBOLD),
            sans: load(assets::FONT_INTER_REGULAR),
            sans_bold: load(assets::FONT_INTER_SEMIBOLD),
        }
    }

    pub fn font(&self, face: Face) -> &Font {
        match face {
            Face::Serif => &self.serif,
            Face::SerifBold => &self.serif_bold,
            Face::Sans => &self.sans,
            Face::SansBold => &self.sans_bold,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GlyphKey {
    face: Face,
    index: u16,
    px: u32,
}

pub struct TextRenderer {
    fonts: Fonts,
    glyphs: RefCell<HashMap<GlyphKey, (Metrics, Vec<u8>)>>,
}

impl TextRenderer {
    pub fn new() -> Self {
        Self {
            fonts: Fonts::embedded(),
            glyphs: RefCell::new(HashMap::new()),
        }
    }

    pub fn draw(
        &self,
        pixmap: &mut Pixmap,
        text: &str,
        style: TextStyle,
        x: f32,
        y: f32,
        max_width: f32,
    ) -> (f32, f32) {
        let (layout, _) = self.layout(text, style, x, y, max_width);

        for glyph in layout.glyphs() {
            if glyph.width == 0 || glyph.height == 0 {
                continue;
            }

            let key = GlyphKey {
                face: style.face,
                index: glyph.key.glyph_index,
                px: glyph.key.px.to_bits(),
            };

            if !self.glyphs.borrow().contains_key(&key) {
                let raster = self
                    .fonts
                    .font(style.face)
                    .rasterize_indexed(glyph.key.glyph_index, glyph.key.px);

                self.glyphs.borrow_mut().insert(key, raster);
            }

            let glyphs = self.glyphs.borrow();
            let (_, coverage) = glyphs.get(&key).expect("glyph was cached");

            blend(
                pixmap,
                glyph.x.round() as i32,
                glyph.y.round() as i32,
                glyph.width,
                glyph.height,
                coverage,
                style.color,
            );
        }

        self.bounds(&layout, x, y)
    }

    pub fn measure(&self, text: &str, style: TextStyle, max_width: f32) -> (f32, f32) {
        let (layout, start) = self.layout(text, style, 0.0, 0.0, max_width);

        self.bounds(&layout, start.0, start.1)
    }

    fn layout(
        &self,
        text: &str,
        style: TextStyle,
        x: f32,
        y: f32,
        max_width: f32,
    ) -> (Layout<()>, (f32, f32)) {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        let settings = LayoutSettings {
            x,
            y,
            max_width: Some(max_width.max(1.0)),
            max_height: None,
            horizontal_align: match style.align {
                Align::Left => HorizontalAlign::Left,
                Align::Center => HorizontalAlign::Center,
                Align::Right => HorizontalAlign::Right,
            },
            vertical_align: VerticalAlign::Top,
            line_height: style.line_height,
            wrap_style: WrapStyle::Word,
            wrap_hard_breaks: true,
        };

        layout.reset(&settings);

        let font = self.fonts.font(style.face);
        layout.append(&[font], &FontdueStyle::new(text, style.px, 0));

        (layout, (x, y))
    }

    fn bounds(&self, layout: &Layout<()>, x: f32, y: f32) -> (f32, f32) {
        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        for glyph in layout.glyphs() {
            width = width.max(glyph.x + glyph.width as f32 - x);
            height = height.max(glyph.y + glyph.height as f32 - y);
        }

        (width.max(0.0), height.max(0.0))
    }
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::too_many_arguments)]
fn blend(
    pixmap: &mut Pixmap,
    x: i32,
    y: i32,
    width: usize,
    height: usize,
    coverage: &[u8],
    color: Rgb,
) {
    let pixmap_width = pixmap.width() as i32;
    let pixmap_height = pixmap.height() as i32;
    let data = pixmap.data_mut();

    for row in 0..height {
        let py = y + row as i32;

        if py < 0 || py >= pixmap_height {
            continue;
        }

        for column in 0..width {
            let px = x + column as i32;

            if px < 0 || px >= pixmap_width {
                continue;
            }

            let alpha = u32::from(coverage[row * width + column]);

            if alpha == 0 {
                continue;
            }

            let index = ((py * pixmap_width + px) * 4) as usize;
            let inverse = 255 - alpha;
            let source_red = u32::from(color.red) * alpha;
            let source_green = u32::from(color.green) * alpha;
            let source_blue = u32::from(color.blue) * alpha;

            data[index] = ((source_red + u32::from(data[index]) * inverse + 127) / 255) as u8;
            data[index + 1] =
                ((source_green + u32::from(data[index + 1]) * inverse + 127) / 255) as u8;
            data[index + 2] =
                ((source_blue + u32::from(data[index + 2]) * inverse + 127) / 255) as u8;
            data[index + 3] =
                ((alpha * 255 + u32::from(data[index + 3]) * inverse + 127) / 255) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_fonts_load() {
        let fonts = Fonts::embedded();

        for face in [Face::Serif, Face::SerifBold, Face::Sans, Face::SansBold] {
            assert!(fonts.font(face).units_per_em() > 0.0);
        }
    }

    #[test]
    fn draw_changes_pixels_inside_the_text_box() {
        let mut pixmap = Pixmap::new(400, 120).unwrap();
        let before = pixmap.data().to_vec();
        let text = TextRenderer::new();

        let (width, height) = text.draw(
            &mut pixmap,
            "Velha Clássica",
            TextStyle::new(Face::SerifBold, 48.0, Token::BrassBright),
            10.0,
            20.0,
            380.0,
        );

        assert!(width > 0.0);
        assert!(height > 0.0);
        assert_ne!(pixmap.data(), before);
    }

    #[test]
    fn measure_reports_a_box_that_fits_the_text() {
        let text = TextRenderer::new();
        let style = TextStyle::new(Face::Sans, 24.0, Token::TextOnWood);
        let (width, height) = text.measure("two players", style, 600.0);

        assert!(width > 40.0);
        assert!(height >= 20.0);
        assert!(width < 600.0);
    }

    #[test]
    fn wrapping_respects_the_maximum_width() {
        let text = TextRenderer::new();
        let style = TextStyle::new(Face::Sans, 24.0, Token::TextOnWood);
        let (narrow, _) = text.measure("the quick brown fox jumps", style, 120.0);
        let (wide, _) = text.measure("the quick brown fox jumps", style, 600.0);

        assert!(narrow <= 130.0);
        assert!(wide > narrow);
    }

    #[test]
    fn centered_text_stays_inside_the_box() {
        let mut pixmap = Pixmap::new(400, 80).unwrap();
        let text = TextRenderer::new();

        text.draw(
            &mut pixmap,
            "Centered",
            TextStyle::new(Face::Sans, 24.0, Token::TextOnWood).align(Align::Center),
            0.0,
            10.0,
            400.0,
        );

        assert!(pixmap.data().iter().any(|byte| *byte != 0));
    }
}
