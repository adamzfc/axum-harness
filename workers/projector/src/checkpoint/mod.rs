//! Projection checkpoint — tracks the last processed event sequence.

use std::sync::atomic::{AtomicU64, Ordering};

/// Tracks the last processed event sequence for resumption.
pub struct ProjectionCheckpoint {
    last_processed: AtomicU64,
}

impl ProjectionCheckpoint {
    pub fn new(initial: u64) -> Self {
        Self {
            last_processed: AtomicU64::new(initial),
        }
    }

    pub fn get(&self) -> u64 {
        self.last_processed.load(Ordering::Acquire)
    }

    pub fn advance(&self, sequence: u64) {
        if sequence > self.get() {
            self.last_processed.store(sequence, Ordering::Release);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkpoint_starts_at_initial() {
        let cp = ProjectionCheckpoint::new(100);
        assert_eq!(cp.get(), 100);
    }

    #[test]
    fn only_advances_forward() {
        let cp = ProjectionCheckpoint::new(100);
        cp.advance(200);
        assert_eq!(cp.get(), 200);
        cp.advance(50); // Should not go backward
        assert_eq!(cp.get(), 200);
    }
}
