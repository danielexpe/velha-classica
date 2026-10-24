#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Home,
    Difficulty,
    Playing,
    Paused,
    RoundOver,
}

impl Screen {
    pub const fn is_game(self) -> bool {
        matches!(self, Self::Playing | Self::Paused | Self::RoundOver)
    }

    pub const fn is_menu(self) -> bool {
        matches!(self, Self::Home | Self::Difficulty)
    }
}
