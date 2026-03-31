#!/usr/bin/env bash
# Quick Test Script - Fast local verification

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo -e "${YELLOW}🚀 Quick Test Runner${NC}"
echo ""

# Fast checks only
echo "Running fast checks..."
echo ""

# 1. Format check
echo -n "Format check: "
if cargo fmt --all -- --check 2>/dev/null; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${RED}❌${NC}"
fi

# 2. Clippy
echo -n "Clippy: "
if cargo clippy --workspace --all-targets -- -D warnings 2>/dev/null; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${RED}❌${NC}"
fi

# 3. Build
echo -n "Build: "
if cargo build --workspace 2>/dev/null; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${RED}❌${NC}"
fi

# 4. Tests
echo -n "Tests: "
if cargo test --workspace 2>/dev/null; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${RED}❌${NC}"
fi

# 5. Frontend check
echo -n "Frontend check: "
if cd apps/client/web/app && bun run check 2>/dev/null; then
    echo -e "${GREEN}✅${NC}"
else
    echo -e "${RED}❌${NC}"
fi

echo ""
echo "Done!"