use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    ToggleLanguage,
    ToggleSound,
    Quit,
    QuitKey,
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Back,
    Rematch,
}

pub fn map_key(key: KeyEvent) -> Option<Action> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Some(Action::Quit);
    }

    match key.code {
        KeyCode::F(2) => Some(Action::ToggleLanguage),
        KeyCode::Char('m' | 'M') => Some(Action::ToggleSound),
        KeyCode::Char('q' | 'Q') => Some(Action::QuitKey),
        KeyCode::Esc => Some(Action::Back),
        KeyCode::Enter | KeyCode::Char(' ') => Some(Action::Confirm),
        KeyCode::Char('r' | 'R') => Some(Action::Rematch),
        KeyCode::Up | KeyCode::Char('w' | 'W') => Some(Action::Up),
        KeyCode::Down | KeyCode::Char('s' | 'S') => Some(Action::Down),
        KeyCode::Left | KeyCode::Char('a' | 'A') => Some(Action::Left),
        KeyCode::Right | KeyCode::Char('d' | 'D') => Some(Action::Right),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn maps_global_keys() {
        assert_eq!(map_key(key(KeyCode::F(2))), Some(Action::ToggleLanguage));
        assert_eq!(map_key(key(KeyCode::Char('m'))), Some(Action::ToggleSound));
        assert_eq!(map_key(key(KeyCode::Char('M'))), Some(Action::ToggleSound));
        assert_eq!(map_key(key(KeyCode::Char('q'))), Some(Action::QuitKey));
        assert_eq!(map_key(key(KeyCode::Esc)), Some(Action::Back));
    }

    #[test]
    fn maps_ctrl_c_to_quit() {
        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);

        assert_eq!(map_key(ctrl_c), Some(Action::Quit));
    }

    #[test]
    fn maps_navigation_confirm_and_rematch() {
        for code in [KeyCode::Up, KeyCode::Char('w'), KeyCode::Char('W')] {
            assert_eq!(map_key(key(code)), Some(Action::Up));
        }
        for code in [KeyCode::Down, KeyCode::Char('s'), KeyCode::Char('S')] {
            assert_eq!(map_key(key(code)), Some(Action::Down));
        }
        for code in [KeyCode::Left, KeyCode::Char('a'), KeyCode::Char('A')] {
            assert_eq!(map_key(key(code)), Some(Action::Left));
        }
        for code in [KeyCode::Right, KeyCode::Char('d'), KeyCode::Char('D')] {
            assert_eq!(map_key(key(code)), Some(Action::Right));
        }
        assert_eq!(map_key(key(KeyCode::Enter)), Some(Action::Confirm));
        assert_eq!(map_key(key(KeyCode::Char(' '))), Some(Action::Confirm));
        assert_eq!(map_key(key(KeyCode::Char('r'))), Some(Action::Rematch));
        assert_eq!(map_key(key(KeyCode::Char('R'))), Some(Action::Rematch));
    }

    #[test]
    fn ignores_unmapped_keys() {
        assert_eq!(map_key(key(KeyCode::Tab)), None);
        assert_eq!(map_key(key(KeyCode::Char('z'))), None);
    }
}
