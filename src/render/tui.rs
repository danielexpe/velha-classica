use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;
use crate::render::layout::{breakpoint, game_area, Breakpoint, MIN_COLS, MIN_ROWS};
use crate::theme::Token;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let background = Style::default().bg(app.color(Token::SurfaceWoodDeep));

    frame.render_widget(Block::default().style(background), area);

    let Some(game) = game_area(area) else {
        draw_too_small(frame, app, area);
        return;
    };

    match breakpoint(game) {
        Breakpoint::Wide => draw_wide(frame, app, game),
        Breakpoint::Compact => draw_compact(frame, app, game),
        Breakpoint::Minimal => draw_minimal(frame, app, game),
    }
}

fn draw_too_small(frame: &mut Frame, app: &App, area: Rect) {
    let strings = app.strings();
    let lines = vec![
        Line::from(Span::styled(
            strings.terminal_too_small(MIN_COLS, MIN_ROWS),
            Style::default()
                .fg(app.color(Token::Danger))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            strings.terminal_resize_hint,
            Style::default().fg(app.color(Token::TextOnWoodMuted)),
        )),
    ];
    let vertical = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(2),
        Constraint::Fill(1),
    ])
    .split(area);

    frame.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        vertical[1],
    );
}

fn draw_wide(frame: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::vertical([
        Constraint::Length(5),
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(area);

    draw_header_wide(frame, app, rows[0]);
    draw_divider(frame, app, rows[1]);

    let body =
        Layout::horizontal([Constraint::Percentage(58), Constraint::Percentage(42)]).split(rows[2]);

    draw_board(frame, app, body[0], 31, 17);
    draw_side_panel(frame, app, body[1]);
    draw_footer(frame, app, rows[3]);
}

fn draw_compact(frame: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
        Constraint::Length(2),
        Constraint::Length(1),
    ])
    .split(area);

    draw_header_compact(frame, app, rows[0]);
    draw_board(frame, app, rows[1], 27, 13);
    draw_status(frame, app, rows[2]);
    draw_footer(frame, app, rows[3]);
}

fn draw_minimal(frame: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(area);

    draw_header_minimal(frame, app, rows[0]);
    draw_board(frame, app, rows[1], 23, 11);
    draw_minimal_status(frame, app, rows[2]);
    draw_footer(frame, app, rows[3]);
}

fn draw_header_wide(frame: &mut Frame, app: &App, area: Rect) {
    let strings = app.strings();
    let lines = vec![
        Line::from(Span::styled(
            title_text(strings.app_title),
            Style::default()
                .fg(app.color(Token::BrassBright))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("{} · {}", strings.app_subtitle, strings.app_tagline),
            Style::default().fg(app.color(Token::TextOnWoodMuted)),
        )),
        Line::from(Span::styled(
            format!(
                "{} · {}",
                strings.language_status(app.language()),
                strings.sound_status(app.sound_enabled())
            ),
            Style::default().fg(app.color(Token::TextOnWoodMuted)),
        )),
        Line::from(Span::styled(
            strings.placeholder_title,
            Style::default().fg(app.color(Token::Brass)),
        )),
        Line::from(Span::styled(
            strings.placeholder_body,
            Style::default().fg(app.color(Token::TextOnWood)),
        )),
    ];

    frame.render_widget(Paragraph::new(lines), area);
}

fn draw_header_compact(frame: &mut Frame, app: &App, area: Rect) {
    let strings = app.strings();
    let lines = vec![
        Line::from(Span::styled(
            strings.app_title.to_uppercase(),
            Style::default()
                .fg(app.color(Token::BrassBright))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("{} · {}", strings.app_subtitle, strings.app_tagline),
            Style::default().fg(app.color(Token::TextOnWoodMuted)),
        )),
    ];

    frame.render_widget(Paragraph::new(lines), area);
}

fn draw_header_minimal(frame: &mut Frame, app: &App, area: Rect) {
    let title = app.strings().app_title.to_uppercase();
    let style = Style::default()
        .fg(app.color(Token::BrassBright))
        .add_modifier(Modifier::BOLD);

    frame.render_widget(Paragraph::new(title).style(style), area);
}

fn draw_divider(frame: &mut Frame, app: &App, area: Rect) {
    let divider = "─".repeat(usize::from(area.width));

    frame.render_widget(
        Paragraph::new(divider).style(Style::default().fg(app.color(Token::Brass))),
        area,
    );
}

fn draw_board(frame: &mut Frame, app: &App, area: Rect, panel_width: u16, panel_height: u16) {
    let panel = centered_rect(area, panel_width, panel_height);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.color(Token::SurfacePaperShadow)))
        .style(Style::default().bg(app.color(Token::SurfacePaper)));
    let inner = block.inner(panel);

    frame.render_widget(block, panel);

    let cell_width = (inner.width.saturating_sub(4) / 3).clamp(3, 5) as usize;
    let cell_height = if inner.height >= 10 { 2 } else { 1 };
    let grid = grid_lines(cell_width, cell_height);
    let grid_height = grid.len() as u16;
    let grid_width = grid
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0) as u16;
    let grid_area = centered_rect(inner, grid_width.min(inner.width), grid_height);

    let lines: Vec<Line> = grid
        .into_iter()
        .map(|text| {
            Line::from(Span::styled(
                text,
                Style::default().fg(app.color(Token::TextOnPaperMuted)),
            ))
        })
        .collect();

    frame.render_widget(
        Paragraph::new(lines).alignment(Alignment::Center),
        grid_area,
    );
}

