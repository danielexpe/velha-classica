use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::app::App;
use crate::core::{Difficulty, Mark, Outcome, BOARD_CELLS, BOARD_SIDE};
use crate::render::layout::{breakpoint, game_area, Breakpoint, MIN_COLS, MIN_ROWS};
use crate::render::scene::{self, SceneInput, Stamp};
use crate::state::{Game, Mode, Screen};
use crate::theme::Token;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let background = Style::default().bg(app.color(Token::SurfaceWoodDeep));

    frame.render_widget(Block::default().style(background), area);

    let Some(game_area) = game_area(area) else {
        draw_too_small(frame, app, area);
        return;
    };

    match app.screen() {
        Screen::Home => draw_home(frame, app, game_area),
        Screen::Difficulty => draw_difficulty(frame, app, game_area),
        Screen::Playing | Screen::Paused | Screen::RoundOver => {
            draw_game(frame, app, game_area);

            if app.screen() == Screen::Paused {
                draw_pause(frame, app, game_area);
            }
        }
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

    frame.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        centered_rect(area, area.width, 2),
    );
}

fn draw_home(frame: &mut Frame, app: &App, area: Rect) {
    let strings = app.strings();
    let breakpoint = breakpoint(area);
    let rows = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(area);
    let mut lines = vec![
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
        Line::from(""),
    ];

    if breakpoint != Breakpoint::Minimal {
        lines.push(divider_line(app, 37));
        lines.push(Line::from(""));
    }

    let items = [
        strings.menu_one_player,
        strings.menu_two_players,
        strings.menu_quit,
    ];

    for (index, label) in items.into_iter().enumerate() {
        lines.push(menu_line(app, label, index == app.home_selection()));
    }

    draw_centered_lines(frame, rows[0], lines);
    draw_footer(frame, app, rows[1], Footer::Home);
}

fn draw_difficulty(frame: &mut Frame, app: &App, area: Rect) {
    let strings = app.strings();
    let breakpoint = breakpoint(area);
    let rows = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(area);
    let mut lines = vec![
        Line::from(Span::styled(
            title_text(strings.app_title),
            Style::default()
                .fg(app.color(Token::BrassBright))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            strings.menu_choose_difficulty,
            Style::default()
                .fg(app.color(Token::Brass))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            strings.difficulty_human_starts,
            Style::default().fg(app.color(Token::TextOnWoodMuted)),
        )),
        Line::from(""),
    ];

    for (index, difficulty) in Difficulty::ALL.into_iter().enumerate() {
        let focused = index == app.difficulty_selection();
        lines.push(menu_line(app, strings.difficulty_name(difficulty), focused));

        if breakpoint != Breakpoint::Minimal {
            lines.push(Line::from(Span::styled(
                format!("    {}", strings.difficulty_description(difficulty)),
                Style::default().fg(app.color(Token::TextOnWoodMuted)),
            )));
        }
    }

    lines.push(menu_line(
        app,
        strings.menu_back,
        app.difficulty_selection() == Difficulty::ALL.len(),
    ));

    if let Some(status) = app.status_text() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            status,
            Style::default()
                .fg(app.color(Token::Danger))
                .add_modifier(Modifier::BOLD),
        )));
    }

    draw_centered_lines(frame, rows[0], lines);
    draw_footer(frame, app, rows[1], Footer::Difficulty);
}

fn draw_game(frame: &mut Frame, app: &App, area: Rect) {
    let Some(game) = app.game() else {
        draw_home(frame, app, area);
        return;
    };

    match breakpoint(area) {
        Breakpoint::Wide => draw_game_wide(frame, app, game, area),
        Breakpoint::Compact => draw_game_compact(frame, app, game, area),
        Breakpoint::Minimal => draw_game_minimal(frame, app, game, area),
    }
}

fn draw_game_wide(frame: &mut Frame, app: &App, game: &Game, area: Rect) {
    let rows = Layout::vertical([
        Constraint::Length(4),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(area);

    draw_game_header_wide(frame, app, game, rows[0]);

    let body =
        Layout::horizontal([Constraint::Percentage(58), Constraint::Percentage(42)]).split(rows[1]);

    draw_board(frame, app, game, body[0], Breakpoint::Wide);
    draw_side_panel(frame, app, game, body[1]);
    draw_footer(frame, app, rows[2], footer_for(app.screen()));
}

fn draw_game_compact(frame: &mut Frame, app: &App, game: &Game, area: Rect) {
    let status_height = if app.screen() == Screen::RoundOver {
        6
    } else {
        2
    };
    let rows = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
        Constraint::Length(status_height),
        Constraint::Length(1),
    ])
    .split(area);

    draw_game_header_compact(frame, app, game, rows[0]);
    draw_board(frame, app, game, rows[1], Breakpoint::Compact);
    draw_compact_status(frame, app, game, rows[2]);
    draw_footer(frame, app, rows[3], footer_for(app.screen()));
}

