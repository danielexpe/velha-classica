use thiserror::Error;

use crate::core::board::BoardError;
use crate::core::{
    outcome, Board, Difficulty, Mark, Outcome, SessionScore, BOARD_CELLS, BOARD_SIDE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    OnePlayer(Difficulty),
    TwoPlayers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum MoveError {
    #[error("the round is already over")]
    GameOver,
    #[error("{0}")]
    Board(#[from] BoardError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    board: Board,
    mode: Mode,
    turn: Mark,
    starter: Mark,
    outcome: Outcome,
    selected: usize,
    score: SessionScore,
    result_recorded: bool,
}

impl Game {
    pub fn new(mode: Mode) -> Self {
        Self {
            board: Board::new(),
            mode,
            turn: Mark::X,
            starter: Mark::X,
            outcome: Outcome::InProgress,
            selected: 4,
            score: SessionScore::new(),
            result_recorded: false,
        }
    }

    pub const fn board(&self) -> &Board {
        &self.board
    }

    pub const fn mode(&self) -> Mode {
        self.mode
    }

    pub const fn turn(&self) -> Mark {
        self.turn
    }

    pub const fn starter(&self) -> Mark {
        self.starter
    }

    pub const fn outcome(&self) -> Outcome {
        self.outcome
    }

    pub const fn selected(&self) -> usize {
        self.selected
    }

    pub const fn score(&self) -> &SessionScore {
        &self.score
    }

    pub const fn is_over(&self) -> bool {
        self.outcome.is_over()
    }

    pub fn select(&mut self, cell: usize) {
        if cell < BOARD_CELLS {
            self.selected = cell;
        }
    }

    pub fn move_selection(&mut self, delta_row: isize, delta_col: isize) {
        let Some((row, col)) = Board::row_col(self.selected) else {
            return;
        };

        let last = BOARD_SIDE as isize - 1;
        let row = (row as isize + delta_row).clamp(0, last) as usize;
        let col = (col as isize + delta_col).clamp(0, last) as usize;

        if let Some(cell) = Board::index(row, col) {
            self.selected = cell;
        }
    }

    pub fn play_selected(&mut self) -> Result<Outcome, MoveError> {
        self.play(self.selected)
    }

    pub fn play(&mut self, cell: usize) -> Result<Outcome, MoveError> {
        if self.outcome.is_over() {
            return Err(MoveError::GameOver);
        }

        self.board.place(cell, self.turn)?;
        self.outcome = outcome(&self.board);

        if self.outcome.is_over() {
            self.record_result();
        } else {
            self.turn = self.turn.other();
        }

        Ok(self.outcome)
    }

    pub fn rematch(&mut self) {
        self.starter = match self.mode {
            Mode::OnePlayer(_) => Mark::X,
            Mode::TwoPlayers => self.starter.other(),
        };
        self.start_round();
    }

    pub fn restart_round(&mut self) {
        self.start_round();
    }

    pub fn reset_session(&mut self) {
        self.score.reset();
        self.starter = Mark::X;
        self.start_round();
    }

    fn start_round(&mut self) {
        self.board.clear();
        self.turn = self.starter;
        self.outcome = Outcome::InProgress;
        self.selected = 4;
        self.result_recorded = false;
    }

    fn record_result(&mut self) {
        if !self.result_recorded {
            self.score.record(self.outcome);
            self.result_recorded = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game_with_moves(mode: Mode, moves: &[usize]) -> Game {
        let mut game = Game::new(mode);

        for &cell in moves {
            game.play(cell).unwrap();
        }

        game
    }

    #[test]
    fn turn_alternates_after_valid_move() {
        let mut game = Game::new(Mode::TwoPlayers);

        assert_eq!(game.turn(), Mark::X);
        game.play(4).unwrap();
        assert_eq!(game.turn(), Mark::O);
        game.play(0).unwrap();
        assert_eq!(game.turn(), Mark::X);
    }

    #[test]
    fn invalid_move_does_not_change_turn() {
        let mut game = Game::new(Mode::TwoPlayers);
        game.play(4).unwrap();
        let turn_before = game.turn();
        let score_before = *game.score();

        assert_eq!(game.play(4), Err(MoveError::Board(BoardError::Occupied(4))));
        assert_eq!(game.turn(), turn_before);
        assert_eq!(*game.score(), score_before);
    }

    #[test]
    fn placing_after_game_over_is_rejected() {
        let mut game = game_with_moves(Mode::TwoPlayers, &[0, 3, 1, 4, 2]);

        assert!(game.is_over());
        assert_eq!(game.play(5), Err(MoveError::GameOver));
    }

    #[test]
    fn double_finish_does_not_double_count() {
        let mut game = game_with_moves(Mode::TwoPlayers, &[0, 3, 1, 4, 2]);

        assert_eq!(game.score().p1_wins(), 1);
        assert_eq!(game.play(5), Err(MoveError::GameOver));
        assert_eq!(game.score().p1_wins(), 1);
    }

    #[test]
    fn rematch_preserves_score() {
        let mut game = game_with_moves(Mode::TwoPlayers, &[0, 3, 1, 4, 2]);

        game.rematch();

        assert_eq!(game.score().p1_wins(), 1);
        assert_eq!(game.outcome(), Outcome::InProgress);
    }

    #[test]
    fn draw_increments_draws_only() {
        let game = game_with_moves(Mode::TwoPlayers, &[0, 4, 8, 1, 2, 6, 3, 5, 7]);

        assert_eq!(game.outcome(), Outcome::Draw);
        assert_eq!(game.score().draws(), 1);
        assert_eq!(game.score().p1_wins(), 0);
        assert_eq!(game.score().p2_wins(), 0);
    }

    #[test]
    fn one_player_human_always_starts() {
        let mut game = Game::new(Mode::OnePlayer(Difficulty::Impossible));

        assert_eq!(game.starter(), Mark::X);
        game.play(0).unwrap();
        game.rematch();

        assert_eq!(game.starter(), Mark::X);
    }

    #[test]
    fn two_player_first_round_x_starts() {
        let game = Game::new(Mode::TwoPlayers);

        assert_eq!(game.starter(), Mark::X);
        assert_eq!(game.turn(), Mark::X);
    }

    #[test]
    fn two_player_rematch_alternates_starter() {
        let mut game = Game::new(Mode::TwoPlayers);

        game.rematch();
        assert_eq!(game.starter(), Mark::O);

        game.rematch();
        assert_eq!(game.starter(), Mark::X);
    }

    #[test]
    fn symbols_do_not_change_between_rounds() {
        let mut game = Game::new(Mode::TwoPlayers);
        game.rematch();
        assert_eq!(game.starter(), Mark::O);

        for cell in [0, 3, 1, 4, 8, 5] {
            game.play(cell).unwrap();
        }

        assert_eq!(
            game.outcome(),
            Outcome::Win {
                mark: Mark::X,
                line: [3, 4, 5]
            }
        );
        assert_eq!(game.score().p1_wins(), 1);
    }

    #[test]
    fn rematch_resets_board_and_selection_to_center() {
        let mut game = game_with_moves(Mode::TwoPlayers, &[0, 4]);
        game.select(8);

        game.rematch();

        assert!(game.board().is_empty());
        assert_eq!(game.selected(), 4);
        assert_eq!(game.outcome(), Outcome::InProgress);
    }

    #[test]
    fn restart_round_keeps_score_and_starter() {
        let mut game = game_with_moves(Mode::TwoPlayers, &[0, 3, 1, 4, 2]);

        assert_eq!(game.score().p1_wins(), 1);
        game.rematch();
        assert_eq!(game.starter(), Mark::O);
        game.play(0).unwrap();
        game.restart_round();

        assert_eq!(game.score().p1_wins(), 1);
        assert_eq!(game.starter(), Mark::O);
        assert!(game.board().is_empty());
        assert_eq!(game.outcome(), Outcome::InProgress);
    }

    #[test]
    fn reset_session_clears_score() {
        let mut game = game_with_moves(Mode::TwoPlayers, &[0, 3, 1, 4, 2]);

        game.reset_session();

        assert_eq!(*game.score(), SessionScore::new());
        assert!(game.board().is_empty());
    }

    #[test]
    fn selection_stops_at_board_edges() {
        let mut game = Game::new(Mode::TwoPlayers);
        game.select(0);

        game.move_selection(-1, -1);
        assert_eq!(game.selected(), 0);

        game.select(8);
        game.move_selection(1, 1);
        assert_eq!(game.selected(), 8);
    }
}
