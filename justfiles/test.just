# Testing — all test and coverage commands

# ── Rust 测试 ───────────────────────────────────────────────

# 日常开发：快速单元测试（无覆盖率）
test:
    moon run repo:test-unit

# 使用 nextest 运行测试（推荐）
test-nextest:
    cargo nextest run --workspace

# cargo-hack feature powerset 检查
test-hack:
    cargo hack check --workspace --feature-powerset

# cargo-mutants 变异测试
test-mutants:
    cargo mutants --workspace

# 集成测试（被 --ignore 标记的）
test-integration:
    cargo test --workspace -- --ignored

# 所有 Rust 测试套件（nextest + coverage + hack）
test-all-rust:
    moon run repo:test-all-rust

# ── 覆盖率 ──────────────────────────────────────────────────

# 本地覆盖率：生成 LCOV（供 CI/Codecov 用）
test-coverage:
    just _require cargo-llvm-cov "cargo install cargo-llvm-cov"
    moon run repo:test-coverage

# 本地覆盖率：生成 HTML 并在浏览器打开
test-coverage-html:
    just _require cargo-llvm-cov "cargo install cargo-llvm-cov"
    moon run repo:test-coverage-html

# 清理覆盖率产物
test-coverage-clean:
    just _require cargo-llvm-cov "cargo install cargo-llvm-cov"
    moon run repo:test-coverage-clean

# ── E2E 测试 ────────────────────────────────────────────────

# Web Playwright E2E
test-e2e:
    moon run repo:test-e2e

# 运行仓库全量 E2E 门禁（runtime preflight + Web + Tauri）
test-e2e-full:
    moon run repo:test-e2e-full

# 运行 Tauri 桌面 E2E（tauri-playwright）
test-desktop:
    moon run repo:test-desktop

# 快速运行 E2E（使用 e2e profile,跳过优化编译）
test-desktop-fast:
    cd {{justfile_directory()}} && cargo build -p native-tauri --profile e2e --features e2e-testing
    cd {{justfile_directory()}}/e2e-desktop-playwright && bun run test:desktop:core

# ── 聚合 ────────────────────────────────────────────────────

# 所有前端测试（check + lint + unit + e2e + desktop）
test-all-frontend:
    moon run repo:test-all-frontend
