use std::io::{self, stdout, Stdout};
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::style::Color;
use ratatui::Terminal;

use crate::core::BoardError;
use crate::i18n::{Language, Strings};
use crate::input::{map_key, Action};
use crate::render;
use crate::state::{Game, Mode, MoveError, Screen, Settings};
use crate::theme::{ColorMode, Token};
use crate::util::tty::{install_panic_hook, TerminalGuard};

const STATUS_DURATION: Duration = Duration::from_millis(900);
const CONFIRM_DEBOUNCE: Duration = Duration::from_millis(150);
const HOME_ITEMS: usize = 3;
const DIFFICULTY_ITEMS: usize = 4;
const PAUSE_ITEMS: usize = 4;
const RESULT_ITEMS: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusKind {
    Occupied,
    DifficultyPlaceholder,
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
}

impl App {
    pub fn new(language: Language, color_mode: ColorMode) -> Self {
        Self {
            screen: Screen::Home,
            settings: Settings::new(language),
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

        self.status().map(|kind| match kind {
            StatusKind::Occupied => strings.game_occupied,
            StatusKind::DifficultyPlaceholder => strings.difficulty_placeholder,
        })
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
                self.settings.sound_enabled = !self.settings.sound_enabled;
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
                if self.difficulty_selection < DIFFICULTY_ITEMS - 1 {
                    self.set_status(StatusKind::DifficultyPlaceholder);
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
                if self.confirm_allowed() {
                    self.play_selected();
                }
            }
            Action::Back => {
                self.pause_selection = 0;
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

        match game.play(cell) {
            Ok(outcome) if outcome.is_over() => {
                self.status = None;
                self.result_selection = 0;
                self.screen = Screen::RoundOver;
            }
            Ok(_) => self.status = None,
            Err(MoveError::GameOver) => {
                self.result_selection = 0;
                self.screen = Screen::RoundOver;
            }
            Err(MoveError::Board(BoardError::Occupied(_))) => {
                self.set_status(StatusKind::Occupied);
            }
            Err(MoveError::Board(BoardError::OutOfBounds(_))) => {}
        }
    }

    fn start_game(&mut self, mode: Mode) {
        self.game = Some(Game::new(mode));
        self.pause_selection = 0;
        self.result_selection = 0;
        self.status = None;
        self.last_confirm = None;
        self.screen = Screen::Playing;
    }

    fn resume(&mut self) {
        self.status = None;
        self.last_confirm = None;
        self.screen = Screen::Playing;
    }

    fn restart_round(&mut self) {
        if let Some(game) = self.game.as_mut() {
            game.restart_round();
        }

        self.resume();
    }

    fn rematch(&mut self) {
        if let Some(game) = self.game.as_mut() {
            game.rematch();
        }

        self.result_selection = 0;
        self.resume();
    }

    fn go_home(&mut self) {
        self.game = None;
        self.home_selection = 0;
        self.status = None;
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
        let result = self.event_loop(&mut terminal);

        drop(terminal);
        guard.restore()?;

        result
    }

    fn event_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        while !self.quit {
            terminal.draw(|frame| render::tui::draw(frame, self))?;

            if event::poll(Duration::from_millis(100))? {
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
        App::new(Language::PtBr, ColorMode::TrueColor)
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

    #[test]
    fn starts_on_home_with_pt_br() {
        let app = app();

        assert_eq!(app.screen(), Screen::Home);
        assert_eq!(app.language(), Language::PtBr);
        assert!(app.sound_enabled());
        assert!(!app.should_quit());
        assert!(app.game().is_none());
    }

    #[test]
    fn f2_toggles_language_without_changing_screen() {
        let mut app = App::new(Language::PtBr, ColorMode::Ansi256);

        assert!(app.handle(Action::ToggleLanguage));

        assert_eq!(app.language(), Language::En);
        assert_eq!(app.screen(), Screen::Home);
        assert!(app.sound_enabled());
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
    fn difficulty_enter_shows_phase_three_notice() {
        let mut app = app();
        app.handle(Action::Confirm);
        app.handle(Action::Down);

        app.handle(Action::Confirm);

        assert_eq!(app.screen(), Screen::Difficulty);
        assert_eq!(app.status(), Some(StatusKind::DifficultyPlaceholder));
        assert!(app.game().is_none());
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
