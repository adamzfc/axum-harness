//! ID generation utilities.

/// Generate a URL-safe unique ID (UUID v7).
pub fn new_id() -> String {
    uuid::Uuid::now_v7().to_string()
}

/// Generate a short ID (first 8 chars of UUID).
pub fn short_id() -> String {
    let id = new_id();
    id[..8].to_string()
}

/// Generate a correlation ID for request tracing.
pub fn correlation_id() -> String {
    format!("req-{}", short_id())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_id_is_unique() {
        let a = new_id();
        let b = new_id();
        assert_ne!(a, b);
    }

    #[test]
    fn short_id_length() {
        assert_eq!(short_id().len(), 8);
    }

    #[test]
    fn correlation_id_prefix() {
        assert!(correlation_id().starts_with("req-"));
    }
}
