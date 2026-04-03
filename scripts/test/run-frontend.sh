#!/usr/bin/env bash
# ── Frontend Test Suite Runner ─────────────────────────────────────
# Unified entrypoint for all frontend testing tools.
# Usage: ./scripts/test/run-frontend.sh [command]
#
# Commands:
#   check      TypeScript/svelte-check
#   lint       Biome lint
#   unit       Vitest unit tests
#   e2e        Playwright E2E tests
#   all        Run all frontend checks

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
WEB_DIR="$PROJECT_ROOT/apps/client/web/app"

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log() { echo -e "${BLUE}[frontend]${NC} $*"; }
ok()  { echo -e "${GREEN}[✓]${NC} $*"; }
fail() { echo -e "${RED}[✗]${NC} $*"; }

run_check() {
    log "Running svelte-check..."
    bun run --cwd "$WEB_DIR" check
    ok "Type check passed"
}

run_lint() {
    log "Running biome lint..."
    bun run --cwd "$WEB_DIR" lint
    ok "Lint passed"
}

run_unit() {
    log "Running vitest unit tests..."
    bun run --cwd "$WEB_DIR" test:unit
    ok "Unit tests passed"
}

run_e2e() {
    log "Running Playwright E2E tests..."
    local project="${1:-}"
    if [ -n "$project" ]; then
        bun run --cwd "$WEB_DIR" test:e2e --project="$project"
    else
        bun run --cwd "$WEB_DIR" test:e2e
    fi
    ok "E2E tests passed"
}

run_all() {
    local failures=0
    run_check || ((failures++))
    run_lint || ((failures++))
    run_unit || ((failures++))
    run_e2e || ((failures++))

    echo ""
    log "═══════════════════════════════════════"
    if [ $failures -eq 0 ]; then
        ok "All frontend checks passed"
    else
        fail "$failures check(s) had issues"
    fi
    log "═══════════════════════════════════════"
    return $failures
}

case "${1:-all}" in
    check) run_check ;;
    lint)  run_lint ;;
    unit)  run_unit ;;
    e2e)   run_e2e "${2:-}" ;;
    all)   run_all ;;
    *)
        echo "Usage: $0 {check|lint|unit|e2e|all}"
        exit 1
        ;;
esac
