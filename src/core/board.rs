use std::fmt;

use thiserror::Error;

pub const BOARD_SIDE: usize = 3;
pub const BOARD_CELLS: usize = BOARD_SIDE * BOARD_SIDE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mark {
    X,
    O,
}

impl Mark {
    pub const fn other(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }

    pub const fn glyph(self) -> char {
        match self {
            Self::X => 'X',
            Self::O => 'O',
        }
    }
}

impl fmt::Display for Mark {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::X => "X",
            Self::O => "O",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum BoardError {
    #[error("cell index out of bounds: {0}")]
    OutOfBounds(usize),
    #[error("cell is already occupied: {0}")]
    Occupied(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Board {
    cells: [Option<Mark>; BOARD_CELLS],
}

impl Board {
    pub const fn new() -> Self {
        Self {
            cells: [None; BOARD_CELLS],
        }
    }

    pub fn get(&self, cell: usize) -> Option<Mark> {
        self.cells.get(cell).copied().flatten()
    }

    pub fn get_checked(&self, cell: usize) -> Result<Option<Mark>, BoardError> {
        self.cells
            .get(cell)
            .copied()
            .ok_or(BoardError::OutOfBounds(cell))
    }

    pub fn place(&mut self, cell: usize, mark: Mark) -> Result<(), BoardError> {
        let slot = self
            .cells
            .get_mut(cell)
            .ok_or(BoardError::OutOfBounds(cell))?;

        if slot.is_some() {
            return Err(BoardError::Occupied(cell));
        }

        *slot = Some(mark);
        Ok(())
    }

    pub fn cells(&self) -> &[Option<Mark>; BOARD_CELLS] {
        &self.cells
    }

    pub fn is_empty(&self) -> bool {
        self.cells.iter().all(Option::is_none)
    }

    pub fn is_full(&self) -> bool {
        self.cells.iter().all(Option::is_some)
    }

    pub fn marks_count(&self) -> usize {
        self.cells.iter().flatten().count()
    }

    pub fn available_cells(&self) -> impl Iterator<Item = usize> + '_ {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(cell, mark)| mark.is_none().then_some(cell))
    }

    pub fn available(&self) -> Vec<usize> {
        self.available_cells().collect()
    }

    pub fn clear(&mut self) {
        self.cells = [None; BOARD_CELLS];
    }

    pub fn index(row: usize, col: usize) -> Option<usize> {
        (row < BOARD_SIDE && col < BOARD_SIDE).then_some(row * BOARD_SIDE + col)
    }

    pub fn row_col(cell: usize) -> Option<(usize, usize)> {
        (cell < BOARD_CELLS).then_some((cell / BOARD_SIDE, cell % BOARD_SIDE))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_board_is_empty() {
        let board = Board::new();

        assert!(board.is_empty());
        assert!(!board.is_full());
        assert_eq!(board.marks_count(), 0);
    }

    #[test]
    fn place_accepts_empty_cell() {
        let mut board = Board::new();

        assert_eq!(board.place(4, Mark::X), Ok(()));
        assert_eq!(board.get(4), Some(Mark::X));
        assert_eq!(board.marks_count(), 1);
    }

    #[test]
    fn place_rejects_occupied_cell() {
        let mut board = Board::new();
        board.place(4, Mark::X).unwrap();

        assert_eq!(board.place(4, Mark::O), Err(BoardError::Occupied(4)));
        assert_eq!(board.get(4), Some(Mark::X));
        assert_eq!(board.marks_count(), 1);
    }

    #[test]
    fn place_rejects_out_of_bounds() {
        let mut board = Board::new();

        assert_eq!(board.place(9, Mark::X), Err(BoardError::OutOfBounds(9)));
        assert_eq!(board.get_checked(9), Err(BoardError::OutOfBounds(9)));
        assert!(board.is_empty());
    }

    #[test]
    fn available_cells_lists_only_empty() {
        let mut board = Board::new();
        board.place(0, Mark::X).unwrap();
        board.place(4, Mark::O).unwrap();
        board.place(8, Mark::X).unwrap();

        assert_eq!(board.available(), vec![1, 2, 3, 5, 6, 7]);
    }

    #[test]
    fn board_index_row_col_roundtrip() {
        for cell in 0..BOARD_CELLS {
            let (row, col) = Board::row_col(cell).unwrap();

            assert_eq!(Board::index(row, col), Some(cell));
        }

        assert_eq!(Board::index(3, 0), None);
        assert_eq!(Board::row_col(BOARD_CELLS), None);
    }

    #[test]
    fn full_board_has_no_available_cells() {
        let mut board = Board::new();
        let marks = [
            Mark::X,
            Mark::O,
            Mark::X,
            Mark::X,
            Mark::O,
            Mark::O,
            Mark::O,
            Mark::X,
            Mark::X,
        ];

        for (cell, mark) in marks.into_iter().enumerate() {
            board.place(cell, mark).unwrap();
        }

        assert!(board.is_full());
        assert!(board.available().is_empty());
    }
}
