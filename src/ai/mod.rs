use crate::core::{outcome, winner, Board, Difficulty, Mark, BOARD_CELLS};
use crate::util::rng::Rng;

pub const HUMAN_MARK: Mark = Mark::X;
pub const AI_MARK: Mark = Mark::O;

const CORNERS: [usize; 4] = [0, 2, 6, 8];
const CENTER: usize = 4;

pub fn choose_move(board: &Board, difficulty: Difficulty, rng: &mut Rng) -> Option<usize> {
    if board.is_full() || outcome(board).is_over() {
        return None;
    }

    match difficulty {
        Difficulty::Easy => easy(board, rng),
        Difficulty::Medium => medium(board, rng),
        Difficulty::Impossible => impossible(board, rng),
    }
}

fn easy(board: &Board, rng: &mut Rng) -> Option<usize> {
    let wins = immediate_wins(board, AI_MARK);

    if !wins.is_empty() && rng.chance(35) {
        return Some(pick(&wins, rng));
    }

    let blocks = immediate_wins(board, HUMAN_MARK);

    if !blocks.is_empty() && rng.chance(35) {
        return Some(pick(&blocks, rng));
    }

    let available = board.available();
    (!available.is_empty()).then(|| pick(&available, rng))
}

fn medium(board: &Board, rng: &mut Rng) -> Option<usize> {
    let wins = immediate_wins(board, AI_MARK);

    if !wins.is_empty() {
        return Some(pick(&wins, rng));
    }

    let blocks = immediate_wins(board, HUMAN_MARK);

    if !blocks.is_empty() {
        return Some(pick(&blocks, rng));
    }

    let available = board.available();

    if available.is_empty() {
        return None;
    }

    if rng.chance(10) {
        return Some(pick(&available, rng));
    }

    let mut best_score = i32::MIN;
    let mut best_moves = Vec::new();

    for cell in available {
        let mut probe = *board;
        probe.place(cell, AI_MARK).ok()?;

        let threats = immediate_wins(&probe, AI_MARK).len();
        let mut score = 0;

        if threats >= 2 {
            score += 800;
        }

        if cell == CENTER {
            score += 300;
        } else if CORNERS.contains(&cell) {
            if board.get(opposite_corner(cell)) == Some(HUMAN_MARK) {
                score += 200;
            }
            score += 100;
        } else {
            score += 50;
        }

        score += rng.below(50) as i32;

        if score > best_score {
            best_score = score;
            best_moves.clear();
            best_moves.push(cell);
        } else if score == best_score {
            best_moves.push(cell);
        }
    }

    (!best_moves.is_empty()).then(|| pick(&best_moves, rng))
}

fn impossible(board: &Board, rng: &mut Rng) -> Option<usize> {
    let mut solver = Solver::default();
    let mut best_score = i32::MIN;
    let mut best_moves = Vec::new();

    for cell in board.available() {
        let mut probe = *board;
        probe.place(cell, AI_MARK).ok()?;

        let score = solver.minimax(&probe, HUMAN_MARK);

        if score > best_score {
            best_score = score;
            best_moves.clear();
            best_moves.push(cell);
        } else if score == best_score {
            best_moves.push(cell);
        }
    }

    (!best_moves.is_empty()).then(|| pick(&best_moves, rng))
}

const MEMO_UNKNOWN: i8 = i8::MIN;
const TERNARY_STATES: usize = 19_683;
const MEMO_SIZE: usize = TERNARY_STATES * 2;

struct Solver {
    memo: Vec<i8>,
}

impl Default for Solver {
    fn default() -> Self {
        Self {
            memo: vec![MEMO_UNKNOWN; MEMO_SIZE],
        }
    }
}

impl Solver {
    fn minimax(&mut self, board: &Board, turn: Mark) -> i32 {
        let key = (encode(board) * 2 + u32::from(turn == AI_MARK)) as usize;

        if self.memo[key] != MEMO_UNKNOWN {
            return i32::from(self.memo[key]);
        }

        let depth = board.marks_count() as i32;
        let score = if let Some((mark, _)) = winner(board) {
            if mark == AI_MARK {
                10 - depth
            } else {
                depth - 10
            }
        } else if board.is_full() {
            0
        } else {
            let mut best = if turn == AI_MARK { -100 } else { 100 };

            for cell in 0..BOARD_CELLS {
                if board.get(cell).is_some() {
                    continue;
                }

                let mut probe = *board;
                probe.place(cell, turn).unwrap();
                let score = self.minimax(&probe, turn.other());

                if turn == AI_MARK {
                    best = best.max(score);
                } else {
                    best = best.min(score);
                }
            }

            best
        };

        self.memo[key] = score as i8;
        score
    }
}

