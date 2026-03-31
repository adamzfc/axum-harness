#!/usr/bin/env bash
# Test Verification Script - Run all quality checks locally

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Quality Gate Verification${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Track results
TOTAL_CHECKS=0
PASSED_CHECKS=0

run_check() {
    local name="$1"
    local cmd="$2"
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    
    echo -e "${YELLOW}[$TOTAL_CHECKS] $name${NC}"
    echo "  Command: $cmd"
    
    if eval "$cmd" > /dev/null 2>&1; then
        echo -e "  ${GREEN}✅ PASSED${NC}"
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
    else
        echo -e "  ${RED}❌ FAILED${NC}"
    fi
    echo ""
}

# ============================================
# Phase 1: Rust Code Quality
# ============================================
echo -e "${BLUE}=== Phase 1: Rust Code Quality ===${NC}"
echo ""

run_check "Rust Format Check" "cargo fmt --all -- --check"
run_check "Rust Clippy Lint" "cargo clippy --workspace --all-targets -- -D warnings"
run_check "Rust Build" "cargo build --workspace --release"

# ============================================
# Phase 2: Tests
# ============================================
echo -e "${BLUE}=== Phase 2: Tests ===${NC}"
echo ""

run_check "Rust Unit Tests" "cargo test --workspace --all-features"
run_check "Rust Doc Tests" "cargo test --workspace --doc"

# ============================================
# Phase 3: Frontend
# ============================================
echo -e "${BLUE}=== Phase 3: Frontend ===${NC}"
echo ""

run_check "Frontend Type Check" "cd apps/client/web/app && bun run check"
run_check "Frontend Lint" "cd apps/client/web/app && bun run lint"
run_check "Frontend Unit Tests" "cd apps/client/web/app && bun run test:unit"

# ============================================
# Summary
# ============================================
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  Summary${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "Total Checks: $TOTAL_CHECKS"
echo -e "Passed: ${GREEN}$PASSED_CHECKS${NC}"
echo -e "Failed: ${RED}$((TOTAL_CHECKS - PASSED_CHECKS))${NC}"
echo ""

if [ "$PASSED_CHECKS" -eq "$TOTAL_CHECKS" ]; then
    echo -e "${GREEN}✅ All quality gates passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ Some quality gates failed!${NC}"
    exit 1
fi