fn draw_game_minimal(frame: &mut Frame, app: &App, game: &Game, area: Rect) {
    let status_height = if app.screen() == Screen::RoundOver {
        5
    } else {
        1
    };
    let rows = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(status_height),
        Constraint::Length(1),
    ])
    .split(area);

    draw_game_header_minimal(frame, app, game, rows[0]);
    draw_board(frame, app, game, rows[1], Breakpoint::Minimal);
    draw_minimal_status(frame, app, game, rows[2]);
    draw_footer(frame, app, rows[3], footer_for(app.screen()));
}

fn draw_game_header_wide(frame: &mut Frame, app: &App, game: &Game, area: Rect) {
    let rows = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(area);

    draw_title_row(frame, app, rows[0], true);
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!(
                "{} · {}",
                app.strings().app_subtitle,
                app.strings().app_tagline
            ),
            Style::default().fg(app.color(Token::TextOnWoodMuted)),
        ))),
        rows[1],
    );
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!("{} · {}", mode_text(app, game), round_text(app, game)),
            Style::default().fg(app.color(Token::TextOnWood)),
        ))),
        rows[2],
    );
    draw_divider(frame, app, rows[3]);
}

fn draw_game_header_compact(frame: &mut Frame, app: &App, game: &Game, area: Rect) {
    let rows = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).split(area);

    draw_title_row(frame, app, rows[0], true);
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!(
                "{} · {} · {} · {}",
                app.strings().app_subtitle,
                mode_text(app, game),
                round_text(app, game),
                app.strings().sound_status(app.sound_enabled())
            ),
            Style::default().fg(app.color(Token::TextOnWoodMuted)),
        ))),
        rows[1],
    );
}

fn draw_game_header_minimal(frame: &mut Frame, app: &App, game: &Game, area: Rect) {
    let indicators = format!(
        "{} · {} · {}",
        mode_text(app, game),
        app.strings().sound_status(app.sound_enabled()),
        app.strings().version_text(app.version())
    );
    let columns = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(indicators.chars().count() as u16),
    ])
    .split(area);

    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            app.strings().app_title.to_uppercase(),
            Style::default()
                .fg(app.color(Token::BrassBright))
                .add_modifier(Modifier::BOLD),
        ))),
        columns[0],
    );
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            indicators,
            Style::default().fg(app.color(Token::TextOnWoodMuted)),
        )))
        .alignment(Alignment::Right),
        columns[1],
    );
}

fn draw_title_row(frame: &mut Frame, app: &App, area: Rect, include_version: bool) {
    let indicators = indicators(app, include_version);
    let columns = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(indicators.chars().count() as u16),
    ])
    .split(area);

    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            title_text(app.strings().app_title),
            Style::default()
                .fg(app.color(Token::BrassBright))
                .add_modifier(Modifier::BOLD),
        ))),
        columns[0],
    );
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(
            indicators,
            Style::default().fg(app.color(Token::TextOnWoodMuted)),
        )))
        .alignment(Alignment::Right),
        columns[1],
    );
}

fn draw_side_panel(frame: &mut Frame, app: &App, game: &Game, area: Rect) {
    let title = mode_text(app, game);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.color(Token::SurfacePaperShadow)))
        .style(Style::default().bg(app.color(Token::SurfacePaper)))
        .title(Line::from(Span::styled(
            format!(" {title} "),
            Style::default()
                .fg(app.color(Token::TextOnPaper))
                .add_modifier(Modifier::BOLD),
        )));
    let inner = block.inner(area);

    frame.render_widget(block, area);

    let lines = if app.screen() == Screen::RoundOver {
        result_panel_lines(app, game)
    } else {
        play_panel_lines(app, game)
    };

    frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: true }), inner);
}

fn play_panel_lines(app: &App, game: &Game) -> Vec<Line<'static>> {
    let strings = app.strings();
    let mut lines = vec![
        Line::from(Span::styled(
            turn_label(app, game).to_uppercase(),
            Style::default()
                .fg(app.color(Token::BrassBright))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!(
                "{} · {}",
                player_label(app, game.turn()),
                game.turn().glyph()
            ),
            Style::default().fg(app.color(Token::TextOnPaper)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            strings.game_session_score.to_uppercase(),
            Style::default().fg(app.color(Token::TextOnPaperMuted)),
        )),
    ];

    lines.extend(score_lines(app, game));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "─".repeat(27),
        Style::default().fg(app.color(Token::Brass)),
    )));

    if let Some(status) = app.status_text() {
        lines.push(Line::from(Span::styled(
            status,
            Style::default()
                .fg(app.color(Token::Danger))
                .add_modifier(Modifier::BOLD),
        )));
    }

    lines.extend([
        Line::from(Span::styled(
            strings.footer_move.to_string(),
            Style::default().fg(app.color(Token::TextOnPaperMuted)),
        )),
        Line::from(Span::styled(
            strings.footer_select_square.to_string(),
            Style::default().fg(app.color(Token::TextOnPaperMuted)),
        )),
        Line::from(Span::styled(
            strings.footer_pause.to_string(),
            Style::default().fg(app.color(Token::TextOnPaperMuted)),
        )),
    ]);

    lines
}

