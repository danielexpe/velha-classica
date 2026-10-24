use super::board::Mark;
use super::rules::Outcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SessionScore {
    p1_wins: u16,
    p2_wins: u16,
    draws: u16,
}

impl SessionScore {
    pub const fn new() -> Self {
        Self {
            p1_wins: 0,
            p2_wins: 0,
            draws: 0,
        }
    }

    pub const fn p1_wins(&self) -> u16 {
        self.p1_wins
    }

    pub const fn p2_wins(&self) -> u16 {
        self.p2_wins
    }

    pub const fn draws(&self) -> u16 {
        self.draws
    }

    pub fn total_rounds(&self) -> u16 {
        self.p1_wins
            .saturating_add(self.p2_wins)
            .saturating_add(self.draws)
    }

    pub fn record(&mut self, outcome: Outcome) -> bool {
        match outcome {
            Outcome::InProgress => return false,
            Outcome::Win { mark: Mark::X, .. } => {
                self.p1_wins = self.p1_wins.saturating_add(1);
            }
            Outcome::Win { mark: Mark::O, .. } => {
                self.p2_wins = self.p2_wins.saturating_add(1);
            }
            Outcome::Draw => {
                self.draws = self.draws.saturating_add(1);
            }
        }

        true
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const X_WIN: Outcome = Outcome::Win {
        mark: Mark::X,
        line: [0, 1, 2],
    };
    const O_WIN: Outcome = Outcome::Win {
        mark: Mark::O,
        line: [3, 4, 5],
    };

    #[test]
    fn win_increments_correct_counter_once() {
        let mut score = SessionScore::new();

        assert!(score.record(X_WIN));
        assert!(score.record(O_WIN));

        assert_eq!(score.p1_wins(), 1);
        assert_eq!(score.p2_wins(), 1);
        assert_eq!(score.draws(), 0);
        assert_eq!(score.total_rounds(), 2);
    }

    #[test]
    fn draw_increments_draws_only() {
        let mut score = SessionScore::new();

        assert!(score.record(Outcome::Draw));

        assert_eq!(score.p1_wins(), 0);
        assert_eq!(score.p2_wins(), 0);
        assert_eq!(score.draws(), 1);
    }

    #[test]
    fn in_progress_outcome_is_not_recorded() {
        let mut score = SessionScore::new();

        assert!(!score.record(Outcome::InProgress));
        assert_eq!(score.total_rounds(), 0);
    }

    #[test]
    fn reset_clears_every_counter() {
        let mut score = SessionScore::new();
        score.record(X_WIN);
        score.record(Outcome::Draw);

        score.reset();

        assert_eq!(score, SessionScore::new());
        assert_eq!(score.total_rounds(), 0);
    }
}
