use std::path::Path;

use tiny_skia::{Pixmap, PixmapPaint, Transform};

use crate::assets;
use crate::core::{Board, Mark};
use crate::i18n::Language;
use crate::render::scene::{self, SceneInput, Stamp};

pub const WIDTH: u32 = 1344;
pub const HEIGHT: u32 = 756;

pub fn render(language: Language) -> Pixmap {
    let mut pixmap = Pixmap::new(WIDTH, HEIGHT).expect("canvas dimensions are valid");

    let wood = assets::decode(assets::WOOD_BG).expect("embedded wood background");
    pixmap.draw_pixmap(
        0,
        0,
        wood.as_ref(),
        &PixmapPaint::default(),
        Transform::from_scale(
            WIDTH as f32 / wood.width() as f32,
            HEIGHT as f32 / wood.height() as f32,
        ),
        None,
    );

    let mut board = Board::new();
    board.place(0, Mark::X).unwrap();
    board.place(1, Mark::X).unwrap();
    board.place(2, Mark::X).unwrap();
    board.place(3, Mark::O).unwrap();
    board.place(4, Mark::O).unwrap();
    let scene = scene::render(
        &board,
        SceneInput {
            selected: 8,
            turn: Mark::O,
            show_cursor: false,
            cursor_opacity: 1.0,
            winning_line: Some([0, 1, 2]),
            win_progress: 1.0,
            last_move: None,
            mark_progress: 1.0,
            stamp: Some(match language {
                Language::PtBr => Stamp::WinPt,
                Language::En => Stamp::WinEn,
            }),
        },
    );

    draw_fitted(&mut pixmap, &scene, 154, 170, 520, 520);

    let panel = assets::decode(assets::PAPER_PANEL).expect("embedded paper panel");
    draw_fitted(&mut pixmap, &panel, 730, 170, 460, 520);

    let grain = assets::decode(assets::GRAIN_OVERLAY).expect("embedded grain overlay");

    for y in (0..HEIGHT).step_by(1024) {
        for x in (0..WIDTH).step_by(1024) {
            pixmap.draw_pixmap(
                x as i32,
                y as i32,
                grain.as_ref(),
                &PixmapPaint {
                    opacity: 0.08,
                    ..PixmapPaint::default()
                },
                Transform::identity(),
                None,
            );
        }
    }

    let vignette = assets::decode(assets::VIGNETTE).expect("embedded vignette");
    pixmap.draw_pixmap(
        0,
        0,
        vignette.as_ref(),
        &PixmapPaint::default(),
        Transform::from_scale(
            WIDTH as f32 / vignette.width() as f32,
            HEIGHT as f32 / vignette.height() as f32,
        ),
        None,
    );

    pixmap
}

pub fn write(path: &Path, language: Language) -> Result<(), String> {
    let pixmap = render(language);
    let png = pixmap.encode_png().map_err(|error| error.to_string())?;

    std::fs::write(path, png).map_err(|error| error.to_string())
}

fn draw_fitted(target: &mut Pixmap, source: &Pixmap, x: u32, y: u32, width: u32, height: u32) {
    let scale = (width as f32 / source.width() as f32).min(height as f32 / source.height() as f32);
    let drawn_width = source.width() as f32 * scale;
    let drawn_height = source.height() as f32 * scale;
    let transform = Transform::from_scale(scale, scale).post_translate(
        x as f32 + (width as f32 - drawn_width) / 2.0,
        y as f32 + (height as f32 - drawn_height) / 2.0,
    );

    target.draw_pixmap(
        0,
        0,
        source.as_ref(),
        &PixmapPaint::default(),
        transform,
        None,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_has_the_expected_canvas_size() {
        let frame = render(Language::PtBr);

        assert_eq!((frame.width(), frame.height()), (WIDTH, HEIGHT));
    }

    #[test]
    fn languages_produce_different_stamps() {
        assert_ne!(render(Language::PtBr).data(), render(Language::En).data());
    }

    #[test]
    fn frame_can_be_written() {
        let path = std::env::temp_dir().join(format!("velha-capture-{}.png", std::process::id()));

        write(&path, Language::PtBr).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        let _ = std::fs::remove_file(&path);

        assert!(bytes.starts_with(b"\x89PNG"));
    }
}