fn result_panel_lines(app: &App, game: &Game) -> Vec<Line<'static>> {
    let strings = app.strings();
    let stamp = result_stamp(app, game).unwrap_or(strings.result_stamp_draw);
    let mut lines = vec![
        Line::from(Span::styled(
            format!("✦ {stamp} ✦"),
            Style::default()
                .fg(app.color(Token::BrassBright))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            result_text(app, game).to_string(),
            Style::default()
                .fg(app.color(Token::TextOnPaper))
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            strings.game_session_score.to_uppercase(),
            Style::default().fg(app.color(Token::TextOnPaperMuted)),
        )),
    ];

    lines.extend(score_lines(app, game));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "─".repeat(27),
        Style::default().fg(app.color(Token::Brass)),
    )));

    for (index, label) in result_actions(app).into_iter().enumerate() {
        lines.push(menu_line_paper(app, label, index == app.result_selection()));
    }

    lines
}

fn draw_compact_status(frame: &mut Frame, app: &App, game: &Game, area: Rect) {
    let strings = app.strings();
    let lines = if app.screen() == Screen::RoundOver {
        let stamp = result_stamp(app, game).unwrap_or(strings.result_stamp_draw);
        let mut lines = vec![
            Line::from(Span::styled(
                format!("✦ {stamp} ✦ · {}", result_text(app, game)),
                Style::default()
                    .fg(app.color(Token::BrassBright))
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                score_inline(app, game),
                Style::default().fg(app.color(Token::TextOnWood)),
            )),
            Line::from(""),
        ];

        for (index, label) in result_actions(app).into_iter().enumerate() {
            lines.push(menu_line(app, label, index == app.result_selection()));
        }

        lines
    } else {
        let mut first = Line::from(Span::styled(
            turn_label(app, game),
            Style::default()
                .fg(app.color(Token::BrassBright))
                .add_modifier(Modifier::BOLD),
        ));

        if let Some(status) = app.status_text() {
            first = first.style(Style::default().fg(app.color(Token::Danger)));
            first.spans.push(Span::raw(" · "));
            first.spans.push(Span::styled(
                status,
                Style::default()
                    .fg(app.color(Token::Danger))
                    .add_modifier(Modifier::BOLD),
            ));
        }

        vec![
            first,
            Line::from(Span::styled(
                score_inline(app, game),
                Style::default().fg(app.color(Token::TextOnWood)),
            )),
        ]
    };

    frame.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        area,
    );
}

fn draw_minimal_status(frame: &mut Frame, app: &App, game: &Game, area: Rect) {
    let strings = app.strings();
    let lines = if app.screen() == Screen::RoundOver {
        let stamp = result_stamp(app, game).unwrap_or(strings.result_stamp_draw);
        let mut lines = vec![
            Line::from(Span::styled(
                format!("✦ {stamp} ✦ · {}", result_text(app, game)),
                Style::default()
                    .fg(app.color(Token::BrassBright))
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                score_inline(app, game),
                Style::default().fg(app.color(Token::TextOnWood)),
            )),
        ];

        for (index, label) in result_actions(app).into_iter().enumerate() {
            lines.push(menu_line(app, label, index == app.result_selection()));
        }

        lines
    } else {
        let text = if let Some(status) = app.status_text() {
            status.to_string()
        } else {
            format!("{} · {}", turn_label(app, game), score_inline(app, game))
        };
        let color = if app.status().is_some() {
            app.color(Token::Danger)
        } else {
            app.color(Token::TextOnWood)
        };

        vec![Line::from(Span::styled(text, Style::default().fg(color)))]
    };

    frame.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        area,
    );
}

