//! Trybuild tests for compile-time error verification.
//!
//! These tests verify that certain code patterns produce the expected
//! compiler errors, ensuring our API is hard to misuse.
//!
//! Run with: cargo test --test ui -- --ignored
//! Update expected output: TRYBUILD=overwrite cargo test --test ui -- --ignored

#[test]
#[ignore = "add compile-fail test cases to tests/ui/ first"]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
