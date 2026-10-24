use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn from_entropy() -> Self {
        Self::new(entropy_seed())
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        value ^ (value >> 31)
    }

    pub fn below(&mut self, upper: usize) -> usize {
        if upper <= 1 {
            0
        } else {
            (self.next_u64() % upper as u64) as usize
        }
    }

    pub fn range_inclusive(&mut self, low: u64, high: u64) -> u64 {
        if high <= low {
            return low;
        }

        low + self.next_u64() % (high - low + 1)
    }

    pub fn chance(&mut self, percent: u64) -> bool {
        self.next_u64() % 100 < percent
    }
}

pub fn entropy_seed() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as u64)
        .unwrap_or(0);
    let pid = u64::from(std::process::id());

    nanos ^ pid.rotate_left(32) ^ 0x5DEE_CE66_D1A4_F3B1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_produces_same_sequence() {
        let mut first = Rng::new(42);
        let mut second = Rng::new(42);

        for _ in 0..16 {
            assert_eq!(first.next_u64(), second.next_u64());
        }
    }

    #[test]
    fn different_seeds_diverge() {
        let mut first = Rng::new(1);
        let mut second = Rng::new(2);

        assert_ne!(first.next_u64(), second.next_u64());
    }

    #[test]
    fn below_stays_in_range() {
        let mut rng = Rng::new(7);

        for _ in 0..1_000 {
            assert!(rng.below(9) < 9);
        }

        assert_eq!(rng.below(0), 0);
        assert_eq!(rng.below(1), 0);
    }

    #[test]
    fn range_inclusive_respects_bounds() {
        let mut rng = Rng::new(9);

        for _ in 0..1_000 {
            let value = rng.range_inclusive(450, 800);
            assert!((450..=800).contains(&value));
        }

        assert_eq!(rng.range_inclusive(5, 5), 5);
        assert_eq!(rng.range_inclusive(9, 3), 9);
    }

    #[test]
    fn chance_extremes_are_deterministic() {
        let mut rng = Rng::new(11);

        assert!(!rng.chance(0));
        assert!(rng.chance(100));
    }

    #[test]
    fn entropy_seed_is_stable_within_process_call() {
        let seed = entropy_seed();

        assert_ne!(seed, 0);
    }
}