fn draw_pause(frame: &mut Frame, app: &App, area: Rect) {
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default().style(Style::default().bg(app.color(Token::SurfaceWoodDeep))),
        area,
    );

    let (width, height) = match breakpoint(area) {
        Breakpoint::Wide => (38, 9),
        Breakpoint::Compact => (32, 9),
        Breakpoint::Minimal => (28, 9),
    };
    let card = centered_rect(area, width, height);

    frame.render_widget(Clear, card);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.color(Token::Brass)))
        .style(Style::default().bg(app.color(Token::SurfacePaper)))
        .title(Line::from(Span::styled(
            format!(" {} ", app.strings().pause_title.to_uppercase()),
            Style::default()
                .fg(app.color(Token::TextOnPaper))
                .add_modifier(Modifier::BOLD),
        )));
    let inner = block.inner(card);

    frame.render_widget(block, card);

    let mut lines = vec![Line::from("")];
    for (index, label) in pause_actions(app).into_iter().enumerate() {
        lines.push(menu_line_paper(app, label, index == app.pause_selection()));
    }

    frame.render_widget(Paragraph::new(lines), inner);
}

fn draw_board(frame: &mut Frame, app: &App, game: &Game, area: Rect, breakpoint: Breakpoint) {
    if draw_board_image(frame, app, game, area) {
        return;
    }

    let (cell_width, cell_height) = board_cell_size(breakpoint);
    let width = cell_width * 3 + 4;
    let height = cell_height * 3 + 4;
    let panel = if breakpoint == Breakpoint::Wide {
        Rect::new(area.x + (area.width - width) / 2, area.y, width, height)
    } else {
        centered_rect(area, width, height)
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.color(Token::SurfacePaperShadow)))
        .style(Style::default().bg(app.color(Token::SurfacePaper)));
    let inner = block.inner(panel);

    frame.render_widget(block, panel);
    frame.render_widget(
        Paragraph::new(
            grid_lines(cell_width, cell_height)
                .into_iter()
                .map(Line::from)
                .collect::<Vec<_>>(),
        )
        .style(
            Style::default()
                .fg(app.color(Token::TextOnPaperMuted))
                .bg(app.color(Token::SurfacePaper)),
        ),
        inner,
    );

    for cell in 0..BOARD_CELLS {
        let (row, column) = (cell / BOARD_SIDE, cell % BOARD_SIDE);
        let cell_area = Rect::new(
            inner.x + column as u16 * (cell_width + 1),
            inner.y + row as u16 * (cell_height + 1),
            cell_width,
            cell_height,
        );

        draw_cell(frame, app, game, cell, cell_area, cell_height);
    }
}

fn draw_board_image(frame: &mut Frame, app: &App, game: &Game, area: Rect) -> bool {
    if app.screen() == Screen::Paused || !app.has_images() {
        return false;
    }

    let mut guard = app.image_state();
    let Some(state) = guard.as_mut() else {
        return false;
    };
    let target = state.square_area(area);
    let input = SceneInput {
        selected: game.selected(),
        turn: game.turn(),
        show_cursor: !game.is_over() && app.screen() == Screen::Playing,
        cursor_opacity: 1.0,
        winning_line: game.outcome().winning_line(),
        win_progress: app.winning_progress(),
        last_move: app.last_move_cell(),
        mark_progress: app.mark_progress(),
        stamp: stamp_for(app, game),
    };
    let signature = scene::signature(game.board(), input);

    if state.signature() != Some(signature) {
        let pixmap = scene::render(game.board(), input);
        state.update(scene::to_dynamic_image(&pixmap), signature);
    }

    state.render(frame, target);

    true
}

fn stamp_for(app: &App, game: &Game) -> Option<Stamp> {
    let english = app.language() == crate::i18n::Language::En;

    match game.outcome() {
        crate::core::Outcome::Win { .. } if english => Some(Stamp::WinEn),
        crate::core::Outcome::Win { .. } => Some(Stamp::WinPt),
        crate::core::Outcome::Draw if english => Some(Stamp::DrawEn),
        crate::core::Outcome::Draw => Some(Stamp::DrawPt),
        crate::core::Outcome::InProgress => None,
    }
}

fn draw_cell(frame: &mut Frame, app: &App, game: &Game, cell: usize, area: Rect, cell_height: u16) {
    let winning = game
        .outcome()
        .winning_line()
        .is_some_and(|line| line.contains(&cell));
    let selected = !game.is_over() && game.selected() == cell;
    let flash = app.animated_cell() == Some(cell);
    let highlight = winning || selected;

    if highlight {
        frame.render_widget(
            Block::default().style(Style::default().bg(app.color(Token::SurfacePaperLight))),
            area,
        );
    }

    if selected && cell_height >= 2 && area.width >= 3 && app.cursor_opacity() >= 0.5 {
        let style = Style::default()
            .fg(app.color(Token::Brass))
            .add_modifier(Modifier::BOLD);
        let top = format!(
            "┌{}┐",
            " ".repeat(usize::from(area.width.saturating_sub(2)))
        );
        let bottom = format!(
            "└{}┘",
            " ".repeat(usize::from(area.width.saturating_sub(2)))
        );

        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(top, style))),
            Rect::new(area.x, area.y, area.width, 1),
        );
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(bottom, style))),
            Rect::new(area.x, area.y + cell_height - 1, area.width, 1),
        );
    }

    let mark = game.board().get(cell);
    let content = match mark {
        Some(mark) => Some((
            mark.glyph().to_string(),
            if winning || flash {
                app.color(Token::BrassBright)
            } else {
                mark_color(app, mark)
            },
        )),
        None if selected => Some(("·".to_string(), app.color(Token::TextOnPaperMuted))),
        None => None,
    };

    if let Some((text, color)) = content {
        let marker_area = Rect::new(area.x, area.y + cell_height / 2, area.width, 1);

        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                text,
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )))
            .alignment(Alignment::Center),
            marker_area,
        );
    }
}

