use crate::i18n::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settings {
    pub language: Language,
    pub sound_enabled: bool,
    pub animations_enabled: bool,
}

impl Settings {
    pub const fn new(language: Language) -> Self {
        Self {
            language,
            sound_enabled: true,
            animations_enabled: true,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new(Language::default())
    }
}
