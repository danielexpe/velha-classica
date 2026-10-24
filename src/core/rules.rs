use super::board::{Board, Mark};

pub const WIN_LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    InProgress,
    Win { mark: Mark, line: [usize; 3] },
    Draw,
}

impl Outcome {
    pub const fn is_over(self) -> bool {
        !matches!(self, Self::InProgress)
    }

    pub const fn winner(self) -> Option<Mark> {
        match self {
            Self::Win { mark, .. } => Some(mark),
            Self::InProgress | Self::Draw => None,
        }
    }

    pub const fn winning_line(self) -> Option<[usize; 3]> {
        match self {
            Self::Win { line, .. } => Some(line),
            Self::InProgress | Self::Draw => None,
        }
    }
}

pub fn winner(board: &Board) -> Option<(Mark, [usize; 3])> {
    WIN_LINES.iter().find_map(|&line| {
        let first = board.get(line[0])?;
        let second = board.get(line[1])?;
        let third = board.get(line[2])?;

        (first == second && second == third).then_some((first, line))
    })
}

pub fn outcome(board: &Board) -> Outcome {
    if let Some((mark, line)) = winner(board) {
        Outcome::Win { mark, line }
    } else if board.is_full() {
        Outcome::Draw
    } else {
        Outcome::InProgress
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board_with(cells: &[(usize, Mark)]) -> Board {
        let mut board = Board::new();

        for &(cell, mark) in cells {
            board.place(cell, mark).unwrap();
        }

        board
    }

    #[test]
    fn detects_all_eight_winning_lines() {
        for line in WIN_LINES {
            let board = board_with(&[(line[0], Mark::X), (line[1], Mark::X), (line[2], Mark::X)]);

            assert_eq!(
                winner(&board),
                Some((Mark::X, line)),
                "line {line:?} was not detected"
            );
            assert_eq!(
                outcome(&board),
                Outcome::Win {
                    mark: Mark::X,
                    line
                }
            );
        }
    }

    #[test]
    fn does_not_report_win_with_two_marks() {
        let board = board_with(&[(0, Mark::X), (1, Mark::X)]);

        assert_eq!(winner(&board), None);
        assert_eq!(outcome(&board), Outcome::InProgress);
    }

    #[test]
    fn does_not_report_win_for_mixed_marks() {
        let board = board_with(&[(0, Mark::X), (1, Mark::O), (2, Mark::X)]);

        assert_eq!(winner(&board), None);
    }

    #[test]
    fn detects_draw_on_full_board_without_winner() {
        let board = board_with(&[
            (0, Mark::X),
            (1, Mark::O),
            (2, Mark::X),
            (3, Mark::X),
            (4, Mark::O),
            (5, Mark::O),
            (6, Mark::O),
            (7, Mark::X),
            (8, Mark::X),
        ]);

        assert_eq!(winner(&board), None);
        assert_eq!(outcome(&board), Outcome::Draw);
    }

    #[test]
    fn winning_line_contains_winning_mark() {
        let board = board_with(&[(3, Mark::O), (4, Mark::O), (5, Mark::O)]);
        let detected = outcome(&board);

        assert_eq!(detected.winner(), Some(Mark::O));

        for cell in detected.winning_line().unwrap() {
            assert_eq!(board.get(cell), Some(Mark::O));
        }
    }

    #[test]
    fn outcome_is_in_progress_on_empty_board() {
        assert_eq!(outcome(&Board::new()), Outcome::InProgress);
        assert!(!outcome(&Board::new()).is_over());
    }
}
