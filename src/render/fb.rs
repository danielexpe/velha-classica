use tiny_skia::{Paint, Pixmap, PixmapPaint, Rect, Transform};

use crate::app::App;
use crate::assets;
use crate::core::{winner, Difficulty, Mark, Outcome};
use crate::i18n::Strings;
use crate::render::scene::{self, SceneInput, Stamp};
use crate::render::text::{Align, Face, TextRenderer, TextStyle};
use crate::state::{Game, Mode, Screen};
use crate::theme::Token;

pub const WIDTH: u32 = 1344;
pub const HEIGHT: u32 = 756;

const COMPOSITION_RATIO: f32 = 0.70;
const MARGIN: f32 = 96.0;
#[derive(Debug, Clone, Copy)]
struct Area {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

const BOARD: Area = Area {
    x: 154.0,
    y: 170.0,
    width: 520.0,
    height: 520.0,
};
const PANEL: Area = Area {
    x: 730.0,
    y: 170.0,
    width: 460.0,
    height: 520.0,
};
const PANEL_PADDING: f32 = 36.0;
const FOOTER_Y: f32 = 706.0;

pub struct Renderer {
    text: TextRenderer,
    background: Pixmap,
    canvas: Pixmap,
    frame: Vec<u8>,
    screen_width: u32,
    screen_height: u32,
    scale: f32,
    offset_x: i32,
    offset_y: i32,
}

impl Renderer {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let screen_width = screen_width.max(1);
        let screen_height = screen_height.max(1);
        let (scale, offset_x, offset_y) = geometry(screen_width, screen_height);

        Self {
            text: TextRenderer::new(),
            background: background(screen_width, screen_height),
            canvas: canvas(),
            frame: vec![0; screen_width as usize * screen_height as usize * 4],
            screen_width,
            screen_height,
            scale,
            offset_x,
            offset_y,
        }
    }

    pub const fn screen_width(&self) -> u32 {
        self.screen_width
    }

    pub const fn screen_height(&self) -> u32 {
        self.screen_height
    }

    pub fn render(&mut self, app: &App) -> &[u8] {
        self.paint(app);
        self.frame.copy_from_slice(self.background.data());
        composite_scaled(
            &mut self.frame,
            self.screen_width,
            self.screen_height,
            &self.canvas,
            self.scale,
            self.offset_x,
            self.offset_y,
        );

        &self.frame
    }

    pub fn capture(&mut self, app: &App) -> Pixmap {
        self.paint(app);

        let mut output = background(WIDTH, HEIGHT);

        composite_scaled(output.data_mut(), WIDTH, HEIGHT, &self.canvas, 1.0, 0, 0);
        output
    }

    fn paint(&mut self, app: &App) {
        self.canvas.data_mut().fill(0);

        match app.screen() {
            Screen::Home => self.draw_home(app),
            Screen::Difficulty => self.draw_difficulty(app),
            Screen::Playing => self.draw_game(app),
            Screen::Paused => {
                self.draw_game(app);
                self.draw_pause(app);
            }
            Screen::RoundOver => self.draw_game(app),
        }
    }

    fn draw_home(&mut self, app: &App) {
        let strings = app.strings();
        let title = spaced(app.strings().app_title);

        self.centered_line(
            122.0,
            &title,
            TextStyle::new(Face::SerifBold, 76.0, Token::BrassBright),
        );
        self.centered_line(
            244.0,
            &format!("{} · {}", strings.app_subtitle, strings.app_tagline),
            TextStyle::new(Face::Serif, 28.0, Token::TextOnWoodMuted),
        );
        self.divider(318.0, 520.0);

        let items = [
            strings.menu_one_player,
            strings.menu_two_players,
            strings.menu_quit,
        ];

        for (index, label) in items.into_iter().enumerate() {
            self.menu_item(
                label,
                368.0 + index as f32 * 58.0,
                index == app.home_selection(),
                false,
            );
        }

        self.footer(app, Footer::Home);
    }

