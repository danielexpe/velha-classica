use std::cell::{RefCell, RefMut};
use std::io::{self, stdout, Stdout};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::style::Color;
use ratatui::Terminal;

use crate::ai::{choose_move, AI_MARK};
use crate::audio::{self, Audio, Sound};
use crate::core::{BoardError, Difficulty};
use crate::i18n::{Language, Strings};
use crate::input::{map_key, Action};
use crate::render;
use crate::render::image::{self, ImageProtocol, ImageState};
use crate::state::{Game, Mode, MoveError, Screen, Settings};
use crate::theme::{ColorMode, Token};
use crate::util::rng::Rng;
use crate::util::tty::{install_panic_hook, TerminalGuard};

const STATUS_DURATION: Duration = Duration::from_millis(900);
const CONFIRM_DEBOUNCE: Duration = Duration::from_millis(150);
const AI_THINK_MIN_MS: u64 = 450;
const AI_THINK_MAX_MS: u64 = 800;
const AI_THINK_REDUCED_MS: u64 = 250;
const MARK_ANIMATION: Duration = Duration::from_millis(140);
const WIN_ANIMATION: Duration = Duration::from_millis(450);
const FRAME_TIME: Duration = Duration::from_millis(40);
const EVENT_POLL: Duration = Duration::from_millis(100);
const HOME_ITEMS: usize = 3;
const DIFFICULTY_ITEMS: usize = 4;
const PAUSE_ITEMS: usize = 4;
const RESULT_ITEMS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusKind {
    Occupied,
}

impl StatusKind {
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Occupied)
    }
}

#[derive(Debug, Clone, Copy)]
struct Status {
    kind: StatusKind,
    expires_at: Instant,
}

pub struct App {
    screen: Screen,
    settings: Settings,
    color_mode: ColorMode,
    quit: bool,
    version: &'static str,
    game: Option<Game>,
    home_selection: usize,
    difficulty_selection: usize,
    pause_selection: usize,
    result_selection: usize,
    status: Option<Status>,
    last_confirm: Option<Instant>,
    rng: Rng,
    ai_pending: Option<Instant>,
    images_enabled: bool,
    image_protocol: ImageProtocol,
    image_state: RefCell<Option<ImageState>>,
    audio: Audio,
    last_move: Option<usize>,
    last_move_at: Option<Instant>,
    round_over_at: Option<Instant>,
    started: Instant,
}

impl App {
    pub fn new(language: Language, color_mode: ColorMode) -> Self {
        Self::with_seed(language, color_mode, crate::util::rng::entropy_seed())
    }

    pub fn with_seed(language: Language, color_mode: ColorMode, seed: u64) -> Self {
        let mut settings = Settings::new(language);
        settings.sound_enabled = audio::available();

        Self {
            screen: Screen::Home,
            settings,
            color_mode,
            quit: false,
            version: env!("CARGO_PKG_VERSION"),
            game: None,
            home_selection: 0,
            difficulty_selection: 0,
            pause_selection: 0,
            result_selection: 0,
            status: None,
            last_confirm: None,
            rng: Rng::new(seed),
            ai_pending: None,
            images_enabled: true,
            image_protocol: ImageProtocol::Auto,
            image_state: RefCell::new(None),
            audio: Audio::new(audio::DEFAULT_VOLUME),
            last_move: None,
            last_move_at: None,
            round_over_at: None,
            started: Instant::now(),
        }
    }

    pub const fn screen(&self) -> Screen {
        self.screen
    }

    pub const fn language(&self) -> Language {
        self.settings.language
    }

