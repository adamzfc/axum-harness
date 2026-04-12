//! Message deduplication — prevents double-processing of outbox entries.
//!
//! Uses a simple in-memory LRU cache. In production, this would be
//! backed by Redis or similar for cross-instance dedup.

use std::collections::HashSet;
use std::sync::RwLock;

/// Tracks recently processed message IDs to prevent duplicates.
pub struct MessageDedup {
    seen: RwLock<HashSet<String>>,
    max_size: usize,
}

impl MessageDedup {
    /// Create a new dedup store with the given capacity.
    pub fn new(max_size: usize) -> Self {
        Self {
            seen: RwLock::new(HashSet::with_capacity(max_size)),
            max_size,
        }
    }

    /// Check if a message ID has already been processed.
    pub fn is_duplicate(&self, id: &str) -> bool {
        let seen = self.seen.read().unwrap();
        seen.contains(id)
    }

    /// Mark a message ID as processed.
    /// Returns true if this is the first time we've seen this ID.
    pub fn mark_processed(&self, id: &str) -> bool {
        let mut seen = self.seen.write().unwrap();

        // Evict oldest entries if at capacity
        if seen.len() >= self.max_size {
            let to_remove: Vec<String> = seen.iter().take(seen.len() / 4).cloned().collect();
            for key in to_remove {
                seen.remove(&key);
            }
        }

        seen.insert(id.to_string())
    }
}

impl Default for MessageDedup {
    fn default() -> Self {
        Self::new(10_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_message_is_not_duplicate() {
        let dedup = MessageDedup::new(100);
        assert!(!dedup.is_duplicate("msg-1"));
        assert!(dedup.mark_processed("msg-1"));
    }

    #[test]
    fn second_message_is_duplicate() {
        let dedup = MessageDedup::new(100);
        dedup.mark_processed("msg-1");
        assert!(dedup.is_duplicate("msg-1"));
    }

    #[test]
    fn different_messages_not_duplicate() {
        let dedup = MessageDedup::new(100);
        dedup.mark_processed("msg-1");
        assert!(!dedup.is_duplicate("msg-2"));
    }
}
