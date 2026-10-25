use std::sync::OnceLock;

use tiny_skia::Pixmap;

pub const WOOD_BG: &[u8] = include_bytes!("../assets/generated/wood_bg_1920x1080.png");
pub const PAPER_BOARD: &[u8] = include_bytes!("../assets/generated/paper_board_720.png");
pub const PAPER_PANEL: &[u8] = include_bytes!("../assets/generated/paper_panel_512x640.png");
pub const MARK_X: &[u8] = include_bytes!("../assets/generated/mark_x_512.png");
pub const MARK_O: &[u8] = include_bytes!("../assets/generated/mark_o_512.png");
pub const MARK_X_GHOST: &[u8] = include_bytes!("../assets/generated/mark_x_ghost_512.png");
pub const MARK_O_GHOST: &[u8] = include_bytes!("../assets/generated/mark_o_ghost_512.png");
pub const CURSOR_BRACKETS: &[u8] = include_bytes!("../assets/generated/cursor_brackets_320.png");
pub const DIVIDER_BRASS: &[u8] = include_bytes!("../assets/generated/divider_brass_640x32.png");
pub const ORNAMENT_CORNER: &[u8] = include_bytes!("../assets/generated/ornament_corner_128.png");
pub const VIGNETTE: &[u8] = include_bytes!("../assets/generated/vignette_1920x1080.png");
pub const GRAIN_OVERLAY: &[u8] = include_bytes!("../assets/generated/grain_overlay_1024.png");
pub const STAMP_WIN_PT: &[u8] = include_bytes!("../assets/generated/stamp_win_pt_640x200.png");
pub const STAMP_WIN_EN: &[u8] = include_bytes!("../assets/generated/stamp_win_en_640x200.png");
pub const STAMP_DRAW_PT: &[u8] = include_bytes!("../assets/generated/stamp_draw_pt_640x200.png");
pub const STAMP_DRAW_EN: &[u8] = include_bytes!("../assets/generated/stamp_draw_en_640x200.png");

pub const FONT_EB_GARAMOND_REGULAR: &[u8] =
    include_bytes!("../assets/fonts/EBGaramond-Regular.ttf");
pub const FONT_EB_GARAMOND_SEMIBOLD: &[u8] =
    include_bytes!("../assets/fonts/EBGaramond-SemiBold.ttf");
pub const FONT_INTER_REGULAR: &[u8] = include_bytes!("../assets/fonts/Inter-Regular.ttf");
pub const FONT_INTER_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/Inter-SemiBold.ttf");

pub const ALL_FONTS: &[(&str, &[u8])] = &[
    ("eb_garamond_regular", FONT_EB_GARAMOND_REGULAR),
    ("eb_garamond_semibold", FONT_EB_GARAMOND_SEMIBOLD),
    ("inter_regular", FONT_INTER_REGULAR),
    ("inter_semibold", FONT_INTER_SEMIBOLD),
];

pub const ALL_PNGS: &[(&str, &[u8])] = &[
    ("wood_bg", WOOD_BG),
    ("paper_board", PAPER_BOARD),
    ("paper_panel", PAPER_PANEL),
    ("mark_x", MARK_X),
    ("mark_o", MARK_O),
    ("mark_x_ghost", MARK_X_GHOST),
    ("mark_o_ghost", MARK_O_GHOST),
    ("cursor_brackets", CURSOR_BRACKETS),
    ("divider_brass", DIVIDER_BRASS),
    ("ornament_corner", ORNAMENT_CORNER),
    ("vignette", VIGNETTE),
    ("grain_overlay", GRAIN_OVERLAY),
    ("stamp_win_pt", STAMP_WIN_PT),
    ("stamp_win_en", STAMP_WIN_EN),
    ("stamp_draw_pt", STAMP_DRAW_PT),
    ("stamp_draw_en", STAMP_DRAW_EN),
];

pub fn decode(bytes: &[u8]) -> Option<Pixmap> {
    Pixmap::decode_png(bytes).ok()
}

macro_rules! cached_pixmap {
    ($name:ident, $bytes:ident) => {
        pub fn $name() -> &'static Pixmap {
            static CACHE: OnceLock<Pixmap> = OnceLock::new();
            CACHE.get_or_init(|| decode($bytes).expect(concat!(stringify!($bytes), " is embedded")))
        }
    };
}

cached_pixmap!(paper_board, PAPER_BOARD);
cached_pixmap!(mark_x, MARK_X);
cached_pixmap!(mark_o, MARK_O);
cached_pixmap!(mark_x_ghost, MARK_X_GHOST);
cached_pixmap!(mark_o_ghost, MARK_O_GHOST);
cached_pixmap!(cursor_brackets, CURSOR_BRACKETS);
cached_pixmap!(stamp_win_pt, STAMP_WIN_PT);
cached_pixmap!(stamp_win_en, STAMP_WIN_EN);
cached_pixmap!(stamp_draw_pt, STAMP_DRAW_PT);
cached_pixmap!(stamp_draw_en, STAMP_DRAW_EN);

pub const fn all() -> &'static [(&'static str, &'static [u8])] {
    ALL_PNGS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_embedded_png_decodes() {
        for (name, bytes) in ALL_PNGS {
            let pixmap = decode(bytes).unwrap_or_else(|| panic!("{name} did not decode"));

            assert!(pixmap.width() > 0);
            assert!(pixmap.height() > 0);
        }
    }

    #[test]
    fn every_embedded_font_is_a_ttf() {
        for (name, bytes) in ALL_FONTS {
            assert!(bytes.len() > 1_000, "{name} is too small");
            let magic = &bytes[..4];

            assert!(
                magic == [0x00, 0x01, 0x00, 0x00] || magic == b"OTTO",
                "{name} is not a TTF/OTF file"
            );
        }
    }

    #[test]
    fn expected_dimensions_match_the_manifest() {
        let expected = [
            ("wood_bg", 1920, 1080),
            ("paper_board", 720, 720),
            ("paper_panel", 512, 640),
            ("mark_x", 512, 512),
            ("mark_o", 512, 512),
            ("cursor_brackets", 320, 320),
            ("divider_brass", 640, 32),
        ];

        for (name, width, height) in expected {
            let (_, bytes) = ALL_PNGS
                .iter()
                .find(|(candidate, _)| *candidate == name)
                .unwrap();
            let pixmap = decode(bytes).unwrap();

            assert_eq!((pixmap.width(), pixmap.height()), (width, height));
        }
    }
}