    fn draw_difficulty(&mut self, app: &App) {
        let strings = app.strings();
        let title = spaced(app.strings().app_title);

        self.centered_line(
            92.0,
            &title,
            TextStyle::new(Face::SerifBold, 54.0, Token::BrassBright),
        );
        self.centered_line(
            186.0,
            strings.menu_choose_difficulty,
            TextStyle::new(Face::SansBold, 30.0, Token::Brass).align(Align::Center),
        );
        self.centered_line(
            230.0,
            strings.difficulty_human_starts,
            TextStyle::new(Face::Sans, 20.0, Token::TextOnWoodMuted),
        );

        let mut y = 296.0;

        for (index, difficulty) in Difficulty::ALL.into_iter().enumerate() {
            self.menu_item(
                strings.difficulty_name(difficulty),
                y,
                index == app.difficulty_selection(),
                false,
            );
            y += 34.0;
            self.centered_line(
                y,
                strings.difficulty_description(difficulty),
                TextStyle::new(Face::Sans, 18.0, Token::TextOnWoodMuted),
            );
            y += 52.0;
        }

        self.menu_item(
            strings.menu_back,
            y,
            app.difficulty_selection() == Difficulty::ALL.len(),
            false,
        );

        self.footer(app, Footer::Difficulty);
    }

    fn draw_game(&mut self, app: &App) {
        let Some(game) = app.game() else {
            self.draw_home(app);
            return;
        };

        self.draw_header(app, game);
        self.draw_board(app, game);
        self.draw_panel(app, game);
        let footer = if app.screen() == Screen::RoundOver {
            Footer::RoundOver
        } else {
            Footer::Game
        };
        self.footer(app, footer);
    }

    fn draw_header(&mut self, app: &App, game: &Game) {
        let strings = app.strings();

        self.text_left(
            MARGIN,
            30.0,
            &spaced(strings.app_title),
            TextStyle::new(Face::SerifBold, 34.0, Token::BrassBright),
            WIDTH as f32 - MARGIN * 2.0,
        );
        self.text_right(
            WIDTH as f32 - MARGIN,
            36.0,
            &indicators(app),
            TextStyle::new(Face::Sans, 16.0, Token::TextOnWoodMuted),
            WIDTH as f32 - MARGIN * 2.0,
        );
        self.text_left(
            MARGIN,
            82.0,
            &format!("{} · {}", strings.app_subtitle, strings.app_tagline),
            TextStyle::new(Face::Sans, 18.0, Token::TextOnWoodMuted),
            WIDTH as f32 - MARGIN * 2.0,
        );
        self.text_left(
            MARGIN,
            110.0,
            &format!(
                "{} · {}",
                mode_text(strings, game),
                round_text(strings, game)
            ),
            TextStyle::new(Face::Sans, 20.0, Token::TextOnWood),
            WIDTH as f32 - MARGIN * 2.0,
        );
        self.divider(144.0, WIDTH as f32 - MARGIN * 2.0);
    }

    fn draw_board(&mut self, app: &App, game: &Game) {
        let round_over = app.screen() == Screen::RoundOver;
        let cursor = app.screen() == Screen::Playing
            && !app.ai_pending()
            && game.board().get(game.selected()).is_none();
        let input = SceneInput {
            selected: game.selected(),
            turn: game.turn(),
            show_cursor: cursor,
            cursor_opacity: app.cursor_opacity(),
            winning_line: winner(game.board()).map(|(_, line)| line),
            win_progress: app.winning_progress(),
            last_move: app.last_move_cell(),
            mark_progress: app.mark_progress(),
            stamp: round_over.then(|| stamp(app)),
        };
        let scene = scene::render(game.board(), input);

        draw_fitted(&mut self.canvas, &scene, BOARD);
    }

