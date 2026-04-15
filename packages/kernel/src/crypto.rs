//! Cryptographic and encoding utilities.

use base64::Engine;
use sha2::{Digest, Sha256};

/// Compute SHA-256 hash and return as hex string.
pub fn sha256_hex(input: &[u8]) -> String {
    let hash = Sha256::digest(input);
    hex::encode(hash)
}

/// Compute SHA-256 hash and return as base64url string (no padding).
pub fn sha256_base64url(input: &[u8]) -> String {
    let hash = Sha256::digest(input);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash)
}

/// Generate a random alphanumeric string of given length.
pub fn random_alphanumeric(len: usize) -> String {
    use rand::Rng;
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_deterministic() {
        let a = sha256_hex(b"hello");
        let b = sha256_hex(b"hello");
        assert_eq!(a, b);
    }

    #[test]
    fn sha256_different_input() {
        let a = sha256_hex(b"hello");
        let b = sha256_hex(b"world");
        assert_ne!(a, b);
    }

    #[test]
    fn random_alphanumeric_length() {
        assert_eq!(random_alphanumeric(16).len(), 16);
    }
}