fn draw_divider(frame: &mut Frame, app: &App, area: Rect) {
    let divider = "─".repeat(usize::from(area.width));

    frame.render_widget(
        Paragraph::new(divider).style(Style::default().fg(app.color(Token::Brass))),
        area,
    );
}

fn draw_centered_lines(frame: &mut Frame, area: Rect, lines: Vec<Line<'static>>) {
    let height = (lines.len() as u16).min(area.height);
    let rows = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(height),
        Constraint::Fill(1),
    ])
    .split(area);

    frame.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        rows[1],
    );
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect, footer: Footer) {
    let hints = footer_hints(app, footer);
    let version = app.strings().version_text(app.version());
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Footer {
    Home,
    Difficulty,
    Game,
    RoundOver,
}

fn footer_for(screen: Screen) -> Footer {
    match screen {
        Screen::RoundOver => Footer::RoundOver,
        _ => Footer::Game,
    }
}

fn footer_hints(app: &App, footer: Footer) -> String {
    let strings = app.strings();

    match footer {
        Footer::Home => format!(
            "{}  ·  {}  ·  {}  ·  {}  ·  {}",
            strings.language_status(app.language()),
            strings.sound_status(app.sound_enabled()),
            strings.footer_language,
            strings.footer_sound,
            strings.footer_quit
        ),
        Footer::Difficulty => format!(
            "{}  ·  {}  ·  {}",
            strings.footer_navigate, strings.footer_select_menu, strings.footer_back
        ),
        Footer::Game => format!(
            "{}  ·  {}  ·  {}",
            strings.footer_move, strings.footer_select_square, strings.footer_pause
        ),
        Footer::RoundOver => format!(
            "{}  ·  {}  ·  {}",
            strings.footer_select_menu, strings.footer_rematch, strings.footer_menu
        ),
    }
}

fn indicators(app: &App, include_version: bool) -> String {
    let strings = app.strings();
    let mut parts = vec![
        strings.language_status(app.language()),
        strings.sound_status(app.sound_enabled()).to_string(),
    ];

    if include_version {
        parts.push(strings.version_text(app.version()));
    }

    parts.join(" · ")
}

fn mode_text(app: &App, game: &Game) -> String {
    let strings = app.strings();

    match game.mode() {
        Mode::TwoPlayers => strings.game_mode_two.to_string(),
        Mode::OnePlayer(difficulty) => format!(
            "{} · {}",
            strings.game_mode_one,
            strings.difficulty_status(difficulty)
        ),
    }
}

fn round_text(app: &App, game: &Game) -> String {
    app.strings().round_text(game.score().total_rounds() + 1)
}

fn turn_label(app: &App, game: &Game) -> &'static str {
    let strings = app.strings();

    match game.mode() {
        Mode::TwoPlayers => {
            if game.turn() == Mark::X {
                strings.game_turn_player1
            } else {
                strings.game_turn_player2
            }
        }
        Mode::OnePlayer(_) => {
            if game.turn() == Mark::X {
                strings.game_turn_you
            } else if app.ai_pending() {
                strings.game_ai_thinking
            } else {
                strings.game_turn_ai
            }
        }
    }
}

fn player_label(app: &App, mark: Mark) -> &'static str {
    let strings = app.strings();

    match (app.game().map(Game::mode), mark) {
        (Some(Mode::OnePlayer(_)), Mark::X) => strings.game_you,
        (Some(Mode::OnePlayer(_)), Mark::O) => strings.game_ai,
        (_, Mark::X) => strings.game_player1,
        (_, Mark::O) => strings.game_player2,
    }
}

fn result_text(app: &App, game: &Game) -> &'static str {
    let strings = app.strings();

    match game.outcome() {
        Outcome::Win { mark, .. } => match (game.mode(), mark) {
            (Mode::OnePlayer(_), Mark::X) => strings.result_win_you,
            (Mode::OnePlayer(_), Mark::O) => strings.result_win_ai,
            (Mode::TwoPlayers, Mark::X) => strings.result_win_player1,
            (Mode::TwoPlayers, Mark::O) => strings.result_win_player2,
        },
        Outcome::Draw => strings.result_draw,
        Outcome::InProgress => "",
    }
}

