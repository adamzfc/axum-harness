//! Checkpoint store — tracks the last processed outbox ID.
//!
//! This allows the worker to resume from where it left off
//! after a restart or crash.

use std::sync::atomic::{AtomicU64, Ordering};

/// Stores and retrieves the last processed outbox sequence number.
pub struct CheckpointStore {
    last_processed: AtomicU64,
}

impl CheckpointStore {
    pub fn new(initial: u64) -> Self {
        Self {
            last_processed: AtomicU64::new(initial),
        }
    }

    /// Get the last processed checkpoint.
    pub fn get(&self) -> u64 {
        self.last_processed.load(Ordering::Acquire)
    }

    /// Advance the checkpoint to a new value.
    pub fn advance(&self, new_value: u64) {
        self.last_processed.store(new_value, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkpoint_starts_at_initial() {
        let store = CheckpointStore::new(42);
        assert_eq!(store.get(), 42);
    }

    #[test]
    fn checkpoint_advances_monotonically() {
        let store = CheckpointStore::new(0);
        store.advance(10);
        assert_eq!(store.get(), 10);
        store.advance(25);
        assert_eq!(store.get(), 25);
    }
}