    pub const fn strings(&self) -> &'static Strings {
        self.settings.language.strings()
    }

    pub const fn color_mode(&self) -> ColorMode {
        self.color_mode
    }

    pub const fn sound_enabled(&self) -> bool {
        self.settings.sound_enabled
    }

    pub const fn version(&self) -> &'static str {
        self.version
    }

    pub const fn should_quit(&self) -> bool {
        self.quit
    }

    pub const fn game(&self) -> Option<&Game> {
        self.game.as_ref()
    }

    pub const fn home_selection(&self) -> usize {
        self.home_selection
    }

    pub const fn difficulty_selection(&self) -> usize {
        self.difficulty_selection
    }

    pub const fn pause_selection(&self) -> usize {
        self.pause_selection
    }

    pub const fn result_selection(&self) -> usize {
        self.result_selection
    }

    pub fn status(&self) -> Option<StatusKind> {
        self.status
            .filter(|status| Instant::now() < status.expires_at)
            .map(|status| status.kind)
    }

    pub fn status_is_error(&self) -> bool {
        self.status().is_some_and(StatusKind::is_error)
    }

    pub fn status_text(&self) -> Option<&'static str> {
        let strings = self.strings();

        self.status()
            .map(|StatusKind::Occupied| strings.game_occupied)
    }

    pub fn ai_pending(&self) -> bool {
        self.ai_pending.is_some()
    }

    pub fn set_images_enabled(&mut self, enabled: bool) {
        self.images_enabled = enabled;

        if !enabled {
            self.image_state.borrow_mut().take();
        }
    }

    pub fn set_image_protocol(&mut self, protocol: ImageProtocol) {
        self.image_protocol = protocol;
    }

    pub fn set_animations_enabled(&mut self, enabled: bool) {
        self.settings.animations_enabled = enabled;
    }

    pub fn set_audio_enabled(&mut self, enabled: bool) {
        self.settings.sound_enabled = enabled;
    }

    pub fn has_images(&self) -> bool {
        self.images_enabled && self.image_state.borrow().is_some()
    }

    pub(crate) fn image_state(&self) -> RefMut<'_, Option<ImageState>> {
        self.image_state.borrow_mut()
    }

    pub fn animations_enabled(&self) -> bool {
        self.settings.animations_enabled
    }

    pub fn animated_cell(&self) -> Option<usize> {
        if self.mark_progress() < 1.0 {
            self.last_move
        } else {
            None
        }
    }

    pub fn last_move_cell(&self) -> Option<usize> {
        self.last_move
    }

    pub fn mark_progress(&self) -> f32 {
        if !self.settings.animations_enabled {
            return 1.0;
        }

        progress(self.last_move_at, MARK_ANIMATION)
    }

    pub fn winning_progress(&self) -> f32 {
        if !self.settings.animations_enabled {
            return 1.0;
        }

        progress(self.round_over_at, WIN_ANIMATION)
    }

    pub fn cursor_opacity(&self) -> f32 {
        if !self.settings.animations_enabled {
            return 1.0;
        }

        let phase = self.started.elapsed().as_secs_f32();
        0.65 + 0.35 * (std::f32::consts::TAU * 0.9 * phase).sin().abs()
    }

    pub fn animation_frame_delay(&self) -> Duration {
        if !self.settings.animations_enabled {
            return EVENT_POLL;
        }

        let text_board_animating = self.screen == Screen::Playing && !self.has_images();
        let mark_animating = self.screen == Screen::Playing && self.mark_progress() < 1.0;
        let win_animating =
            self.screen == Screen::RoundOver && self.has_images() && self.winning_progress() < 1.0;

        if text_board_animating || mark_animating || win_animating {
            FRAME_TIME
        } else {
            EVENT_POLL
        }
    }

    pub fn color(&self, token: Token) -> Color {
        crate::theme::color(token, self.color_mode)
    }

    pub fn handle(&mut self, action: Action) -> bool {
        match action {
            Action::ToggleLanguage => {
                self.settings.language = self.settings.language.toggle();
                return true;
            }
            Action::ToggleSound => {
                if audio::available() {
                    self.settings.sound_enabled = !self.settings.sound_enabled;
                    self.audio.set_enabled(self.settings.sound_enabled);
                }
                return true;
            }
            Action::Quit => {
                self.quit = true;
                return true;
            }
            _ => {}
        }

        match self.screen {
            Screen::Home => self.handle_home(action),
            Screen::Difficulty => self.handle_difficulty(action),
            Screen::Playing => self.handle_playing(action),
            Screen::Paused => self.handle_paused(action),
            Screen::RoundOver => self.handle_round_over(action),
        }
    }

    fn handle_home(&mut self, action: Action) -> bool {
        match action {
            Action::Up => self.shift_selection(Selection::Home, -1, HOME_ITEMS),
            Action::Down => self.shift_selection(Selection::Home, 1, HOME_ITEMS),
            Action::Confirm => match self.home_selection {
                0 => {
                    self.difficulty_selection = 0;
                    self.status = None;
                    self.screen = Screen::Difficulty;
                }
                1 => self.start_game(Mode::TwoPlayers),
                _ => self.quit = true,
            },
            Action::Back | Action::QuitKey => self.quit = true,
            _ => return false,
        }

        true
    }

    fn handle_difficulty(&mut self, action: Action) -> bool {
        match action {
            Action::Up => self.shift_selection(Selection::Difficulty, -1, DIFFICULTY_ITEMS),
            Action::Down => self.shift_selection(Selection::Difficulty, 1, DIFFICULTY_ITEMS),
            Action::Confirm => {
                if let Some(&difficulty) = Difficulty::ALL.get(self.difficulty_selection) {
                    self.start_game(Mode::OnePlayer(difficulty));
                } else {
                    self.go_home();
                }
            }
            Action::Back => self.go_home(),
            _ => return false,
        }

        true
    }

    fn handle_playing(&mut self, action: Action) -> bool {
        match action {
            Action::Up => self.move_cursor(-1, 0),
            Action::Down => self.move_cursor(1, 0),
            Action::Left => self.move_cursor(0, -1),
            Action::Right => self.move_cursor(0, 1),
            Action::Confirm => {
                if !self.ai_pending() && self.confirm_allowed() {
                    self.play_selected();
                }
            }
            Action::Back => {
                self.pause_selection = 0;
                self.ai_pending = None;
                self.screen = Screen::Paused;
            }
            _ => return false,
        }

        true
    }

    fn handle_paused(&mut self, action: Action) -> bool {
        match action {
            Action::Up => self.shift_selection(Selection::Pause, -1, PAUSE_ITEMS),
            Action::Down => self.shift_selection(Selection::Pause, 1, PAUSE_ITEMS),
            Action::Confirm => match self.pause_selection {
                0 => self.resume(),
                1 => self.restart_round(),
                2 => self.go_home(),
                _ => self.quit = true,
            },
            Action::Back => self.resume(),
            Action::QuitKey => self.quit = true,
            _ => return false,
        }

        true
    }

    fn handle_round_over(&mut self, action: Action) -> bool {
        match action {
            Action::Up => self.shift_selection(Selection::Result, -1, RESULT_ITEMS),
            Action::Down => self.shift_selection(Selection::Result, 1, RESULT_ITEMS),
            Action::Confirm => match self.result_selection {
                0 => self.rematch(),
                1 => self.go_home(),
                _ => self.quit = true,
            },
            Action::Rematch => self.rematch(),
            Action::Back => self.go_home(),
            _ => return false,
        }

        true
    }

    fn shift_selection(&mut self, selection: Selection, delta: isize, len: usize) {
        let index = match selection {
            Selection::Home => &mut self.home_selection,
            Selection::Difficulty => &mut self.difficulty_selection,
            Selection::Pause => &mut self.pause_selection,
            Selection::Result => &mut self.result_selection,
        };
        let len = len as isize;

        *index = (*index as isize + delta).rem_euclid(len) as usize;
        self.audio.play(Sound::Select);
    }

    fn confirm_allowed(&mut self) -> bool {
        let now = Instant::now();

        if self
            .last_confirm
            .is_some_and(|last| now.duration_since(last) < CONFIRM_DEBOUNCE)
        {
            return false;
        }

        self.last_confirm = Some(now);
        true
    }

    fn move_cursor(&mut self, delta_row: isize, delta_col: isize) {
        if let Some(game) = self.game.as_mut() {
            game.move_selection(delta_row, delta_col);
        }

        self.audio.play(Sound::Move);
    }

    fn play_selected(&mut self) {
        let Some(cell) = self.game.as_ref().map(Game::selected) else {
            return;
        };

        self.play_cell(cell);
    }

    fn play_cell(&mut self, cell: usize) {
        let Some(game) = self.game.as_mut() else {
            return;
        };
        let mark = game.turn();

        match game.play(cell) {
            Ok(outcome) if outcome.is_over() => {
                self.status = None;
                self.result_selection = 0;
                self.screen = Screen::RoundOver;
                let now = Instant::now();
                self.last_move = Some(cell);
                self.last_move_at = Some(now);
                self.round_over_at = Some(now);
                self.audio.play(match outcome {
                    crate::core::Outcome::Draw => Sound::Draw,
                    _ => Sound::Win,
                });
            }
            Ok(_) => {
                self.status = None;
                self.last_move = Some(cell);
                self.last_move_at = Some(Instant::now());
                self.audio.play(if mark == crate::core::Mark::X {
                    Sound::PlaceX
                } else {
                    Sound::PlaceO
                });
            }
            Err(MoveError::GameOver) => {
                self.result_selection = 0;
                self.screen = Screen::RoundOver;
            }
            Err(MoveError::Board(BoardError::Occupied(_))) => {
                self.set_status(StatusKind::Occupied);
                self.audio.play(Sound::Invalid);
            }
            Err(MoveError::Board(BoardError::OutOfBounds(_))) => {}
        }

        self.sync_ai();
    }

    fn start_game(&mut self, mode: Mode) {
        self.game = Some(Game::new(mode));
        self.pause_selection = 0;
        self.result_selection = 0;
        self.status = None;
        self.last_confirm = None;
        self.ai_pending = None;
        self.last_move = None;
        self.last_move_at = None;
        self.round_over_at = None;
        self.screen = Screen::Playing;
        self.sync_ai();
    }

    fn resume(&mut self) {
        self.status = None;
        self.last_confirm = None;
        self.screen = Screen::Playing;
        self.sync_ai();
    }

    fn sync_ai(&mut self) {
        self.ai_pending = None;

        if self.screen != Screen::Playing {
            return;
        }

        let Some(game) = self.game.as_ref() else {
            return;
        };

        if game.is_over() || game.turn() != AI_MARK {
            return;
        }

        if !matches!(game.mode(), Mode::OnePlayer(_)) {
            return;
        }

        let delay = if self.settings.animations_enabled {
            self.rng.range_inclusive(AI_THINK_MIN_MS, AI_THINK_MAX_MS)
        } else {
            AI_THINK_REDUCED_MS
        };

        self.ai_pending = Some(Instant::now() + Duration::from_millis(delay));
    }

    pub fn update_ai(&mut self, now: Instant) {
        let Some(deadline) = self.ai_pending else {
            return;
        };

        if now < deadline {
            return;
        }

        self.ai_pending = None;

        if self.screen != Screen::Playing {
            return;
        }

        let Some(difficulty) = self.game.as_ref().and_then(|game| match game.mode() {
            Mode::OnePlayer(difficulty) if game.turn() == AI_MARK => Some(difficulty),
            _ => None,
        }) else {
            return;
        };

        let cell = match self.game.as_ref() {
            Some(game) => choose_move(game.board(), difficulty, &mut self.rng),
            None => None,
        };

        if let Some(cell) = cell {
            self.play_cell(cell);
        }

        self.sync_ai();
    }

    fn restart_round(&mut self) {
        if let Some(game) = self.game.as_mut() {
            game.restart_round();
        }

        self.last_move = None;
        self.last_move_at = None;
        self.round_over_at = None;
        self.resume();
    }

    fn rematch(&mut self) {
        if let Some(game) = self.game.as_mut() {
            game.rematch();
        }

        self.result_selection = 0;
        self.last_move = None;
        self.last_move_at = None;
        self.round_over_at = None;
        self.resume();
    }

    fn go_home(&mut self) {
        self.game = None;
        self.home_selection = 0;
        self.status = None;
        self.ai_pending = None;
        self.last_move = None;
        self.last_move_at = None;
        self.round_over_at = None;
        self.screen = Screen::Home;
    }

    fn set_status(&mut self, kind: StatusKind) {
        self.status = Some(Status {
            kind,
            expires_at: Instant::now() + STATUS_DURATION,
        });
    }

    pub fn run(&mut self) -> io::Result<()> {
        install_panic_hook();

        let mut guard = TerminalGuard::enter()?;
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        if self.images_enabled {
            let picker = image::create_picker(self.image_protocol);
            *self.image_state.borrow_mut() = Some(ImageState::new(picker));
        }

        self.audio.allow_start();
        self.audio.set_enabled(self.settings.sound_enabled);

        let result = self.event_loop(&mut terminal);

        drop(terminal);
        guard.restore()?;

        result
    }

    fn event_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        while !self.quit {
            self.update_ai(Instant::now());
            terminal.draw(|frame| render::tui::draw(frame, self))?;

            if event::poll(self.animation_frame_delay())? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        if let Some(action) = map_key(key) {
                            self.handle(action);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn progress(started: Option<Instant>, duration: Duration) -> f32 {
    let Some(started) = started else {
        return 1.0;
    };

    (started.elapsed().as_secs_f32() / duration.as_secs_f32()).clamp(0.0, 1.0)
}

#[derive(Debug, Clone, Copy)]
enum Selection {
    Home,
    Difficulty,
    Pause,
    Result,
}

impl App {
    #[cfg(test)]
    pub(crate) fn play_for_test(&mut self, cell: usize) {
        self.play_cell(cell);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Board, Mark, Outcome};

    fn app() -> App {
        let mut app = App::new(Language::PtBr, ColorMode::TrueColor);
        app.set_animations_enabled(false);
        app
    }

    fn start_two_players(app: &mut App) {
        app.handle(Action::Down);
        app.handle(Action::Confirm);
    }

    fn play_win(app: &mut App) {
        for cell in [0, 3, 1, 4, 2] {
            app.play_for_test(cell);
        }
    }

    fn force_ai(app: &mut App) {
        app.ai_pending = Some(Instant::now());
        app.update_ai(Instant::now());
    }

    #[test]
    fn starts_on_home_with_pt_br() {
        let app = app();

        assert_eq!(app.screen(), Screen::Home);
        assert_eq!(app.language(), Language::PtBr);
        assert_eq!(app.sound_enabled(), audio::available());
        assert!(!app.should_quit());
        assert!(app.game().is_none());
    }

    #[test]
    fn f2_toggles_language_without_changing_screen() {
        let mut app = App::new(Language::PtBr, ColorMode::Ansi256);

        assert!(app.handle(Action::ToggleLanguage));

        assert_eq!(app.language(), Language::En);
        assert_eq!(app.screen(), Screen::Home);
        assert_eq!(app.sound_enabled(), audio::available());
    }

    #[test]
    fn m_toggles_sound_without_changing_language() {
        let mut app = App::new(Language::En, ColorMode::Ansi16);

        assert!(app.handle(Action::ToggleSound));

        assert!(!app.sound_enabled());
        assert_eq!(app.language(), Language::En);
    }

    #[test]
    fn home_navigation_wraps_and_quit_sets_flag() {
        let mut app = app();

        app.handle(Action::Up);
        assert_eq!(app.home_selection(), HOME_ITEMS - 1);
        app.handle(Action::Confirm);
        assert!(app.should_quit());
    }

    #[test]
    fn home_enter_one_player_goes_to_difficulty() {
        let mut app = app();

        app.handle(Action::Confirm);

        assert_eq!(app.screen(), Screen::Difficulty);
        assert_eq!(app.difficulty_selection(), 0);
    }

    #[test]
    fn difficulty_enter_starts_one_player_game_with_selected_level() {
        for (index, difficulty) in Difficulty::ALL.into_iter().enumerate() {
            let mut app = app();
            app.handle(Action::Confirm);

            for _ in 0..index {
                app.handle(Action::Down);
            }

            app.handle(Action::Confirm);

            assert_eq!(app.screen(), Screen::Playing);
            assert_eq!(
                app.game().map(Game::mode),
                Some(Mode::OnePlayer(difficulty))
            );
            assert_eq!(app.game().map(Game::turn), Some(Mark::X));
            assert!(!app.ai_pending());
        }
    }

    #[test]
    fn difficulty_back_returns_home_and_last_item_goes_home() {
        let mut app = app();
        app.handle(Action::Confirm);
        app.handle(Action::Back);
        assert_eq!(app.screen(), Screen::Home);

        app.handle(Action::Confirm);
        app.handle(Action::Up);
        assert_eq!(app.difficulty_selection(), DIFFICULTY_ITEMS - 1);
        app.handle(Action::Confirm);
        assert_eq!(app.screen(), Screen::Home);
    }

    #[test]
    fn home_enter_two_players_starts_game() {
        let mut app = app();
        start_two_players(&mut app);

        assert_eq!(app.screen(), Screen::Playing);
        assert_eq!(app.game().map(Game::mode), Some(Mode::TwoPlayers));
        assert_eq!(app.game().map(Game::turn), Some(Mark::X));
    }

    #[test]
    fn esc_on_home_quits() {
        let mut app = app();

        app.handle(Action::Back);

        assert!(app.should_quit());
    }

    #[test]
    fn esc_during_play_opens_pause_and_resumes() {
        let mut app = app();
        start_two_players(&mut app);
        app.handle(Action::Confirm);
        let board = *app.game().unwrap().board();
        let turn = app.game().unwrap().turn();

        app.handle(Action::Back);
        assert_eq!(app.screen(), Screen::Paused);
        app.handle(Action::Back);

        assert_eq!(app.screen(), Screen::Playing);
        assert_eq!(*app.game().unwrap().board(), board);
        assert_eq!(app.game().unwrap().turn(), turn);
    }

    #[test]
    fn pause_continue_returns_to_same_state() {
        let mut app = app();
        start_two_players(&mut app);
        app.handle(Action::Confirm);
        app.handle(Action::Back);

        app.handle(Action::Confirm);

        assert_eq!(app.screen(), Screen::Playing);
        assert_eq!(app.game().unwrap().board().marks_count(), 1);
    }

    #[test]
    fn pause_restart_resets_round_but_keeps_score() {
        let mut app = app();
        start_two_players(&mut app);
        play_win(&mut app);
        assert_eq!(app.screen(), Screen::RoundOver);
        app.handle(Action::Rematch);
        app.handle(Action::Back);
        app.handle(Action::Down);
        app.handle(Action::Confirm);

        assert_eq!(app.screen(), Screen::Playing);
        assert_eq!(app.game().unwrap().score().p1_wins(), 1);
        assert_eq!(app.game().unwrap().score().draws(), 0);
        assert_eq!(*app.game().unwrap().board(), Board::new());
    }

    #[test]
    fn pause_main_menu_resets_session() {
        let mut app = app();
        start_two_players(&mut app);
        app.handle(Action::Back);
        app.handle(Action::Down);
        app.handle(Action::Down);
        app.handle(Action::Confirm);

        assert_eq!(app.screen(), Screen::Home);
        assert!(app.game().is_none());
    }

    #[test]
    fn pause_quit_exits() {
        let mut app = app();
        start_two_players(&mut app);
        app.handle(Action::Back);
        app.handle(Action::Down);
        app.handle(Action::Down);
        app.handle(Action::Down);
        app.handle(Action::Confirm);

        assert!(app.should_quit());
    }

    #[test]
    fn quit_key_is_ignored_while_playing() {
        let mut app = app();
        start_two_players(&mut app);

        app.handle(Action::QuitKey);

        assert!(!app.should_quit());
        assert_eq!(app.screen(), Screen::Playing);
    }

    #[test]
    fn quit_key_exits_from_pause() {
        let mut app = app();
        start_two_players(&mut app);
        app.handle(Action::Back);

        app.handle(Action::QuitKey);

        assert!(app.should_quit());
    }

    #[test]
    fn round_over_rematch_keeps_score_and_alternates_starter() {
        let mut app = app();
        start_two_players(&mut app);
        play_win(&mut app);

        assert_eq!(app.screen(), Screen::RoundOver);
        assert_eq!(app.game().unwrap().score().p1_wins(), 1);
        app.handle(Action::Rematch);

        assert_eq!(app.screen(), Screen::Playing);
        assert_eq!(app.game().unwrap().score().p1_wins(), 1);
        assert_eq!(app.game().unwrap().starter(), Mark::O);
    }

    #[test]
    fn round_over_esc_goes_to_menu() {
        let mut app = app();
        start_two_players(&mut app);
        play_win(&mut app);

        app.handle(Action::Back);

        assert_eq!(app.screen(), Screen::Home);
        assert!(app.game().is_none());
    }

    #[test]
    fn round_over_actions_are_selectable() {
        let mut app = app();
        start_two_players(&mut app);
        play_win(&mut app);
        app.handle(Action::Down);
        assert_eq!(app.result_selection(), 1);

        app.handle(Action::Confirm);

        assert_eq!(app.screen(), Screen::Home);
    }

    #[test]
    fn f2_and_m_do_not_change_game_state() {
        let mut app = app();
        start_two_players(&mut app);
        app.play_for_test(4);
        let board = *app.game().unwrap().board();
        let turn = app.game().unwrap().turn();

        app.handle(Action::ToggleLanguage);
        app.handle(Action::ToggleSound);

        assert_eq!(app.screen(), Screen::Playing);
        assert_eq!(*app.game().unwrap().board(), board);
        assert_eq!(app.game().unwrap().turn(), turn);
    }

    #[test]
    fn repeated_confirm_is_ignored_while_playing() {
        let mut app = app();
        start_two_players(&mut app);
        app.play_for_test(4);
        let turn = app.game().unwrap().turn();
        let marks = app.game().unwrap().board().marks_count();

        app.last_confirm = Some(Instant::now());
        app.handle(Action::Confirm);

        assert_eq!(app.game().unwrap().turn(), turn);
        assert_eq!(app.game().unwrap().board().marks_count(), marks);
    }

    #[test]
    fn one_player_human_turn_schedules_ai_and_blocks_board_input() {
        let mut app = app();
        app.handle(Action::Confirm);
        app.handle(Action::Confirm);
        app.play_for_test(4);

        assert_eq!(app.game().unwrap().turn(), Mark::O);
        assert!(app.ai_pending());

        app.handle(Action::Confirm);

        assert_eq!(app.game().unwrap().board().marks_count(), 1);
        assert!(app.ai_pending());
    }

    #[test]
    fn ai_thinking_delay_stays_within_range() {
        let mut app = App::with_seed(Language::PtBr, ColorMode::TrueColor, 5);
        app.handle(Action::Confirm);
        app.handle(Action::Confirm);
        app.play_for_test(4);

        let remaining = app
            .ai_pending
            .expect("AI should be scheduled")
            .saturating_duration_since(Instant::now());

        assert!(remaining >= Duration::from_millis(449));
        assert!(remaining <= Duration::from_millis(800));
    }

    #[test]
    fn ai_plays_after_deadline_and_returns_turn_to_human() {
        let mut app = app();
        app.handle(Action::Confirm);
        app.handle(Action::Confirm);
        app.play_for_test(4);

        force_ai(&mut app);

        assert_eq!(app.game().unwrap().board().marks_count(), 2);
        assert_eq!(app.game().unwrap().turn(), Mark::X);
        assert!(!app.ai_pending());
    }

    #[test]
    fn pause_cancels_ai_pending_and_resume_reschedules() {
        let mut app = app();
        app.handle(Action::Confirm);
        app.handle(Action::Confirm);
        app.play_for_test(4);

        app.handle(Action::Back);
        assert_eq!(app.screen(), Screen::Paused);
        assert!(!app.ai_pending());

        app.update_ai(Instant::now() + Duration::from_secs(10));
        assert_eq!(app.game().unwrap().board().marks_count(), 1);

        app.handle(Action::Confirm);
        assert_eq!(app.screen(), Screen::Playing);
        assert!(app.ai_pending());

        force_ai(&mut app);
        assert_eq!(app.game().unwrap().board().marks_count(), 2);
    }

    #[test]
    fn one_player_rematch_keeps_human_starting() {
        let mut app = app();
        app.handle(Action::Confirm);
        app.handle(Action::Confirm);
        app.play_for_test(4);
        force_ai(&mut app);

        if app.screen() == Screen::Playing {
            app.handle(Action::Back);
            app.handle(Action::Down);
            app.handle(Action::Confirm);
        }

        app.game.as_mut().unwrap().rematch();
        app.screen = Screen::Playing;
        app.sync_ai();

        assert_eq!(app.game().unwrap().starter(), Mark::X);
        assert_eq!(app.game().unwrap().turn(), Mark::X);
        assert!(!app.ai_pending());
    }

    #[test]
    fn animations_can_be_disabled_without_changing_turns() {
        let mut app = app();
        app.set_animations_enabled(true);
        app.handle(Action::Confirm);
        app.handle(Action::Confirm);
        app.play_for_test(4);

        assert!(app.animations_enabled());
        assert!(app.mark_progress() <= 1.0);
        assert!(app.cursor_opacity() >= 0.0 && app.cursor_opacity() <= 1.0);

        app.set_animations_enabled(false);
        assert!(!app.animations_enabled());
        assert_eq!(app.mark_progress(), 1.0);
        assert_eq!(app.winning_progress(), 1.0);
        assert_eq!(app.cursor_opacity(), 1.0);
        assert_eq!(app.animated_cell(), None);
    }

    #[test]
    fn no_animations_reduces_ai_thinking_delay() {
        let mut app = app();
        app.handle(Action::Confirm);
        app.handle(Action::Confirm);
        app.play_for_test(4);

        let remaining = app
            .ai_pending
            .expect("AI should be scheduled")
            .saturating_duration_since(Instant::now());

        assert!(remaining <= Duration::from_millis(250));
        assert!(remaining > Duration::from_millis(200));
    }

    #[test]
    fn invalid_move_sets_status_for_900ms() {
        let mut app = app();
        start_two_players(&mut app);
        app.play_for_test(4);
        let turn = app.game().unwrap().turn();

        app.play_for_test(4);

        assert_eq!(app.game().unwrap().turn(), turn);
        assert_eq!(app.status(), Some(StatusKind::Occupied));
        assert!(app.status_is_error());
        assert_eq!(app.status_text(), Some("Casa ocupada"));

        app.status.as_mut().unwrap().expires_at = Instant::now();

        assert_eq!(app.status(), None);
        assert_eq!(app.status_text(), None);
    }

    #[test]
    fn board_cursor_moves_without_leaving_board() {
        let mut app = app();
        start_two_players(&mut app);

        app.handle(Action::Left);
        app.handle(Action::Left);
        app.handle(Action::Left);
        assert_eq!(app.game().unwrap().selected(), 3);
        app.handle(Action::Up);
        app.handle(Action::Up);
        assert_eq!(app.game().unwrap().selected(), 0);
    }

    #[test]
    fn draw_outcome_is_recorded_once() {
        let mut app = app();
        start_two_players(&mut app);
        for cell in [0, 4, 8, 1, 2, 6, 3, 5, 7] {
            app.play_for_test(cell);
        }

        assert_eq!(app.screen(), Screen::RoundOver);
        assert_eq!(app.game().unwrap().outcome(), Outcome::Draw);
        assert_eq!(app.game().unwrap().score().draws(), 1);
        app.play_for_test(0);
        assert_eq!(app.game().unwrap().score().draws(), 1);
    }
}
