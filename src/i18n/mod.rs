use std::str::FromStr;

use crate::core::Difficulty;

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
    pub menu_choose_difficulty: &'static str,
    pub menu_back: &'static str,
    pub difficulty_easy: &'static str,
    pub difficulty_easy_desc: &'static str,
    pub difficulty_medium: &'static str,
    pub difficulty_medium_desc: &'static str,
    pub difficulty_impossible: &'static str,
    pub difficulty_impossible_desc: &'static str,
    pub difficulty_human_starts: &'static str,
    pub difficulty_placeholder: &'static str,
    pub game_turn_you: &'static str,
    pub game_turn_ai: &'static str,
    pub game_turn_player1: &'static str,
    pub game_turn_player2: &'static str,
    pub game_you: &'static str,
    pub game_ai: &'static str,
    pub game_player1: &'static str,
    pub game_player2: &'static str,
    pub game_draws: &'static str,
    pub game_session_score: &'static str,
    pub game_you_play_x: &'static str,
    pub game_occupied: &'static str,
    pub game_mode_one: &'static str,
    pub game_mode_two: &'static str,
    pub game_difficulty: &'static str,
    pub game_round: &'static str,
    pub result_win_you: &'static str,
    pub result_win_ai: &'static str,
    pub result_win_player1: &'static str,
    pub result_win_player2: &'static str,
    pub result_draw: &'static str,
    pub result_stamp_win: &'static str,
    pub result_stamp_draw: &'static str,
    pub action_resume: &'static str,
    pub action_restart: &'static str,
    pub action_rematch: &'static str,
    pub action_main_menu: &'static str,
    pub action_quit_game: &'static str,
    pub action_back: &'static str,
    pub pause_title: &'static str,
    pub footer_move: &'static str,
    pub footer_select_square: &'static str,
    pub footer_select_menu: &'static str,
    pub footer_navigate: &'static str,
    pub footer_pause: &'static str,
    pub footer_back: &'static str,
    pub footer_rematch: &'static str,
    pub footer_menu: &'static str,
    pub footer_language: &'static str,
    pub footer_sound: &'static str,
    pub footer_quit: &'static str,
    pub language_label: &'static str,
    pub language_pt_br: &'static str,
    pub language_en: &'static str,
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

    pub const fn difficulty_name(&self, difficulty: Difficulty) -> &'static str {
        match difficulty {
            Difficulty::Easy => self.difficulty_easy,
            Difficulty::Medium => self.difficulty_medium,
            Difficulty::Impossible => self.difficulty_impossible,
        }
    }

    pub const fn difficulty_description(&self, difficulty: Difficulty) -> &'static str {
        match difficulty {
            Difficulty::Easy => self.difficulty_easy_desc,
            Difficulty::Medium => self.difficulty_medium_desc,
            Difficulty::Impossible => self.difficulty_impossible_desc,
        }
    }

    pub fn difficulty_status(&self, difficulty: Difficulty) -> String {
        self.game_difficulty
            .replace("{difficulty}", self.difficulty_name(difficulty))
    }

    pub fn round_text(&self, round: u16) -> String {
        self.game_round.replace("{n}", &round.to_string())
    }

    pub fn all(&self) -> Vec<&'static str> {
        vec![
            self.app_title,
            self.app_subtitle,
            self.app_tagline,
            self.menu_one_player,
            self.menu_two_players,
            self.menu_quit,
            self.menu_choose_difficulty,
            self.menu_back,
            self.difficulty_easy,
            self.difficulty_easy_desc,
            self.difficulty_medium,
            self.difficulty_medium_desc,
            self.difficulty_impossible,
            self.difficulty_impossible_desc,
            self.difficulty_human_starts,
            self.difficulty_placeholder,
            self.game_turn_you,
            self.game_turn_ai,
            self.game_turn_player1,
            self.game_turn_player2,
            self.game_you,
            self.game_ai,
            self.game_player1,
            self.game_player2,
            self.game_draws,
            self.game_session_score,
            self.game_you_play_x,
            self.game_occupied,
            self.game_mode_one,
            self.game_mode_two,
            self.game_difficulty,
            self.game_round,
            self.result_win_you,
            self.result_win_ai,
            self.result_win_player1,
            self.result_win_player2,
            self.result_draw,
            self.result_stamp_win,
            self.result_stamp_draw,
            self.action_resume,
            self.action_restart,
            self.action_rematch,
            self.action_main_menu,
            self.action_quit_game,
            self.action_back,
            self.pause_title,
            self.footer_move,
            self.footer_select_square,
            self.footer_select_menu,
            self.footer_navigate,
            self.footer_pause,
            self.footer_back,
            self.footer_rematch,
            self.footer_menu,
            self.footer_language,
            self.footer_sound,
            self.footer_quit,
            self.language_label,
            self.language_pt_br,
            self.language_en,
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
    menu_choose_difficulty: "Escolha a dificuldade",
    menu_back: "Voltar",
    difficulty_easy: "Fácil",
    difficulty_easy_desc: "Joga sem muita estratégia.",
    difficulty_medium: "Médio",
    difficulty_medium_desc: "Aproveita erros e bloqueia ameaças.",
    difficulty_impossible: "Impossível",
    difficulty_impossible_desc: "Nunca perde. Empate já é vitória.",
    difficulty_human_starts: "Você começa com X.",
    difficulty_placeholder: "A IA chega na Fase 3. Jogue em 2 Jogadores.",
    game_turn_you: "Sua vez",
    game_turn_ai: "Vez da IA",
    game_turn_player1: "Vez do Jogador 1",
    game_turn_player2: "Vez do Jogador 2",
    game_you: "Você",
    game_ai: "IA",
    game_player1: "Jogador 1",
    game_player2: "Jogador 2",
    game_draws: "Empates",
    game_session_score: "Placar da sessão",
    game_you_play_x: "Você joga com X",
    game_occupied: "Casa ocupada",
    game_mode_one: "1 Jogador",
    game_mode_two: "2 Jogadores",
    game_difficulty: "Dificuldade: {difficulty}",
    game_round: "Rodada {n}",
    result_win_you: "Você venceu!",
    result_win_ai: "A IA venceu!",
    result_win_player1: "Jogador 1 venceu!",
    result_win_player2: "Jogador 2 venceu!",
    result_draw: "Empate!",
    result_stamp_win: "VITÓRIA",
    result_stamp_draw: "EMPATE",
    action_resume: "Continuar",
    action_restart: "Reiniciar rodada",
    action_rematch: "Revanche",
    action_main_menu: "Menu principal",
    action_quit_game: "Sair do jogo",
    action_back: "Voltar",
    pause_title: "Pausado",
    footer_move: "Setas/WASD mover",
    footer_select_square: "Enter marcar",
    footer_select_menu: "Enter selecionar",
    footer_navigate: "Setas/WASD navegar",
    footer_pause: "Esc pausar",
    footer_back: "Esc voltar",
    footer_rematch: "R revanche",
    footer_menu: "Esc menu",
    footer_language: "F2 idioma",
    footer_sound: "M som",
    footer_quit: "Q sair",
    language_label: "Idioma",
    language_pt_br: "Português",
    language_en: "Inglês",
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
    menu_choose_difficulty: "Choose difficulty",
    menu_back: "Back",
    difficulty_easy: "Easy",
    difficulty_easy_desc: "Plays without much strategy.",
    difficulty_medium: "Medium",
    difficulty_medium_desc: "Punishes mistakes and blocks threats.",
    difficulty_impossible: "Impossible",
    difficulty_impossible_desc: "Never loses. A draw is a win.",
    difficulty_human_starts: "You start as X.",
    difficulty_placeholder: "AI arrives in Phase 3. Play with 2 Players.",
    game_turn_you: "Your turn",
    game_turn_ai: "AI's turn",
    game_turn_player1: "Player 1's turn",
    game_turn_player2: "Player 2's turn",
    game_you: "You",
    game_ai: "AI",
    game_player1: "Player 1",
    game_player2: "Player 2",
    game_draws: "Draws",
    game_session_score: "Session score",
    game_you_play_x: "You play as X",
    game_occupied: "Square occupied",
    game_mode_one: "1 Player",
    game_mode_two: "2 Players",
    game_difficulty: "Difficulty: {difficulty}",
    game_round: "Round {n}",
    result_win_you: "You win!",
    result_win_ai: "The AI wins!",
    result_win_player1: "Player 1 wins!",
    result_win_player2: "Player 2 wins!",
    result_draw: "Draw!",
    result_stamp_win: "WIN",
    result_stamp_draw: "DRAW",
    action_resume: "Resume",
    action_restart: "Restart round",
    action_rematch: "Rematch",
    action_main_menu: "Main menu",
    action_quit_game: "Quit game",
    action_back: "Back",
    pause_title: "Paused",
    footer_move: "Arrows/WASD move",
    footer_select_square: "Enter mark",
    footer_select_menu: "Enter select",
    footer_navigate: "Arrows/WASD navigate",
    footer_pause: "Esc pause",
    footer_back: "Esc back",
    footer_rematch: "R rematch",
    footer_menu: "Esc menu",
    footer_language: "F2 language",
    footer_sound: "M sound",
    footer_quit: "Q quit",
    language_label: "Language",
    language_pt_br: "Portuguese",
    language_en: "English",
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
        assert_eq!(
            strings.difficulty_status(Difficulty::Medium),
            "Dificuldade: Médio"
        );
        assert_eq!(strings.round_text(3), "Rodada 3");
    }

    #[test]
    fn difficulty_names_are_translated() {
        let portuguese = Language::PtBr.strings();
        let english = Language::En.strings();

        assert_eq!(portuguese.difficulty_name(Difficulty::Easy), "Fácil");
        assert_eq!(
            portuguese.difficulty_name(Difficulty::Impossible),
            "Impossível"
        );
        assert_eq!(english.difficulty_name(Difficulty::Easy), "Easy");
        assert_eq!(
            english.difficulty_name(Difficulty::Impossible),
            "Impossible"
        );
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
