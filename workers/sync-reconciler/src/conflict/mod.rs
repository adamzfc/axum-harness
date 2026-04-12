//! Conflict resolver — handles sync conflicts.

/// A sync conflict between source and target.
#[derive(Debug, Clone)]
pub struct SyncConflict {
    pub key: String,
    pub source_value: String,
    pub target_value: String,
}

impl SyncConflict {
    pub fn new(key: &str, source: &str, target: &str) -> Self {
        Self {
            key: key.to_string(),
            source_value: source.to_string(),
            target_value: target.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conflict_creation() {
        let conflict = SyncConflict::new("user:123", "name=Alice", "name=Bob");
        assert_eq!(conflict.key, "user:123");
        assert_eq!(conflict.source_value, "name=Alice");
    }
}