fn draw_side_panel(frame: &mut Frame, app: &App, area: Rect) {
    let strings = app.strings();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.color(Token::SurfacePaperShadow)))
        .style(Style::default().bg(app.color(Token::SurfacePaper)))
        .title(Line::from(Span::styled(
            format!(" {} ", strings.placeholder_title),
            Style::default()
                .fg(app.color(Token::TextOnPaper))
                .add_modifier(Modifier::BOLD),
        )));
    let inner = block.inner(area);

    frame.render_widget(block, area);

    let lines = vec![
        Line::from(Span::styled(
            strings.placeholder_body,
            Style::default().fg(app.color(Token::TextOnPaperMuted)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            strings.language_status(app.language()),
            Style::default().fg(app.color(Token::TextOnPaper)),
        )),
        Line::from(Span::styled(
            strings.sound_status(app.sound_enabled()),
            Style::default().fg(app.color(Token::TextOnPaper)),
        )),
    ];

    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), inner);
}

fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let strings = app.strings();
    let lines = vec![
        Line::from(Span::styled(
            format!(
                "{} · {}",
                strings.placeholder_title, strings.placeholder_body
            ),
            Style::default().fg(app.color(Token::TextOnWood)),
        )),
        Line::from(Span::styled(
            format!(
                "{} · {}",
                strings.language_status(app.language()),
                strings.sound_status(app.sound_enabled())
            ),
            Style::default().fg(app.color(Token::TextOnWoodMuted)),
        )),
    ];

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), area);
}

fn draw_minimal_status(frame: &mut Frame, app: &App, area: Rect) {
    let strings = app.strings();
    let status = format!(
        "{} · {}",
        strings.language_status(app.language()),
        strings.placeholder_title
    );

    frame.render_widget(
        Paragraph::new(status)
            .alignment(Alignment::Center)
            .style(Style::default().fg(app.color(Token::TextOnWoodMuted))),
        area,
    );
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let strings = app.strings();
    let hints = format!(
        "{}  ·  {}  ·  {}",
        strings.footer_language, strings.footer_sound, strings.footer_quit
    );
    let version = strings.version_text(app.version());
    let columns = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(version.chars().count() as u16),
    ])
    .split(area);
    let style = Style::default().fg(app.color(Token::TextOnWoodMuted));

    frame.render_widget(Paragraph::new(hints).style(style), columns[0]);
    frame.render_widget(
        Paragraph::new(version)
            .alignment(Alignment::Right)
            .style(style),
        columns[1],
    );
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);

    Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    )
}

fn title_text(title: &str) -> String {
    let mut spaced = String::new();

    for character in title.to_uppercase().chars() {
        spaced.push(character);
        spaced.push(' ');
    }

    format!("✦  {} ✦", spaced.trim_end())
}

fn grid_lines(cell_width: usize, cell_height: usize) -> Vec<String> {
    let horizontal = "─".repeat(cell_width);
    let empty = " ".repeat(cell_width);
    let top = format!("┌{horizontal}┬{horizontal}┬{horizontal}┐");
    let middle = format!("├{horizontal}┼{horizontal}┼{horizontal}┤");
    let bottom = format!("└{horizontal}┴{horizontal}┴{horizontal}┘");
    let row = format!("│{empty}│{empty}│{empty}│");
    let mut lines = Vec::with_capacity(cell_height * 3 + 4);

    lines.push(top);

    for section in 0..3 {
        for _ in 0..cell_height {
            lines.push(row.clone());
        }

        if section < 2 {
            lines.push(middle.clone());
        }
    }

    lines.push(bottom);
    lines
}

#[cfg(test)]
mod tests {
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    use super::*;
    use crate::i18n::Language;
    use crate::theme::ColorMode;

    fn render(language: Language, cols: u16, rows: u16) -> String {
        let backend = TestBackend::new(cols, rows);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = App::new(language, ColorMode::TrueColor);

        terminal.draw(|frame| draw(frame, &app)).unwrap();

        let buffer = terminal.backend().buffer();
        let width = usize::from(buffer.area.width);
        let mut text = String::new();

        for row in buffer.content().chunks(width) {
            for cell in row {
                text.push_str(cell.symbol());
            }
            text.push('\n');
        }

        text
    }

    #[test]
    fn grid_has_square_cells_and_uniform_width() {
        let lines = grid_lines(5, 2);
        let width = lines[0].chars().count();

        assert_eq!(lines.len(), 10);
        assert!(lines.iter().all(|line| line.chars().count() == width));
    }

    #[test]
    fn title_uses_brand_and_ornaments() {
        assert_eq!(
            title_text("Velha Clássica"),
            "✦  V E L H A   C L Á S S I C A ✦"
        );
    }

    #[test]
    fn renders_placeholder_in_both_languages() {
        let portuguese = render(Language::PtBr, 192, 54);
        let english = render(Language::En, 192, 54);

        assert!(portuguese.contains("Fundação pronta"));
        assert!(portuguese.contains("Idioma: Português"));
        assert!(english.contains("Foundation ready"));
        assert!(english.contains("Language: English"));
    }

    #[test]
    fn renders_minimum_size_warning() {
        let portuguese = render(Language::PtBr, 79, 23);
        let english = render(Language::En, 79, 23);

        assert!(portuguese.contains("Terminal muito pequeno. Mínimo: 80×24."));
        assert!(english.contains("Terminal too small. Minimum: 80×24."));
    }
}
