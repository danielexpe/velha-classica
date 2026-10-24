use ratatui::layout::Rect;

pub const MIN_COLS: u16 = 80;
pub const MIN_ROWS: u16 = 24;
pub const MAX_COLS: u16 = 140;
pub const MAX_ROWS: u16 = 45;
pub const GAME_RATIO: f64 = 0.70;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Breakpoint {
    Minimal,
    Compact,
    Wide,
}

pub fn game_area(term: Rect) -> Option<Rect> {
    if term.width < MIN_COLS || term.height < MIN_ROWS {
        return None;
    }

    let width = scaled(term.width, MIN_COLS, MAX_COLS).min(term.width);
    let height = scaled(term.height, MIN_ROWS, MAX_ROWS).min(term.height);

    Some(Rect::new(
        term.x + (term.width - width) / 2,
        term.y + (term.height - height) / 2,
        width,
        height,
    ))
}

pub fn breakpoint(area: Rect) -> Breakpoint {
    if area.width >= 120 && area.height >= 34 {
        Breakpoint::Wide
    } else if area.width >= 96 && area.height >= 28 {
        Breakpoint::Compact
    } else {
        Breakpoint::Minimal
    }
}

fn scaled(value: u16, min: u16, max: u16) -> u16 {
    let raw = ((u32::from(value) * 7 + 5) / 10) as u16;

    raw.clamp(min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_uses_70_percent_with_clamps() {
        let cases = [
            (192, 54, 134, 38, Breakpoint::Wide),
            (160, 45, 112, 32, Breakpoint::Compact),
            (220, 60, 140, 42, Breakpoint::Wide),
            (120, 34, 84, 24, Breakpoint::Minimal),
            (100, 32, 80, 24, Breakpoint::Minimal),
            (96, 28, 80, 24, Breakpoint::Minimal),
            (80, 24, 80, 24, Breakpoint::Minimal),
        ];

        for (cols, rows, expected_cols, expected_rows, expected_breakpoint) in cases {
            let area = game_area(Rect::new(0, 0, cols, rows)).unwrap();

            assert_eq!(area.width, expected_cols, "terminal {cols}x{rows}");
            assert_eq!(area.height, expected_rows, "terminal {cols}x{rows}");
            assert_eq!(breakpoint(area), expected_breakpoint);
        }
    }

    #[test]
    fn layout_centers_content() {
        let area = game_area(Rect::new(5, 5, 100, 30)).unwrap();

        assert_eq!(area, Rect::new(15, 8, 80, 24));
    }

    #[test]
    fn minimum_size_warning_below_80x24() {
        assert_eq!(game_area(Rect::new(0, 0, 79, 23)), None);
        assert_eq!(game_area(Rect::new(0, 0, 80, 23)), None);
        assert_eq!(game_area(Rect::new(0, 0, 79, 24)), None);
        assert!(game_area(Rect::new(0, 0, 80, 24)).is_some());
    }

    #[test]
    fn area_never_exceeds_terminal() {
        for (cols, rows) in [(80, 24), (100, 32), (192, 54), (300, 100)] {
            let term = Rect::new(0, 0, cols, rows);
            let area = game_area(term).unwrap();

            assert!(area.width <= term.width);
            assert!(area.height <= term.height);
        }
    }
}
