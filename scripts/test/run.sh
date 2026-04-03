#!/usr/bin/env bash
# ── Rust Test Suite Runner ─────────────────────────────────────────
# Unified entrypoint for all Rust testing tools.
# Usage: ./scripts/test/run.sh [command] [options]
#
# Commands:
#   nextest          Run tests with cargo-nextest (default)
#   coverage         Run tests with cargo-llvm-cov
#   hack             Run cargo-hack feature powerset check
#   mutants          Run cargo-mutants mutation testing
#   all              Run all test suites sequentially
#   quick            Quick smoke test (unit only, no integration)

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$PROJECT_ROOT"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() { echo -e "${BLUE}[test]${NC} $*"; }
ok()  { echo -e "${GREEN}[✓]${NC} $*"; }
fail() { echo -e "${RED}[✗]${NC} $*"; }
warn() { echo -e "${YELLOW}[!]${NC} $*"; }

# ── Tool availability checks ───────────────────────────────────────
check_tool() {
    if ! command -v "$1" &>/dev/null; then
        warn "$1 not found — install with: $2"
        return 1
    fi
    return 0
}

# ── cargo-nextest ──────────────────────────────────────────────────
run_nextest() {
    log "Running cargo-nextest..."
    if ! check_tool cargo-nextest "cargo install cargo-nextest --locked"; then
        return 1
    fi

    local profile="${PROFILE:-default}"
    log "Profile: $profile"

    cargo nextest run --workspace --profile "$profile" "$@"
    local exit_code=$?

    if [ $exit_code -eq 0 ]; then
        ok "All nextest tests passed"
    else
        fail "nextest tests failed (exit code: $exit_code)"
    fi
    return $exit_code
}

# ── cargo-llvm-cov ─────────────────────────────────────────────────
run_coverage() {
    log "Running cargo-llvm-cov..."
    if ! check_tool cargo-llvm-cov "cargo install cargo-llvm-cov --locked"; then
        return 1
    fi

    local output_format="${COV_FORMAT:-lcov}"
    local output_path="target/lcov.info"

    log "Format: $output_format → $output_path"

    cargo llvm-cov --workspace \
        --"$output_format" \
        --output-path "$output_path" \
        --ignore-filename-regex "tests/" \
        "$@"

    local exit_code=$?

    if [ $exit_code -eq 0 ]; then
        ok "Coverage report generated: $output_path"
        # Show summary
        cargo llvm-cov --workspace --summary-only 2>/dev/null || true
    else
        fail "Coverage run failed (exit code: $exit_code)"
    fi
    return $exit_code
}

# ── cargo-hack ─────────────────────────────────────────────────────
run_hack() {
    log "Running cargo-hack feature powerset..."
    if ! check_tool cargo-hack "cargo install cargo-hack --locked"; then
        return 1
    fi

    cargo hack check --workspace --feature-powerset "$@"
    local exit_code=$?

    if [ $exit_code -eq 0 ]; then
        ok "All feature combinations compile"
    else
        fail "Some feature combinations failed (exit code: $exit_code)"
    fi
    return $exit_code
}

# ── cargo-mutants ──────────────────────────────────────────────────
run_mutants() {
    log "Running cargo-mutants..."
    if ! check_tool cargo-mutants "cargo install cargo-mutants --locked"; then
        return 1
    fi

    cargo mutants --workspace "$@"
    local exit_code=$?

    if [ $exit_code -eq 0 ]; then
        ok "All mutants caught by tests"
    else
        warn "Some mutants survived — tests may need strengthening"
    fi
    return 0  # Non-zero exit is expected behavior, not a failure
}

# ── Quick smoke test ───────────────────────────────────────────────
run_quick() {
    log "Running quick smoke test..."
    cargo check --workspace --quiet && ok "cargo check" || { fail "cargo check"; return 1; }
    cargo test --workspace --lib --quiet && ok "cargo test --lib" || { fail "cargo test --lib"; return 1; }
    ok "Quick smoke test passed"
}

# ── All suites ─────────────────────────────────────────────────────
run_all() {
    local failures=0

    run_quick || ((failures++))
    run_nextest || ((failures++))
    run_coverage || ((failures++))
    run_hack || ((failures++))
    run_mutants || ((failures++))

    echo ""
    log "═══════════════════════════════════════"
    if [ $failures -eq 0 ]; then
        ok "All test suites passed"
    else
        fail "$failures suite(s) had issues"
    fi
    log "═══════════════════════════════════════"

    return $failures
}

# ── Main dispatcher ────────────────────────────────────────────────
case "${1:-nextest}" in
    nextest)   run_nextest "${@:2}" ;;
    coverage)  run_coverage "${@:2}" ;;
    hack)      run_hack "${@:2}" ;;
    mutants)   run_mutants "${@:2}" ;;
    quick)     run_quick ;;
    all)       run_all ;;
    *)
        echo "Usage: $0 {nextest|coverage|hack|mutants|quick|all}"
        echo ""
        echo "Commands:"
        echo "  nextest    Run tests with cargo-nextest (default)"
        echo "  coverage   Run tests with cargo-llvm-cov"
        echo "  hack       Run cargo-hack feature powerset check"
        echo "  mutants    Run cargo-mutants mutation testing"
        echo "  quick      Quick smoke test (unit only)"
        echo "  all        Run all test suites"
        exit 1
        ;;
esac
