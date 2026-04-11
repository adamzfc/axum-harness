# Quality — fmt, lint, typecheck, verify, contracts

# 格式化代码（Rust + Web）
fmt:
    moon run repo:fmt

# 修复格式
fmt-fix:
    moon run repo:format-fix

# Lint（Rust clippy + Web biome）
lint:
    moon run repo:quality-lint

# 类型检查（Rust check + Web svelte-check）
typecheck:
    moon run repo:typecheck

# 合约类型漂移检查（typegen + drift detection）
contracts-check:
    moon run repo:contracts-check

# 生成合约类型绑定
typegen:
    moon run repo:typegen

# 架构边界检查（features / contracts 依赖约束）
boundary-check:
    moon run repo:boundary-check

# 完整质量门禁（fmt + lint + check + test）
verify:
    moon run repo:verify

# 生成目录分类索引（Agent 搜索优先级提示）
gen-dir-cats:
    bun run scripts/gen-directory-categories.ts

# ── 内部工具 ──────────────────────────────────────────────────

_require TOOL CMD:
    @bun -e "const { spawnSync } = require('child_process'); const t = '{{TOOL}}'; const r = spawnSync(process.platform === 'win32' ? 'where' : 'which', [t], { shell: process.platform === 'win32' }); if (r.status !== 0) { console.log('Missing {{TOOL}} — install with: {{CMD}}'); process.exit(1); }"
