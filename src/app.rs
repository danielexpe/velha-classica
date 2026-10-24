use std::io::{self, stdout, Stdout};
use std::time::Duration;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::style::Color;
use ratatui::Terminal;

use crate::i18n::{Language, Strings};
use crate::input::{map_key, Action};
use crate::render;
use crate::state::{Screen, Settings};
use crate::theme::{ColorMode, Token};
use crate::util::tty::{install_panic_hook, TerminalGuard};

pub struct App {
    screen: Screen,
    settings: Settings,
    color_mode: ColorMode,
    quit: bool,
    version: &'static str,
}

impl App {
    pub fn new(language: Language, color_mode: ColorMode) -> Self {
        Self {
            screen: Screen::Home,
            settings: Settings::new(language),
            color_mode,
            quit: false,
            version: env!("CARGO_PKG_VERSION"),
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

    pub fn color(&self, token: Token) -> Color {
        crate::theme::color(token, self.color_mode)
    }

    pub fn handle(&mut self, action: Action) -> bool {
        match action {
            Action::ToggleLanguage => {
                self.settings.language = self.settings.language.toggle();
            }
            Action::ToggleSound => {
                self.settings.sound_enabled = !self.settings.sound_enabled;
            }
            Action::Quit => {
                self.quit = true;
            }
        }

        true
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_on_home_with_pt_br() {
        let app = App::new(Language::PtBr, ColorMode::TrueColor);

        assert_eq!(app.screen(), Screen::Home);
        assert_eq!(app.language(), Language::PtBr);
        assert!(app.sound_enabled());
        assert!(!app.should_quit());
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
    fn quit_sets_flag() {
        let mut app = App::new(Language::PtBr, ColorMode::Monochrome);

        assert!(app.handle(Action::Quit));
        assert!(app.should_quit());
    }
}
