use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, PixmapPaint, Rect, Stroke, Transform};

use crate::assets;
use crate::core::{Board, Mark, BOARD_CELLS, BOARD_SIDE};

pub const SIZE: u32 = 640;
const PADDING: f32 = 84.0;
const GRID: f32 = SIZE as f32 - PADDING * 2.0;
const CELL: f32 = GRID / BOARD_SIDE as f32;
const MARK_SIZE: f32 = CELL * 0.74;
const CURSOR_SIZE: f32 = CELL * 0.90;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stamp {
    WinPt,
    WinEn,
    DrawPt,
    DrawEn,
}

#[derive(Debug, Clone, Copy)]
pub struct SceneInput {
    pub selected: usize,
    pub turn: Mark,
    pub show_cursor: bool,
    pub cursor_opacity: f32,
    pub winning_line: Option<[usize; 3]>,
    pub win_progress: f32,
    pub last_move: Option<usize>,
    pub mark_progress: f32,
    pub stamp: Option<Stamp>,
}

impl SceneInput {
    pub const fn new(selected: usize, turn: Mark, show_cursor: bool) -> Self {
        Self {
            selected,
            turn,
            show_cursor,
            cursor_opacity: 1.0,
            winning_line: None,
            win_progress: 1.0,
            last_move: None,
            mark_progress: 1.0,
            stamp: None,
        }
    }
}

pub fn render(board: &Board, input: SceneInput) -> Pixmap {
    let mut pixmap = Pixmap::new(SIZE, SIZE).expect("scene dimensions are valid");
    let paper = assets::paper_board();
    let scale = SIZE as f32 / paper.width() as f32;

    pixmap.draw_pixmap(
        0,
        0,
        paper.as_ref(),
        &PixmapPaint::default(),
        Transform::from_scale(scale, scale),
        None,
    );

    draw_grid(&mut pixmap);
    draw_winning_line(&mut pixmap, input.winning_line, input.win_progress);
    draw_marks(&mut pixmap, board, input);
    draw_stamp(&mut pixmap, input);

    if input.show_cursor && board.get(input.selected).is_none() {
        draw_ghost(&mut pixmap, input);
        draw_cursor(&mut pixmap, input);
    }

    pixmap
}

