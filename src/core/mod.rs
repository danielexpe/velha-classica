pub mod board;
pub mod rules;
pub mod score;

pub use board::{Board, BoardError, Mark, BOARD_CELLS, BOARD_SIDE};
pub use rules::{outcome, winner, Outcome, WIN_LINES};
pub use score::SessionScore;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Difficulty {
    #[default]
    Easy,
    Medium,
    Impossible,
}

impl Difficulty {
    pub const ALL: [Self; 3] = [Self::Easy, Self::Medium, Self::Impossible];

    pub const fn next(self) -> Self {
        match self {
            Self::Easy => Self::Medium,
            Self::Medium => Self::Impossible,
            Self::Impossible => Self::Easy,
        }
    }

    pub const fn previous(self) -> Self {
        match self {
            Self::Easy => Self::Impossible,
            Self::Medium => Self::Easy,
            Self::Impossible => Self::Medium,
        }
    }
}