    fn draw_panel(&mut self, app: &App, game: &Game) {
        let strings = app.strings();
        let panel = assets::paper_panel();

        draw_fitted(&mut self.canvas, panel, PANEL);

        let x = PANEL.x + PANEL_PADDING;
        let width = PANEL.width - PANEL_PADDING * 2.0;
        let mut y = PANEL.y + 30.0;

        if app.screen() == Screen::RoundOver {
            self.text_left(
                x,
                y,
                result_text(strings, game),
                TextStyle::new(Face::SerifBold, 32.0, Token::TextOnPaper),
                width,
            );
        } else {
            self.text_left(
                x,
                y,
                &mode_text(strings, game),
                TextStyle::new(Face::SerifBold, 28.0, Token::TextOnPaper),
                width,
            );
        }

        y += 48.0;
        self.rule(x, y, width);
        y += 24.0;

        if app.screen() == Screen::RoundOver {
            self.text_left(
                x,
                y,
                strings.game_session_score,
                TextStyle::new(Face::SansBold, 17.0, Token::TextOnPaperMuted),
                width,
            );
        } else {
            self.text_left(
                x,
                y,
                &turn_label(app).to_uppercase(),
                TextStyle::new(Face::SansBold, 18.0, Token::TextOnPaperMuted),
                width,
            );
            y += 30.0;
            let color = if game.turn() == Mark::X {
                Token::MarkX
            } else {
                Token::MarkO
            };
            self.text_left(
                x,
                y,
                &format!(
                    "{} · {}",
                    player_label(strings, game, game.turn()),
                    game.turn().glyph()
                ),
                TextStyle::new(Face::SansBold, 24.0, color),
                width,
            );
            if let Some(status) = app.status_text() {
                self.text_left(
                    x,
                    y + 34.0,
                    status,
                    TextStyle::new(Face::SansBold, 17.0, Token::Danger),
                    width,
                );
            }
        }

        y += 54.0;

        if app.screen() != Screen::RoundOver {
            self.text_left(
                x,
                y,
                strings.game_session_score.to_uppercase().as_str(),
                TextStyle::new(Face::SansBold, 17.0, Token::TextOnPaperMuted),
                width,
            );
            y += 30.0;
        }

        let score = game.score();
        let (player1, player2) = score_labels(strings, game);
        let rows = [
            (player1, score.p1_wins()),
            (player2, score.p2_wins()),
            (strings.game_draws, score.draws()),
        ];

        for (label, value) in rows {
            self.text_left(
                x,
                y,
                label,
                TextStyle::new(Face::Sans, 20.0, Token::TextOnPaper),
                width - 40.0,
            );
            self.text_right(
                x + width,
                y - 2.0,
                &value.to_string(),
                TextStyle::new(Face::SansBold, 24.0, Token::TextOnPaper),
                width,
            );
            y += 34.0;
        }

        y += 18.0;
        self.rule(x, y, width);
        y += 24.0;

        if app.screen() == Screen::RoundOver {
            for (index, label) in result_actions(strings).into_iter().enumerate() {
                self.paper_menu_item(label, y, index == app.result_selection());
                y += 44.0;
            }
        } else {
            for hint in [
                strings.footer_move,
                strings.footer_select_square,
                strings.footer_pause,
            ] {
                self.text_left(
                    x,
                    y,
                    hint,
                    TextStyle::new(Face::Sans, 16.0, Token::TextOnPaperMuted),
                    width,
                );
                y += 26.0;
            }
        }
    }

    fn draw_pause(&mut self, app: &App) {
        let strings = app.strings();

        fill_rect(
            &mut self.canvas,
            0.0,
            0.0,
            WIDTH as f32,
            HEIGHT as f32,
            Token::Overlay,
            0.72,
        );

        let card = Area {
            x: 342.0,
            y: 188.0,
            width: 660.0,
            height: 380.0,
        };

        fill_rect(
            &mut self.canvas,
            card.x + 10.0,
            card.y + 12.0,
            card.width,
            card.height,
            Token::Shadow,
            0.55,
        );
        fill_rect(
            &mut self.canvas,
            card.x,
            card.y,
            card.width,
            card.height,
            Token::SurfacePaper,
            1.0,
        );
        stroke_rect(&mut self.canvas, card, Token::Brass, 2.0);

        self.text_centered(
            card.x,
            card.y + 34.0,
            &strings.pause_title.to_uppercase(),
            TextStyle::new(Face::SerifBold, 42.0, Token::TextOnPaper),
            card.width,
        );

        let mut y = card.y + 128.0;

        for (index, label) in pause_actions(strings).into_iter().enumerate() {
            self.paper_menu_item_in(card.x, card.width, label, y, index == app.pause_selection());
            y += 56.0;
        }
    }