fn result_stamp(app: &App, game: &Game) -> Option<&'static str> {
    let strings = app.strings();

    match game.outcome() {
        Outcome::Win { .. } => Some(strings.result_stamp_win),
        Outcome::Draw => Some(strings.result_stamp_draw),
        Outcome::InProgress => None,
    }
}

fn score_labels(app: &App, game: &Game) -> (&'static str, &'static str) {
    let strings = app.strings();

    match game.mode() {
        Mode::TwoPlayers => (strings.game_player1, strings.game_player2),
        Mode::OnePlayer(_) => (strings.game_you, strings.game_ai),
    }
}

fn score_lines(app: &App, game: &Game) -> Vec<Line<'static>> {
    let strings = app.strings();
    let score = game.score();
    let (player1, player2) = score_labels(app, game);

    vec![
        score_line(app, player1, score.p1_wins()),
        score_line(app, player2, score.p2_wins()),
        score_line(app, strings.game_draws, score.draws()),
    ]
}

fn score_line(app: &App, label: &'static str, value: u16) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{label:<14}"),
            Style::default().fg(app.color(Token::TextOnPaper)),
        ),
        Span::styled(
            value.to_string(),
            Style::default()
                .fg(app.color(Token::TextOnPaper))
                .add_modifier(Modifier::BOLD),
        ),
    ])
}

fn score_inline(app: &App, game: &Game) -> String {
    let strings = app.strings();
    let score = game.score();
    let (player1, player2) = score_labels(app, game);

    format!(
        "{} {} · {} {} · {} {}",
        player1,
        score.p1_wins(),
        player2,
        score.p2_wins(),
        strings.game_draws,
        score.draws()
    )
}

fn pause_actions(app: &App) -> [&'static str; 4] {
    let strings = app.strings();

    [
        strings.action_resume,
        strings.action_restart,
        strings.action_main_menu,
        strings.action_quit_game,
    ]
}

fn result_actions(app: &App) -> [&'static str; 3] {
    let strings = app.strings();

    [
        strings.action_rematch,
        strings.action_main_menu,
        strings.action_quit_game,
    ]
}

fn mark_color(app: &App, mark: Mark) -> ratatui::style::Color {
    match mark {
        Mark::X => app.color(Token::MarkX),
        Mark::O => app.color(Token::MarkO),
    }
}

