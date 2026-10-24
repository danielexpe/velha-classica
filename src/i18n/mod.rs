use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Language {
    #[default]
    PtBr,
    En,
}

impl Language {
    pub const fn toggle(self) -> Self {
        match self {
            Self::PtBr => Self::En,
            Self::En => Self::PtBr,
        }
    }

    pub const fn code(self) -> &'static str {
        match self {
            Self::PtBr => "pt",
            Self::En => "en",
        }
    }

    pub const fn strings(self) -> &'static Strings {
        match self {
            Self::PtBr => &PT_BR,
            Self::En => &EN,
        }
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_lowercase().as_str() {
            "pt" | "pt-br" | "pt_br" | "portugues" | "português" => Ok(Self::PtBr),
            "en" | "en-us" | "en_us" | "english" | "ingles" | "inglês" => Ok(Self::En),
            other => Err(format!("unknown language: {other}")),
        }
    }
}

pub struct Strings {
    pub app_title: &'static str,
    pub app_subtitle: &'static str,
    pub app_tagline: &'static str,
    pub menu_one_player: &'static str,
    pub menu_two_players: &'static str,
    pub menu_quit: &'static str,
    pub footer_language: &'static str,
    pub footer_sound: &'static str,
    pub footer_quit: &'static str,
    pub language_label: &'static str,
    pub language_pt_br: &'static str,
    pub language_en: &'static str,
    pub placeholder_title: &'static str,
    pub placeholder_body: &'static str,
    pub sound_on: &'static str,
    pub sound_off: &'static str,
    pub terminal_too_small_prefix: &'static str,
    pub terminal_too_small_suffix: &'static str,
    pub terminal_resize_hint: &'static str,
}

impl Strings {
    pub fn version_text(&self, version: &str) -> String {
        format!("v{version}")
    }

    pub fn language_name(&self, language: Language) -> &'static str {
        match language {
            Language::PtBr => self.language_pt_br,
            Language::En => self.language_en,
        }
    }

    pub fn language_status(&self, language: Language) -> String {
        format!("{}: {}", self.language_label, self.language_name(language))
    }

    pub fn sound_status(&self, enabled: bool) -> &'static str {
        if enabled {
            self.sound_on
        } else {
            self.sound_off
        }
    }

    pub fn terminal_too_small(&self, cols: u16, rows: u16) -> String {
        format!(
            "{}{cols}×{rows}{}",
            self.terminal_too_small_prefix, self.terminal_too_small_suffix
        )
    }

    pub fn all(&self) -> [&'static str; 19] {
        [
            self.app_title,
            self.app_subtitle,
            self.app_tagline,
            self.menu_one_player,
            self.menu_two_players,
            self.menu_quit,
            self.footer_language,
            self.footer_sound,
            self.footer_quit,
            self.language_label,
            self.language_pt_br,
            self.language_en,
            self.placeholder_title,
            self.placeholder_body,
            self.sound_on,
            self.sound_off,
            self.terminal_too_small_prefix,
            self.terminal_too_small_suffix,
            self.terminal_resize_hint,
        ]
    }
}

pub static PT_BR: Strings = Strings {
    app_title: "Velha Clássica",
    app_subtitle: "Jogo da Velha",
    app_tagline: "Mesa de Madeira",
    menu_one_player: "1 Jogador",
    menu_two_players: "2 Jogadores",
    menu_quit: "Sair",
    footer_language: "F2 idioma",
    footer_sound: "M som",
    footer_quit: "Q sair",
    language_label: "Idioma",
    language_pt_br: "Português",
    language_en: "Inglês",
    placeholder_title: "Fundação pronta",
    placeholder_body: "Fase 1 · esqueleto visual",
    sound_on: "Som ligado",
    sound_off: "Som desligado",
    terminal_too_small_prefix: "Terminal muito pequeno. Mínimo: ",
    terminal_too_small_suffix: ".",
    terminal_resize_hint: "Aumente a janela e tente novamente.",
};

pub static EN: Strings = Strings {
    app_title: "Velha Clássica",
    app_subtitle: "Tic-Tac-Toe",
    app_tagline: "Wood & Paper",
    menu_one_player: "1 Player",
    menu_two_players: "2 Players",
    menu_quit: "Quit",
    footer_language: "F2 language",
    footer_sound: "M sound",
    footer_quit: "Q quit",
    language_label: "Language",
    language_pt_br: "Portuguese",
    language_en: "English",
    placeholder_title: "Foundation ready",
    placeholder_body: "Phase 1 · visual skeleton",
    sound_on: "Sound on",
    sound_off: "Sound off",
    terminal_too_small_prefix: "Terminal too small. Minimum: ",
    terminal_too_small_suffix: ".",
    terminal_resize_hint: "Resize the window and try again.",
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_language_is_pt_br() {
        assert_eq!(Language::default(), Language::PtBr);
    }

    #[test]
    fn toggle_round_trip() {
        assert_eq!(Language::PtBr.toggle(), Language::En);
        assert_eq!(Language::En.toggle(), Language::PtBr);
        assert_eq!(Language::PtBr.toggle().toggle(), Language::PtBr);
    }

    #[test]
    fn from_str_accepts_aliases() {
        assert_eq!("pt".parse::<Language>(), Ok(Language::PtBr));
        assert_eq!("pt-BR".parse::<Language>(), Ok(Language::PtBr));
        assert_eq!("PT_BR".parse::<Language>(), Ok(Language::PtBr));
        assert_eq!("en".parse::<Language>(), Ok(Language::En));
        assert_eq!(" EN-us ".parse::<Language>(), Ok(Language::En));
    }

    #[test]
    fn from_str_rejects_unknown() {
        assert!("fr".parse::<Language>().is_err());
        assert!("".parse::<Language>().is_err());
    }

    #[test]
    fn all_translations_are_non_empty() {
        for language in [Language::PtBr, Language::En] {
            for text in language.strings().all() {
                assert!(!text.trim().is_empty(), "{language:?} has an empty string");
            }
        }
    }

    #[test]
    fn formatted_text_uses_requested_values() {
        let strings = Language::PtBr.strings();

        assert_eq!(
            strings.terminal_too_small(80, 24),
            "Terminal muito pequeno. Mínimo: 80×24."
        );
        assert_eq!(strings.language_status(Language::En), "Idioma: Inglês");
        assert_eq!(strings.version_text("1.0.0"), "v1.0.0");
        assert_eq!(strings.sound_status(false), "Som desligado");
    }

    #[test]
    fn english_strings_are_translated() {
        let strings = Language::En.strings();

        assert_eq!(strings.language_status(Language::En), "Language: English");
        assert_eq!(
            strings.terminal_too_small(80, 24),
            "Terminal too small. Minimum: 80×24."
        );
    }
}