    fn footer(&mut self, app: &App, footer: Footer) {
        let hints = footer_hints(app.strings(), app, footer);

        self.text_left(
            MARGIN,
            FOOTER_Y,
            &hints,
            TextStyle::new(Face::Sans, 17.0, Token::TextOnWoodMuted),
            WIDTH as f32 - MARGIN * 2.0 - 120.0,
        );
        self.text_right(
            WIDTH as f32 - MARGIN,
            FOOTER_Y,
            &app.strings().version_text(app.version()),
            TextStyle::new(Face::Sans, 17.0, Token::TextOnWoodMuted),
            WIDTH as f32 - MARGIN * 2.0,
        );
    }

    fn text_left(
        &mut self,
        x: f32,
        y: f32,
        text: &str,
        style: TextStyle,
        max_width: f32,
    ) -> (f32, f32) {
        self.text
            .draw(&mut self.canvas, text, style, x, y, max_width)
    }

    fn text_right(
        &mut self,
        right: f32,
        y: f32,
        text: &str,
        style: TextStyle,
        max_width: f32,
    ) -> (f32, f32) {
        let style = style.align(Align::Left);
        let (width, height) = self.text.measure(text, style, max_width);
        let x = (right - width).max(0.0);

        self.text
            .draw(&mut self.canvas, text, style, x, y, max_width);
        (x, height)
    }

    fn text_centered(
        &mut self,
        area_x: f32,
        y: f32,
        text: &str,
        style: TextStyle,
        area_width: f32,
    ) -> (f32, f32) {
        let measured = style.align(Align::Left);
        let (width, height) = self.text.measure(text, measured, area_width);
        let x = area_x + (area_width - width) / 2.0;

        self.text.draw(
            &mut self.canvas,
            text,
            measured,
            x.max(area_x),
            y,
            area_width,
        );
        (x, height)
    }

    fn centered_line(&mut self, y: f32, text: &str, style: TextStyle) -> (f32, f32) {
        self.text_centered(0.0, y, text, style, WIDTH as f32)
    }

    fn menu_item(&mut self, label: &str, y: f32, selected: bool, on_paper: bool) {
        self.menu_item_in(0.0, WIDTH as f32, label, y, selected, on_paper);
    }

    fn paper_menu_item(&mut self, label: &str, y: f32, selected: bool) {
        self.menu_item_in(
            PANEL.x + PANEL_PADDING,
            PANEL.width - PANEL_PADDING * 2.0,
            label,
            y,
            selected,
            true,
        );
    }

    fn paper_menu_item_in(&mut self, x: f32, width: f32, label: &str, y: f32, selected: bool) {
        self.menu_item_in(x, width, label, y, selected, true);
    }

    fn menu_item_in(
        &mut self,
        x: f32,
        width: f32,
        label: &str,
        y: f32,
        selected: bool,
        on_paper: bool,
    ) {
        let color = match (selected, on_paper) {
            (true, true) => Token::TextOnPaper,
            (true, false) => Token::BrassBright,
            (false, true) => Token::TextOnPaperMuted,
            (false, false) => Token::TextOnWood,
        };
        let face = if selected { Face::SansBold } else { Face::Sans };
        let style = TextStyle::new(face, 28.0, color);
        let (text_width, text_height) = self.text.measure(label, style, width);
        let text_x = x + (width - text_width) / 2.0;

        if selected {
            fill_rect(
                &mut self.canvas,
                text_x - 26.0,
                y + text_height * 0.30,
                10.0,
                9.0,
                Token::Brass,
                1.0,
            );
        }

        self.text
            .draw(&mut self.canvas, label, style, text_x, y, width);
    }