pub fn signature(board: &Board, input: SceneInput) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;

    for cell in 0..BOARD_CELLS {
        let value = match board.get(cell) {
            None => 0u64,
            Some(Mark::X) => 1,
            Some(Mark::O) => 2,
        };
        hash = (hash ^ value).wrapping_mul(0x0000_0100_0000_01b3);
    }

    hash = (hash ^ input.selected as u64).wrapping_mul(0x0000_0100_0000_01b3);
    hash = (hash ^ u64::from(input.turn == Mark::O)).wrapping_mul(0x0000_0100_0000_01b3);
    hash = (hash ^ u64::from(input.show_cursor)).wrapping_mul(0x0000_0100_0000_01b3);
    hash = (hash ^ (input.cursor_opacity * 60.0) as u64).wrapping_mul(0x0000_0100_0000_01b3);
    hash = (hash ^ (input.win_progress * 60.0) as u64).wrapping_mul(0x0000_0100_0000_01b3);
    hash = (hash ^ (input.mark_progress * 60.0) as u64).wrapping_mul(0x0000_0100_0000_01b3);
    hash =
        (hash ^ input.last_move.unwrap_or(usize::MAX) as u64).wrapping_mul(0x0000_0100_0000_01b3);

    if let Some(line) = input.winning_line {
        for cell in line {
            hash = (hash ^ cell as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    hash
}

pub fn to_dynamic_image(pixmap: &Pixmap) -> image::DynamicImage {
    let buffer =
        image::RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.data().to_vec())
            .expect("pixmap buffer has the right size");

    image::DynamicImage::ImageRgba8(buffer)
}

fn draw_grid(pixmap: &mut Pixmap) {
    let mut paint = Paint::default();
    paint.set_color_rgba8(0x6E, 0x5A, 0x43, 150);
    paint.anti_alias = true;

    let stroke = Stroke {
        width: 2.6,
        ..Stroke::default()
    };

    for index in 1..BOARD_SIDE {
        let offset = PADDING + CELL * index as f32;
        let mut builder = PathBuilder::new();
        builder.move_to(offset, PADDING + 6.0);
        builder.line_to(offset, PADDING + GRID - 6.0);
        if let Some(path) = builder.finish() {
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }

        let mut builder = PathBuilder::new();
        builder.move_to(PADDING + 6.0, offset);
        builder.line_to(PADDING + GRID - 6.0, offset);
        if let Some(path) = builder.finish() {
            pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
        }
    }
}

fn draw_marks(pixmap: &mut Pixmap, board: &Board, input: SceneInput) {
    let x = assets::mark_x();
    let o = assets::mark_o();
    let progress = input.mark_progress.clamp(0.0, 1.0);

    for cell in 0..BOARD_CELLS {
        let source = match board.get(cell) {
            Some(Mark::X) => Some(x),
            Some(Mark::O) => Some(o),
            None => None,
        };

        if let Some(source) = source {
            let scale = if input.last_move == Some(cell) {
                1.0 + 0.18 * (1.0 - progress)
            } else {
                1.0
            };

            draw_centered(pixmap, source, cell, MARK_SIZE * scale, 1.0);
        }
    }
}

fn draw_ghost(pixmap: &mut Pixmap, input: SceneInput) {
    let source = match input.turn {
        Mark::X => assets::mark_x_ghost(),
        Mark::O => assets::mark_o_ghost(),
    };

    draw_centered(pixmap, source, input.selected, MARK_SIZE, 1.0);
}

fn draw_stamp(pixmap: &mut Pixmap, input: SceneInput) {
    let Some(stamp) = input.stamp else {
        return;
    };
    let source = match stamp {
        Stamp::WinPt => assets::stamp_win_pt(),
        Stamp::WinEn => assets::stamp_win_en(),
        Stamp::DrawPt => assets::stamp_draw_pt(),
        Stamp::DrawEn => assets::stamp_draw_en(),
    };
    let opacity = input.win_progress.clamp(0.0, 1.0);
    let width = 300.0;
    let height = width * source.height() as f32 / source.width() as f32;
    let scale = width / source.width() as f32;
    let x = (SIZE as f32 - width) / 2.0;
    let y = SIZE as f32 - height - 8.0;
    draw_stamp_backing(
        pixmap,
        x - 8.0,
        y - 8.0,
        width + 16.0,
        height + 16.0,
        opacity,
    );
    let transform = Transform::from_scale(scale, scale).post_translate(x, y);

    pixmap.draw_pixmap(
        0,
        0,
        source.as_ref(),
        &PixmapPaint {
            opacity,
            ..PixmapPaint::default()
        },
        transform,
        None,
    );
}

fn draw_stamp_backing(pixmap: &mut Pixmap, x: f32, y: f32, width: f32, height: f32, opacity: f32) {
    let Some(rect) = Rect::from_xywh(x, y, width, height) else {
        return;
    };
    let mut builder = PathBuilder::new();
    builder.push_rect(rect);

    let Some(path) = builder.finish() else {
        return;
    };

    let mut fill = Paint::default();
    fill.set_color_rgba8(0xF7, 0xE9, 0xCD, 225);
    fill.anti_alias = true;
    pixmap.fill_path(&path, &fill, FillRule::Winding, Transform::identity(), None);
    let mut stroke = Paint::default();
    stroke.set_color_rgba8(0x8E, 0x3B, 0x2F, (140.0 * opacity) as u8);
    stroke.anti_alias = true;
    pixmap.stroke_path(
        &path,
        &stroke,
        &Stroke {
            width: 2.0,
            ..Stroke::default()
        },
        Transform::identity(),
        None,
    );
}

fn draw_cursor(pixmap: &mut Pixmap, input: SceneInput) {
    let source = assets::cursor_brackets();

    draw_centered(
        pixmap,
        source,
        input.selected,
        CURSOR_SIZE,
        input.cursor_opacity.clamp(0.0, 1.0),
    );
}

fn draw_winning_line(pixmap: &mut Pixmap, line: Option<[usize; 3]>, progress: f32) {
    let Some(line) = line else {
        return;
    };

    let progress = progress.clamp(0.0, 1.0);

    if progress <= 0.0 {
        return;
    }

    let start = cell_center(line[0]);
    let end = cell_center(line[2]);
    let end = (
        start.0 + (end.0 - start.0) * progress,
        start.1 + (end.1 - start.1) * progress,
    );
    let mut builder = PathBuilder::new();
    builder.move_to(start.0, start.1);
    builder.line_to(end.0, end.1);

    let Some(path) = builder.finish() else {
        return;
    };

    let mut paint = Paint::default();
    paint.set_color_rgba8(0xE6, 0xC7, 0x66, 210);
    paint.anti_alias = true;
    let stroke = Stroke {
        width: 7.0,
        ..Stroke::default()
    };

    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    draw_winning_caps(pixmap, start);
    draw_winning_caps(pixmap, end);
}

fn draw_winning_caps(pixmap: &mut Pixmap, center: (f32, f32)) {
    let mut paint = Paint::default();
    paint.set_color_rgba8(0xE6, 0xC7, 0x66, 230);
    paint.anti_alias = true;

    if let Some(rect) = Rect::from_xywh(center.0 - 6.0, center.1 - 6.0, 12.0, 12.0) {
        let mut builder = PathBuilder::new();
        builder.push_rect(rect);
        if let Some(path) = builder.finish() {
            pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
    }
}

fn draw_centered(target: &mut Pixmap, source: &Pixmap, cell: usize, size: f32, opacity: f32) {
    let (cx, cy) = cell_center(cell);
    let scale = size / source.width() as f32;
    let transform =
        Transform::from_scale(scale, scale).post_translate(cx - size / 2.0, cy - size / 2.0);
    let paint = PixmapPaint {
        opacity,
        ..PixmapPaint::default()
    };

    target.draw_pixmap(0, 0, source.as_ref(), &paint, transform, None);
}

fn cell_center(cell: usize) -> (f32, f32) {
    let (row, column) = (cell / BOARD_SIDE, cell % BOARD_SIDE);
    (
        PADDING + CELL * (column as f32 + 0.5),
        PADDING + CELL * (row as f32 + 0.5),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board_renders_at_the_expected_size() {
        let scene = render(&Board::new(), SceneInput::new(4, Mark::X, true));

        assert_eq!((scene.width(), scene.height()), (SIZE, SIZE));
    }

    #[test]
    fn marks_change_the_scene() {
        let mut board = Board::new();
        let empty = render(&board, SceneInput::new(4, Mark::X, false));
        board.place(0, Mark::X).unwrap();
        let marked = render(&board, SceneInput::new(4, Mark::O, false));

        assert_ne!(empty.data(), marked.data());
    }

    #[test]
    fn winning_line_is_drawn_brass() {
        let mut board = Board::new();
        for cell in [0, 1, 2, 3, 4] {
            let mark = if cell < 3 { Mark::X } else { Mark::O };
            board.place(cell, mark).unwrap();
        }

        let mut input = SceneInput::new(8, Mark::O, false);
        input.winning_line = crate::core::outcome(&board).winning_line();
        let scene = render(&board, input);

        let brass = scene
            .data()
            .as_chunks::<4>()
            .0
            .iter()
            .any(|pixel| pixel[0] > 150 && pixel[1] > 120 && pixel[2] < 130 && pixel[3] > 150);

        assert!(brass, "winning line was not drawn in brass");
    }

    #[test]
    fn stamp_overlay_only_appears_on_results() {
        let mut board = Board::new();
        for cell in [0, 1, 2] {
            board.place(cell, Mark::X).unwrap();
        }
        let mut input = SceneInput::new(8, Mark::O, false);
        input.winning_line = crate::core::outcome(&board).winning_line();
        let plain = render(&board, input);
        input.stamp = Some(Stamp::WinPt);
        let stamped = render(&board, input);

        assert_ne!(plain.data(), stamped.data());
    }

    #[test]
    fn signature_tracks_board_cursor_and_animation() {
        let board = Board::new();
        let first = SceneInput::new(4, Mark::X, true);
        let second = SceneInput::new(5, Mark::X, true);
        let mut animated = first;
        animated.mark_progress = 0.5;

        assert_eq!(signature(&board, first), signature(&board, first));
        assert_ne!(signature(&board, first), signature(&board, second));
        assert_ne!(signature(&board, first), signature(&board, animated));
    }

    #[test]
    fn converted_image_matches_the_pixmap() {
        let scene = render(&Board::new(), SceneInput::new(4, Mark::X, false));
        let image = to_dynamic_image(&scene);

        assert_eq!(image.width(), SIZE);
        assert_eq!(image.height(), SIZE);
    }
}