fn encode(board: &Board) -> u32 {
    board.cells().iter().fold(0, |acc, mark| {
        let value = match mark {
            None => 0,
            Some(Mark::X) => 1,
            Some(Mark::O) => 2,
        };
        acc * 3 + value
    })
}

fn immediate_wins(board: &Board, mark: Mark) -> Vec<usize> {
    board
        .available()
        .into_iter()
        .filter(|&cell| {
            let mut probe = *board;
            probe.place(cell, mark).is_ok()
                && winner(&probe).is_some_and(|(found, _)| found == mark)
        })
        .collect()
}

fn pick(cells: &[usize], rng: &mut Rng) -> usize {
    cells[rng.below(cells.len())]
}

const fn opposite_corner(cell: usize) -> usize {
    match cell {
        0 => 8,
        2 => 6,
        6 => 2,
        8 => 0,
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Outcome, BOARD_CELLS};

    fn rng(seed: u64) -> Rng {
        Rng::new(seed)
    }

    fn board_with(cells: &[(usize, Mark)]) -> Board {
        let mut board = Board::new();

        for &(cell, mark) in cells {
            board.place(cell, mark).unwrap();
        }

        board
    }

    fn reference_score(board: &Board, turn: Mark) -> i32 {
        reference_rec(board, turn, &mut std::collections::HashMap::new())
    }

    fn reference_rec(
        board: &Board,
        turn: Mark,
        memo: &mut std::collections::HashMap<u32, i32>,
    ) -> i32 {
        let key = encode(board) * 2 + u32::from(turn == AI_MARK);

        if let Some(&score) = memo.get(&key) {
            return score;
        }

        let depth = board.marks_count() as i32;
        let score = if let Some((mark, _)) = winner(board) {
            if mark == AI_MARK {
                10 - depth
            } else {
                depth - 10
            }
        } else if board.is_full() {
            0
        } else {
            let mut best = if turn == AI_MARK { -100 } else { 100 };

            for cell in board.available() {
                let mut probe = *board;
                probe.place(cell, turn).unwrap();
                let score = reference_rec(&probe, turn.other(), memo);

                if turn == AI_MARK {
                    best = best.max(score);
                } else {
                    best = best.min(score);
                }
            }

            best
        };

        memo.insert(key, score);
        score
    }

    fn random_board(rng: &mut Rng) -> Board {
        let mut board = Board::new();
        let mut turn = HUMAN_MARK;
        let moves = rng.below(BOARD_CELLS + 1);

        for _ in 0..moves {
            if outcome(&board).is_over() {
                break;
            }

            let available = board.available();
            let cell = available[rng.below(available.len())];
            board.place(cell, turn).unwrap();
            turn = turn.other();
        }

        board
    }

    fn assert_legal(board: &Board, cell: Option<usize>) {
        let cell = cell.expect("AI must return a move on a playable board");

        assert!(cell < BOARD_CELLS, "cell {cell} is out of bounds");
        assert_eq!(board.get(cell), None, "AI chose occupied cell {cell}");
    }

    #[test]
    fn easy_returns_legal_move() {
        let board = Board::new();
        let mut rng = rng(1);

        assert_legal(&board, choose_move(&board, Difficulty::Easy, &mut rng));
    }

    #[test]
    fn easy_never_returns_occupied_cell() {
        let mut setup = rng(1234);

        for _ in 0..10_000 {
            let board = random_board(&mut setup);
            let mut rng = rng(setup.next_u64());

            if outcome(&board).is_over() {
                assert_eq!(choose_move(&board, Difficulty::Easy, &mut rng), None);
                continue;
            }

            assert_legal(&board, choose_move(&board, Difficulty::Easy, &mut rng));
        }
    }

    #[test]
    fn easy_is_beatable_over_many_games() {
        let mut human_rng = rng(777);
        let mut ai_rng = rng(778);
        let mut human_wins = 0;

        for _ in 0..500 {
            let mut board = Board::new();

            loop {
                if outcome(&board).is_over() {
                    break;
                }

                let available = board.available();
                let cell = available[human_rng.below(available.len())];
                board.place(cell, HUMAN_MARK).unwrap();

                if outcome(&board).is_over() {
                    break;
                }

                let reply = choose_move(&board, Difficulty::Easy, &mut ai_rng).unwrap();
                board.place(reply, AI_MARK).unwrap();
            }

            if outcome(&board).winner() == Some(HUMAN_MARK) {
                human_wins += 1;
            }
        }

        assert!(
            human_wins > 50,
            "easy should be beatable often, got {human_wins} wins in 500 games"
        );
    }

    #[test]
    fn medium_takes_immediate_win() {
        let board = board_with(&[(0, AI_MARK), (1, AI_MARK), (4, HUMAN_MARK), (8, HUMAN_MARK)]);
        let mut rng = rng(3);

        assert_eq!(choose_move(&board, Difficulty::Medium, &mut rng), Some(2));
    }

    #[test]
    fn medium_blocks_immediate_loss() {
        let board = board_with(&[(0, HUMAN_MARK), (1, HUMAN_MARK), (4, AI_MARK)]);
        let mut rng = rng(4);

        assert_eq!(choose_move(&board, Difficulty::Medium, &mut rng), Some(2));
    }

    #[test]
    fn medium_prefers_center_when_free() {
        let board = Board::new();
        let mut rng = rng(0);

        assert_eq!(
            choose_move(&board, Difficulty::Medium, &mut rng),
            Some(CENTER)
        );
    }

    #[test]
    fn medium_returns_legal_move() {
        let mut setup = rng(55);

        for _ in 0..2_000 {
            let board = random_board(&mut setup);
            let mut rng = rng(setup.next_u64());

            if outcome(&board).is_over() {
                continue;
            }

            assert_legal(&board, choose_move(&board, Difficulty::Medium, &mut rng));
        }
    }

    fn human_move(board: &Board) -> (usize, bool) {
        let wins = immediate_wins(board, HUMAN_MARK);

        if let Some(&cell) = wins.first() {
            return (cell, false);
        }

        let blocks = immediate_wins(board, AI_MARK);

        if let Some(&cell) = blocks.first() {
            return (cell, false);
        }

        let forks: Vec<usize> = board
            .available()
            .into_iter()
            .filter(|&cell| {
                let mut probe = *board;
                probe.place(cell, HUMAN_MARK).unwrap();
                immediate_wins(&probe, HUMAN_MARK).len() >= 2
            })
            .collect();

        if let Some(&cell) = forks.first() {
            return (cell, true);
        }

        for cell in [CENTER, 0, 2, 6, 8, 1, 3, 5, 7] {
            if board.get(cell).is_none() {
                return (cell, false);
            }
        }

        (board.available()[0], false)
    }

    #[test]
    fn medium_can_be_beaten_by_a_fork_with_fixed_seed() {
        let mut rng = rng(24);
        let mut board = Board::new();
        let mut forked = false;

        while !outcome(&board).is_over() {
            let (human, is_fork) = human_move(&board);
            forked |= is_fork;
            board.place(human, HUMAN_MARK).unwrap();

            if outcome(&board).is_over() {
                break;
            }

            let reply = choose_move(&board, Difficulty::Medium, &mut rng).unwrap();
            board.place(reply, AI_MARK).unwrap();
        }

        assert!(forked, "human did not create a fork");
        assert_eq!(outcome(&board).winner(), Some(HUMAN_MARK));
    }

    #[test]
    fn impossible_returns_legal_move() {
        let board = Board::new();
        let mut rng = rng(5);

        assert_legal(
            &board,
            choose_move(&board, Difficulty::Impossible, &mut rng),
        );
    }

    fn explore_human_tree(board: Board, ai_rng: &mut Rng) {
        assert_ne!(outcome(&board).winner(), Some(HUMAN_MARK));

        if outcome(&board).is_over() {
            return;
        }

        for cell in board.available() {
            let mut after_human = board;
            after_human.place(cell, HUMAN_MARK).unwrap();

            if outcome(&after_human).winner() == Some(HUMAN_MARK) {
                panic!("impossible AI allowed human win on cell {cell}");
            }

            if outcome(&after_human).is_over() {
                continue;
            }

            let reply = choose_move(&after_human, Difficulty::Impossible, ai_rng)
                .expect("AI must answer while the game is in progress");
            assert_eq!(after_human.get(reply), None);

            let mut after_ai = after_human;
            after_ai.place(reply, AI_MARK).unwrap();
            explore_human_tree(after_ai, ai_rng);
        }
    }

    #[test]
    fn impossible_never_loses_exhaustive() {
        let mut rng = rng(99);

        explore_human_tree(Board::new(), &mut rng);
    }

    #[test]
    fn impossible_draws_against_perfect_human() {
        let mut ai_rng = rng(21);
        let mut board = Board::new();

        while !outcome(&board).is_over() {
            let mut best_score = 100;
            let mut best_moves = Vec::new();

            for cell in board.available() {
                let mut probe = board;
                probe.place(cell, HUMAN_MARK).unwrap();
                let score = reference_score(&probe, AI_MARK);

                if score < best_score {
                    best_score = score;
                    best_moves.clear();
                    best_moves.push(cell);
                } else if score == best_score {
                    best_moves.push(cell);
                }
            }

            let human = best_moves[0];
            board.place(human, HUMAN_MARK).unwrap();

            if outcome(&board).is_over() {
                break;
            }

            let reply = choose_move(&board, Difficulty::Impossible, &mut ai_rng).unwrap();
            board.place(reply, AI_MARK).unwrap();
        }

        assert_eq!(outcome(&board), Outcome::Draw);
    }

    #[test]
    fn impossible_random_tie_break_still_never_loses() {
        for seed in 0..8 {
            let mut rng = rng(seed);

            explore_human_tree(Board::new(), &mut rng);
        }
    }

    #[test]
    fn impossible_move_is_optimal() {
        let mut setup = rng(2024);

        for _ in 0..500 {
            let board = random_board(&mut setup);

            if outcome(&board).is_over() {
                continue;
            }

            let mut expected = -100;

            for cell in board.available() {
                let mut probe = board;
                probe.place(cell, AI_MARK).unwrap();
                expected = expected.max(reference_score(&probe, HUMAN_MARK));
            }

            let mut rng = rng(setup.next_u64());
            let chosen = choose_move(&board, Difficulty::Impossible, &mut rng).unwrap();
            let mut probe = board;
            probe.place(chosen, AI_MARK).unwrap();
            let actual = reference_score(&probe, HUMAN_MARK);

            assert_eq!(actual, expected, "suboptimal move {chosen} on {board:?}");
        }
    }

    #[test]
    fn impossible_move_completes_within_5ms() {
        let board = Board::new();
        let mut rng = rng(8);
        let start = std::time::Instant::now();

        let _ = choose_move(&board, Difficulty::Impossible, &mut rng);
        let elapsed = start.elapsed();
        let limit = if cfg!(debug_assertions) { 50 } else { 5 };

        assert!(
            elapsed < std::time::Duration::from_millis(limit),
            "first impossible move took {elapsed:?}"
        );
    }

    #[test]
    fn seeded_ai_is_reproducible() {
        let boards = [
            Board::new(),
            board_with(&[(4, HUMAN_MARK)]),
            board_with(&[(0, HUMAN_MARK), (4, AI_MARK), (8, HUMAN_MARK)]),
        ];

        for board in boards {
            for difficulty in Difficulty::ALL {
                let mut first = rng(123);
                let mut second = rng(123);

                assert_eq!(
                    choose_move(&board, difficulty, &mut first),
                    choose_move(&board, difficulty, &mut second),
                    "{difficulty:?} was not reproducible"
                );
            }
        }
    }

    #[test]
    fn easy_and_medium_legal_across_random_boards() {
        let mut setup = rng(4242);

        for _ in 0..5_000 {
            let board = random_board(&mut setup);

            if outcome(&board).is_over() {
                continue;
            }

            for difficulty in [Difficulty::Easy, Difficulty::Medium] {
                let mut rng = rng(setup.next_u64());
                assert_legal(&board, choose_move(&board, difficulty, &mut rng));
            }
        }
    }

    #[test]
    fn finished_board_has_no_move() {
        let board = board_with(&[
            (0, HUMAN_MARK),
            (1, AI_MARK),
            (2, HUMAN_MARK),
            (3, HUMAN_MARK),
            (4, AI_MARK),
            (5, AI_MARK),
            (6, AI_MARK),
            (7, HUMAN_MARK),
            (8, HUMAN_MARK),
        ]);

        for difficulty in Difficulty::ALL {
            let mut rng = rng(1);
            assert_eq!(choose_move(&board, difficulty, &mut rng), None);
        }
    }
}