    fn divider(&mut self, y: f32, width: f32) {
        self.rule((WIDTH as f32 - width) / 2.0, y, width);
    }

    fn rule(&mut self, x: f32, y: f32, width: f32) {
        fill_rect(&mut self.canvas, x, y, width, 1.5, Token::Brass, 0.85);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Footer {
    Home,
    Difficulty,
    Game,
    RoundOver,
}

fn geometry(screen_width: u32, screen_height: u32) -> (f32, i32, i32) {
    let scale = (screen_width as f32 * COMPOSITION_RATIO / WIDTH as f32)
        .min(screen_height as f32 * COMPOSITION_RATIO / HEIGHT as f32)
        .max(0.01);
    let drawn_width = WIDTH as f32 * scale;
    let drawn_height = HEIGHT as f32 * scale;

    (
        scale,
        ((screen_width as f32 - drawn_width) / 2.0).round() as i32,
        ((screen_height as f32 - drawn_height) / 2.0).round() as i32,
    )
}

fn canvas() -> Pixmap {
    Pixmap::new(WIDTH, HEIGHT).expect("canvas dimensions are valid")
}

fn background(width: u32, height: u32) -> Pixmap {
    let mut background = Pixmap::new(width, height).expect("framebuffer dimensions are valid");
    let wood = assets::decode(assets::WOOD_BG).expect("embedded wood background");

    background.draw_pixmap(
        0,
        0,
        wood.as_ref(),
        &PixmapPaint::default(),
        Transform::from_scale(
            width as f32 / wood.width() as f32,
            height as f32 / wood.height() as f32,
        ),
        None,
    );

    let grain = assets::decode(assets::GRAIN_OVERLAY).expect("embedded grain overlay");

    for y in (0..height).step_by(1024) {
        for x in (0..width).step_by(1024) {
            background.draw_pixmap(
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

    background.draw_pixmap(
        0,
        0,
        vignette.as_ref(),
        &PixmapPaint::default(),
        Transform::from_scale(
            width as f32 / vignette.width() as f32,
            height as f32 / vignette.height() as f32,
        ),
        None,
    );

    background
}

fn draw_fitted(target: &mut Pixmap, source: &Pixmap, area: Area) {
    let scale = (area.width / source.width() as f32).min(area.height / source.height() as f32);
    let drawn_width = source.width() as f32 * scale;
    let drawn_height = source.height() as f32 * scale;
    let transform = Transform::from_scale(scale, scale).post_translate(
        area.x + (area.width - drawn_width) / 2.0,
        area.y + (area.height - drawn_height) / 2.0,
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

fn fill_rect(
    target: &mut Pixmap,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    token: Token,
    opacity: f32,
) {
    let Some(rect) = Rect::from_xywh(x, y, width, height) else {
        return;
    };
    let rgb = token.rgb();
    let alpha = (opacity.clamp(0.0, 1.0) * 255.0).round() as u8;
    let mut paint = Paint::default();

    paint.set_color_rgba8(rgb.red, rgb.green, rgb.blue, alpha);
    target.fill_rect(rect, &paint, Transform::identity(), None);
}

fn stroke_rect(target: &mut Pixmap, rect: Area, token: Token, width: f32) {
    fill_rect(target, rect.x, rect.y, rect.width, width, token, 1.0);
    fill_rect(
        target,
        rect.x,
        rect.y + rect.height - width,
        rect.width,
        width,
        token,
        1.0,
    );
    fill_rect(target, rect.x, rect.y, width, rect.height, token, 1.0);
    fill_rect(
        target,
        rect.x + rect.width - width,
        rect.y,
        width,
        rect.height,
        token,
        1.0,
    );
}

fn composite_scaled(
    destination: &mut [u8],
    destination_width: u32,
    destination_height: u32,
    source: &Pixmap,
    scale: f32,
    offset_x: i32,
    offset_y: i32,
) {
    let source_width = source.width();
    let source_height = source.height();
    let source_data = source.data();
    let drawn_width = (source_width as f32 * scale).round() as i32;
    let drawn_height = (source_height as f32 * scale).round() as i32;
    let left = offset_x.max(0);
    let top = offset_y.max(0);
    let right = (offset_x + drawn_width).min(destination_width as i32);
    let bottom = (offset_y + drawn_height).min(destination_height as i32);
    let last_x = source_width.saturating_sub(1) as usize;
    let last_y = source_height.saturating_sub(1) as usize;

    if left >= right || top >= bottom {
        return;
    }

    if (scale - 1.0).abs() < 0.001
        && offset_x >= 0
        && offset_y >= 0
        && offset_x as u32 + source_width <= destination_width
        && offset_y as u32 + source_height <= destination_height
    {
        composite_unscaled(
            destination,
            destination_width,
            source,
            offset_x as usize,
            offset_y as usize,
        );
        return;
    }

    for y in top..bottom {
        let source_y = ((y as f32 - offset_y as f32 + 0.5) / scale - 0.5).max(0.0);
        let y0 = (source_y.floor() as usize).min(last_y);
        let y1 = (y0 + 1).min(last_y);
        let weight_y = (source_y - y0 as f32).clamp(0.0, 1.0);

        for x in left..right {
            let source_x = ((x as f32 - offset_x as f32 + 0.5) / scale - 0.5).max(0.0);
            let x0 = (source_x.floor() as usize).min(last_x);
            let x1 = (x0 + 1).min(last_x);
            let weight_x = (source_x - x0 as f32).clamp(0.0, 1.0);
            let sample = sample_bilinear(
                source_data,
                source_width as usize,
                (x0, y0),
                (x1, y1),
                weight_x,
                weight_y,
            );
            let alpha = u32::from(sample[3]);

            if alpha == 0 {
                continue;
            }

            let destination_index = ((y as u32 * destination_width + x as u32) * 4) as usize;
            let inverse = 255 - alpha;

            for channel in 0..3 {
                let blended = u32::from(sample[channel])
                    + (u32::from(destination[destination_index + channel]) * inverse + 127) / 255;
                destination[destination_index + channel] = blended.min(255) as u8;
            }

            let blended_alpha =
                alpha + (u32::from(destination[destination_index + 3]) * inverse + 127) / 255;
            destination[destination_index + 3] = blended_alpha.min(255) as u8;
        }
    }
}

fn composite_unscaled(
    destination: &mut [u8],
    destination_width: u32,
    source: &Pixmap,
    offset_x: usize,
    offset_y: usize,
) {
    let source_width = source.width() as usize;
    let source_height = source.height() as usize;
    let source_data = source.data();
    let destination_width = destination_width as usize;

    for row in 0..source_height {
        let source_start = row * source_width * 4;
        let destination_start = ((offset_y + row) * destination_width + offset_x) * 4;
        let source_row = &source_data[source_start..source_start + source_width * 4];
        let destination_row =
            &mut destination[destination_start..destination_start + source_width * 4];

        for (pixel, target) in source_row
            .as_chunks::<4>()
            .0
            .iter()
            .zip(destination_row.as_chunks_mut::<4>().0.iter_mut())
        {
            let alpha = u32::from(pixel[3]);

            if alpha == 0 {
                continue;
            }

            let inverse = 255 - alpha;

            for channel in 0..3 {
                let blended =
                    u32::from(pixel[channel]) + (u32::from(target[channel]) * inverse + 127) / 255;
                target[channel] = blended.min(255) as u8;
            }

            let blended_alpha = alpha + (u32::from(target[3]) * inverse + 127) / 255;
            target[3] = blended_alpha.min(255) as u8;
        }
    }
}

fn sample_bilinear(
    data: &[u8],
    width: usize,
    top_left: (usize, usize),
    bottom_right: (usize, usize),
    weight_x: f32,
    weight_y: f32,
) -> [u8; 4] {
    let first = (top_left.1 * width + top_left.0) * 4;
    let second = (top_left.1 * width + bottom_right.0) * 4;
    let third = (bottom_right.1 * width + top_left.0) * 4;
    let fourth = (bottom_right.1 * width + bottom_right.0) * 4;
    let mut output = [0u8; 4];

    for (channel, value) in output.iter_mut().enumerate() {
        let top = f32::from(data[first + channel]) * (1.0 - weight_x)
            + f32::from(data[second + channel]) * weight_x;
        let bottom = f32::from(data[third + channel]) * (1.0 - weight_x)
            + f32::from(data[fourth + channel]) * weight_x;
        let mixed = top * (1.0 - weight_y) + bottom * weight_y;

        *value = mixed.round().clamp(0.0, 255.0) as u8;
    }

    output
}

fn spaced(title: &str) -> String {
    let mut spaced = String::new();

    for character in title.to_uppercase().chars() {
        spaced.push(character);
        spaced.push(' ');
    }

    spaced.trim_end().to_string()
}

fn indicators(app: &App) -> String {
    let strings = app.strings();

    [
        strings.language_status(app.language()),
        strings.sound_status(app.sound_enabled()).to_string(),
        strings.version_text(app.version()),
    ]
    .join(" · ")
}

fn footer_hints(strings: &Strings, app: &App, footer: Footer) -> String {
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

fn mode_text(strings: &Strings, game: &Game) -> String {
    match game.mode() {
        Mode::TwoPlayers => strings.game_mode_two.to_string(),
        Mode::OnePlayer(difficulty) => format!(
            "{} · {}",
            strings.game_mode_one,
            strings.difficulty_status(difficulty)
        ),
    }
}

fn round_text(strings: &Strings, game: &Game) -> String {
    strings.round_text(game.score().total_rounds() + 1)
}

fn turn_label(app: &App) -> &'static str {
    let strings = app.strings();

    match app.game().map(Game::mode) {
        Some(Mode::TwoPlayers) => match app.game().map(Game::turn) {
            Some(Mark::O) => strings.game_turn_player2,
            _ => strings.game_turn_player1,
        },
        Some(Mode::OnePlayer(_)) => match app.game().map(Game::turn) {
            Some(Mark::X) => strings.game_turn_you,
            _ if app.ai_pending() => strings.game_ai_thinking,
            _ => strings.game_turn_ai,
        },
        None => "",
    }
}

fn player_label(strings: &Strings, game: &Game, mark: Mark) -> &'static str {
    match (game.mode(), mark) {
        (Mode::OnePlayer(_), Mark::X) => strings.game_you,
        (Mode::OnePlayer(_), Mark::O) => strings.game_ai,
        (_, Mark::X) => strings.game_player1,
        (_, Mark::O) => strings.game_player2,
    }
}

fn score_labels(strings: &Strings, game: &Game) -> (&'static str, &'static str) {
    match game.mode() {
        Mode::TwoPlayers => (strings.game_player1, strings.game_player2),
        Mode::OnePlayer(_) => (strings.game_you, strings.game_ai),
    }
}

fn result_text(strings: &Strings, game: &Game) -> &'static str {
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

fn stamp(app: &App) -> Stamp {
    let win = matches!(app.game().map(Game::outcome), Some(Outcome::Win { .. }));

    match (app.language(), win) {
        (crate::i18n::Language::PtBr, true) => Stamp::WinPt,
        (crate::i18n::Language::En, true) => Stamp::WinEn,
        (crate::i18n::Language::PtBr, false) => Stamp::DrawPt,
        (crate::i18n::Language::En, false) => Stamp::DrawEn,
    }
}

fn pause_actions(strings: &Strings) -> [&'static str; 4] {
    [
        strings.action_resume,
        strings.action_restart,
        strings.action_main_menu,
        strings.action_quit_game,
    ]
}

fn result_actions(strings: &Strings) -> [&'static str; 3] {
    [
        strings.action_rematch,
        strings.action_main_menu,
        strings.action_quit_game,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;
    use crate::state::Mode;
    use crate::theme::ColorMode;

    fn app() -> App {
        let mut app = App::with_seed(Language::PtBr, ColorMode::TrueColor, 7);
        app.set_animations_enabled(false);
        app
    }

    fn two_players() -> App {
        let mut app = app();
        app.handle(crate::input::Action::Down);
        app.handle(crate::input::Action::Confirm);
        app
    }

    fn round_over() -> App {
        let mut app = two_players();

        for cell in [0, 3, 1, 4, 2] {
            app.play_for_test(cell);
        }

        app
    }

    #[test]
    fn geometry_centers_seventy_percent_on_1080p() {
        let (scale, x, y) = geometry(1920, 1080);

        assert!((scale - 1.0).abs() < 0.001);
        assert_eq!((x, y), (288, 162));
    }

    #[test]
    fn geometry_scales_down_on_smaller_screens() {
        let (scale, x, y) = geometry(1280, 720);

        assert!(scale < 0.7);
        assert!(x > 0);
        assert!(y > 0);
    }

    #[test]
    fn background_matches_the_screen_size() {
        let background = background(800, 600);

        assert_eq!((background.width(), background.height()), (800, 600));
    }

    #[test]
    fn every_screen_renders_a_full_frame() {
        let mut renderer = Renderer::new(1920, 1080);

        for screen in [
            Screen::Home,
            Screen::Difficulty,
            Screen::Playing,
            Screen::Paused,
            Screen::RoundOver,
        ] {
            let app = match screen {
                Screen::Home => app(),
                Screen::Difficulty => {
                    let mut app = app();
                    app.handle(crate::input::Action::Confirm);
                    app
                }
                Screen::Playing => two_players(),
                Screen::Paused => {
                    let mut app = two_players();
                    app.handle(crate::input::Action::Back);
                    app
                }
                Screen::RoundOver => round_over(),
            };

            let frame = renderer.render(&app);

            assert_eq!(frame.len(), 1920 * 1080 * 4);
            assert!(frame.as_chunks::<4>().0.iter().any(|pixel| pixel[3] == 255));
        }
    }

    #[test]
    fn menu_selection_changes_the_frame() {
        let mut app = two_players();
        let mut renderer = Renderer::new(1920, 1080);
        app.handle(crate::input::Action::Back);
        let paused = renderer.render(&app).to_vec();
        app.handle(crate::input::Action::Down);

        assert_ne!(renderer.render(&app).to_vec(), paused);
    }

    #[test]
    fn language_changes_the_rendered_text() {
        let mut app = app();
        let mut renderer = Renderer::new(1920, 1080);
        let portuguese = renderer.render(&app).to_vec();
        app.handle(crate::input::Action::ToggleLanguage);

        assert_ne!(renderer.render(&app), portuguese);
    }

    #[test]
    fn scaled_composition_overlays_transparent_pixels() {
        let mut source = Pixmap::new(2, 1).unwrap();

        source.data_mut()[0..4].copy_from_slice(&[255, 0, 0, 255]);
        let mut destination = vec![10u8, 20, 30, 255, 40, 50, 60, 255];

        composite_scaled(&mut destination, 2, 1, &source, 1.0, 0, 0);

        assert_eq!(&destination[0..4], &[255, 0, 0, 255]);
        assert_eq!(&destination[4..8], &[40, 50, 60, 255]);
    }

    #[test]
    fn scaled_composition_uses_bilinear_sampling() {
        let mut source = Pixmap::new(1, 1).unwrap();

        source.data_mut().copy_from_slice(&[255, 0, 0, 255]);
        let mut destination = vec![0u8; 16];

        composite_scaled(&mut destination, 4, 1, &source, 2.0, 0, 0);

        assert_eq!(&destination[0..4], &[255, 0, 0, 255]);
        assert_eq!(&destination[4..8], &[255, 0, 0, 255]);
        assert_eq!(&destination[8..12], &[0, 0, 0, 0]);
    }

    #[test]
    fn game_mode_is_reported_in_the_header() {
        let app = two_players();
        let game = app.game().unwrap();

        assert_eq!(mode_text(app.strings(), game), app.strings().game_mode_two);
        assert_eq!(game.mode(), Mode::TwoPlayers);
        assert_eq!(game.turn(), Mark::X);
    }
}
