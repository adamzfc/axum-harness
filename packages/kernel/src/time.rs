//! Time formatting utilities.

use chrono::{DateTime, Utc};

/// Current UTC time as ISO 8601 string.
pub fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

/// Current UTC time as Unix timestamp (seconds).
pub fn now_ts() -> i64 {
    Utc::now().timestamp()
}

/// Format a DateTime as ISO 8601 string.
pub fn format_iso(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn now_iso_is_non_empty() {
        assert!(!now_iso().is_empty());
    }

    #[test]
    fn now_ts_is_positive() {
        assert!(now_ts() > 0);
    }
}