fn menu_line(app: &App, label: &'static str, focused: bool) -> Line<'static> {
    if focused {
        let marker = if app.cursor_opacity() > 0.8 {
            Token::BrassBright
        } else {
            Token::Brass
        };

        Line::from(vec![
            Span::styled(
                "▸ ",
                Style::default()
                    .fg(app.color(marker))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                label,
                Style::default()
                    .fg(app.color(Token::BrassBright))
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        Line::from(Span::styled(
            format!("  {label}"),
            Style::default().fg(app.color(Token::TextOnWood)),
        ))
    }
}

fn menu_line_paper(app: &App, label: &'static str, focused: bool) -> Line<'static> {
    if focused {
        Line::from(vec![
            Span::styled(
                "▸ ",
                Style::default()
                    .fg(app.color(Token::Brass))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                label,
                Style::default()
                    .fg(app.color(Token::TextOnPaper))
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        Line::from(Span::styled(
            format!("  {label}"),
            Style::default().fg(app.color(Token::TextOnPaperMuted)),
        ))
    }
}

fn divider_line(app: &App, width: usize) -> Line<'static> {
    Line::from(Span::styled(
        "─".repeat(width),
        Style::default().fg(app.color(Token::Brass)),
    ))
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

fn board_cell_size(breakpoint: Breakpoint) -> (u16, u16) {
    match breakpoint {
        Breakpoint::Wide => (9, 3),
        Breakpoint::Compact => (7, 2),
        Breakpoint::Minimal => (5, 2),
    }
}

fn grid_lines(cell_width: u16, cell_height: u16) -> Vec<String> {
    let horizontal = "─".repeat(usize::from(cell_width));
    let separator = format!("{horizontal}┼{horizontal}┼{horizontal}");
    let empty = format!(
        "{}│{}│{}",
        " ".repeat(usize::from(cell_width)),
        " ".repeat(usize::from(cell_width)),
        " ".repeat(usize::from(cell_width))
    );
    let mut lines = Vec::with_capacity(usize::from(cell_height * 3 + 2));

    for row in 0..3 {
        for _ in 0..cell_height {
            lines.push(empty.clone());
        }

        if row < 2 {
            lines.push(separator.clone());
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::PathBuf;

    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    use super::*;
    use crate::app::StatusKind;
    use crate::i18n::Language;
    use crate::input::Action;
    use crate::theme::ColorMode;

    fn render(app: &App, cols: u16, rows: u16) -> String {
        let backend = TestBackend::new(cols, rows);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|frame| draw(frame, app)).unwrap();

        let buffer = terminal.backend().buffer();
        let width = usize::from(buffer.area.width);
        let mut text = String::new();

        for row in buffer.content().chunks(width) {
            let line: String = row.iter().map(|cell| cell.symbol()).collect();
            text.push_str(line.trim_end());
            text.push('\n');
        }

        text
    }

    fn app_with(language: Language) -> App {
        let mut app = App::new(language, ColorMode::TrueColor);
        app.set_animations_enabled(false);
        app.set_audio_enabled(true);
        app
    }

    fn app() -> App {
        app_with(Language::PtBr)
    }

    fn start_two_players(app: &mut App) {
        app.handle(Action::Down);
        app.handle(Action::Confirm);
    }

    fn snapshot_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("snapshots")
            .join(format!("{name}.txt"))
    }

    fn assert_snapshot(name: &str, actual: &str) {
        let path = snapshot_path(name);

        if env::var_os("UPDATE_SNAPSHOTS").is_some() {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, actual).unwrap();
            return;
        }

        let expected = fs::read_to_string(&path).unwrap_or_else(|error| {
            panic!(
                "missing snapshot {}: {error}; run with UPDATE_SNAPSHOTS=1",
                path.display()
            )
        });

        assert_eq!(actual, expected, "snapshot {name} differs");
    }

    #[test]
    fn grid_has_uniform_width_and_expected_height() {
        for (width, height) in [(9, 3), (7, 2), (5, 2)] {
            let lines = grid_lines(width, height);
            let expected_width = usize::from(width * 3 + 2);

            assert_eq!(lines.len(), usize::from(height * 3 + 2));
            assert!(lines
                .iter()
                .all(|line| line.chars().count() == expected_width));
        }
    }

    #[test]
    fn title_uses_brand_and_ornaments() {
        assert_eq!(
            title_text("Velha Clássica"),
            "✦  V E L H A   C L Á S S I C A ✦"
        );
    }

    #[test]
    fn renders_home_in_both_languages() {
        let portuguese = render(&app_with(Language::PtBr), 192, 54);
        let english = render(&app_with(Language::En), 192, 54);

        assert!(portuguese.contains("1 Jogador"));
        assert!(portuguese.contains("2 Jogadores"));
        assert!(portuguese.contains("Idioma: Português"));
        assert!(english.contains("1 Player"));
        assert!(english.contains("2 Players"));
        assert!(english.contains("Language: English"));
    }

    #[test]
    fn renders_difficulty_descriptions() {
        let mut app = app();
        app.handle(Action::Confirm);

        let text = render(&app, 192, 54);

        assert!(text.contains("Escolha a dificuldade"));
        assert!(text.contains("Fácil"));
        assert!(text.contains("Impossível"));
        assert!(text.contains("Nunca perde"));
    }

    #[test]
    fn renders_one_player_thinking_turn() {
        let mut app = App::with_seed(Language::PtBr, ColorMode::TrueColor, 7);
        app.handle(Action::Confirm);
        app.handle(Action::Confirm);
        app.play_for_test(4);

        let text = render(&app, 192, 54);

        assert!(text.contains("IA PENSANDO"));
        assert_eq!(app.game().unwrap().board().marks_count(), 1);
    }

    #[test]
    fn renders_two_player_game_and_marks() {
        let mut app = app();
        start_two_players(&mut app);
        app.play_for_test(4);

        let text = render(&app, 192, 54);

        assert!(text.contains("VEZ DO JOGADOR 2"));
        assert!(text.contains("Jogador 1"));
        assert!(text.contains("PLACAR DA SESSÃO"));
        assert!(text.contains("X"));
    }

    #[test]
    fn renders_invalid_move_feedback() {
        let mut app = app();
        start_two_players(&mut app);
        app.play_for_test(4);
        app.play_for_test(4);

        let text = render(&app, 192, 54);

        assert_eq!(app.status(), Some(StatusKind::Occupied));
        assert!(text.contains("Casa ocupada"));
    }

    #[test]
    fn renders_pause_overlay() {
        let mut app = app();
        start_two_players(&mut app);
        app.handle(Action::Back);

        let text = render(&app, 192, 54);

        assert!(text.contains("PAUSADO"));
        assert!(text.contains("Continuar"));
        assert!(text.contains("Reiniciar rodada"));
        assert!(text.contains("Sair do jogo"));
    }

    #[test]
    fn renders_round_over_with_actions() {
        let mut app = app();
        start_two_players(&mut app);
        for cell in [0, 3, 1, 4, 2] {
            app.play_for_test(cell);
        }

        let text = render(&app, 192, 54);

        assert!(text.contains("VITÓRIA"));
        assert!(text.contains("Jogador 1 venceu!"));
        assert!(text.contains("Revanche"));
        assert!(text.contains("Menu principal"));
    }

    #[test]
    fn renders_draw_result() {
        let mut app = app();
        start_two_players(&mut app);
        for cell in [0, 4, 8, 1, 2, 6, 3, 5, 7] {
            app.play_for_test(cell);
        }

        let text = render(&app, 192, 54);

        assert!(text.contains("EMPATE"));
        assert!(text.contains("Empate!"));
    }

    #[test]
    fn minimal_size_renders_every_screen() {
        let mut home_app = app();
        let home = render(&home_app, 80, 24);
        assert!(home.contains("2 Jogadores"));

        home_app.handle(Action::Confirm);
        let difficulty = render(&home_app, 80, 24);
        assert!(difficulty.contains("Escolha a dificuldade"));

        let mut app = app();
        start_two_players(&mut app);
        app.play_for_test(4);
        let playing = render(&app, 80, 24);
        assert!(playing.contains("Vez do Jogador 2"));

        app.handle(Action::Back);
        let paused = render(&app, 80, 24);
        assert!(paused.contains("PAUSADO"));
    }

    #[test]
    fn renders_minimum_size_warning() {
        let portuguese = render(&app(), 79, 23);
        let english = render(&app_with(Language::En), 79, 23);

        assert!(portuguese.contains("Terminal muito pequeno. Mínimo: 80×24."));
        assert!(english.contains("Terminal too small. Minimum: 80×24."));
    }

    #[test]
    fn resize_during_play_preserves_state() {
        let mut app = app();
        start_two_players(&mut app);
        app.play_for_test(4);

        let wide = render(&app, 192, 54);
        let compact = render(&app, 160, 46);
        let minimal = render(&app, 80, 24);

        assert!(wide.contains("VEZ DO JOGADOR 2"));
        assert!(compact.contains("Vez do Jogador 2"));
        assert!(minimal.contains("Vez do Jogador 2"));
        assert_eq!(app.game().unwrap().board().marks_count(), 1);
    }

    #[test]
    fn home_wide_snapshot() {
        let app = app();
        assert_snapshot("home_wide_pt", &render(&app, 192, 54));
    }

    #[test]
    fn home_minimal_english_snapshot() {
        let app = app_with(Language::En);
        assert_snapshot("home_minimal_en", &render(&app, 80, 24));
    }

    #[test]
    fn difficulty_wide_snapshot() {
        let mut app = app();
        app.handle(Action::Confirm);
        assert_snapshot("difficulty_wide_pt", &render(&app, 192, 54));
    }

    #[test]
    fn playing_wide_snapshot() {
        let mut app = app();
        start_two_players(&mut app);
        app.play_for_test(4);
        assert_snapshot("playing_wide_pt", &render(&app, 192, 54));
    }

    #[test]
    fn playing_compact_snapshot() {
        let mut app = app();
        start_two_players(&mut app);
        app.play_for_test(4);
        assert_snapshot("playing_compact_pt", &render(&app, 160, 46));
    }

    #[test]
    fn playing_minimal_snapshot() {
        let mut app = app();
        start_two_players(&mut app);
        app.play_for_test(4);
        assert_snapshot("playing_minimal_pt", &render(&app, 80, 24));
    }

    #[test]
    fn paused_wide_snapshot() {
        let mut app = app();
        start_two_players(&mut app);
        app.play_for_test(4);
        app.handle(Action::Back);
        assert_snapshot("paused_wide_pt", &render(&app, 192, 54));
    }

    #[test]
    fn round_over_wide_snapshot() {
        let mut app = app();
        start_two_players(&mut app);
        for cell in [0, 3, 1, 4, 2] {
            app.play_for_test(cell);
        }
        assert_snapshot("round_over_wide_pt", &render(&app, 192, 54));
    }

    #[test]
    fn round_over_draw_wide_snapshot() {
        let mut app = app();
        start_two_players(&mut app);
        for cell in [0, 4, 8, 1, 2, 6, 3, 5, 7] {
            app.play_for_test(cell);
        }
        assert_snapshot("round_over_draw_wide_pt", &render(&app, 192, 54));
    }

    #[test]
    fn too_small_snapshot() {
        let app = app();
        assert_snapshot("too_small_pt", &render(&app, 79, 23));
    }
}
