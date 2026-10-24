use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    ToggleLanguage,
    ToggleSound,
    Quit,
}

pub fn map_key(key: KeyEvent) -> Option<Action> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Some(Action::Quit);
    }

    match key.code {
        KeyCode::F(2) => Some(Action::ToggleLanguage),
        KeyCode::Char('m' | 'M') => Some(Action::ToggleSound),
        KeyCode::Char('q' | 'Q') | KeyCode::Esc => Some(Action::Quit),
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
        assert_eq!(map_key(key(KeyCode::Char('q'))), Some(Action::Quit));
        assert_eq!(map_key(key(KeyCode::Esc)), Some(Action::Quit));
    }

    #[test]
    fn maps_ctrl_c_to_quit() {
        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);

        assert_eq!(map_key(ctrl_c), Some(Action::Quit));
    }

    #[test]
    fn ignores_unmapped_keys() {
        assert_eq!(map_key(key(KeyCode::Up)), None);
        assert_eq!(map_key(key(KeyCode::Enter)), None);
        assert_eq!(map_key(key(KeyCode::Char('z'))), None);
    }
}
