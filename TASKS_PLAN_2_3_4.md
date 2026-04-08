# 任务文档：方案 2/3/4 执行计划

> 创建时间：2026-04-08
> 前置条件：方案 1 已完成并推送到 main
> 执行方式：按顺序执行，每个方案完成后需用户确认

---

## 方案 2：修复 Justfile 的 setup-sccache 不污染系统环境变量

### 问题描述

当前 `Justfile` 中的 `setup-sccache` 任务使用 `setx` 命令写入 Windows 系统环境变量：
```just
setup-sccache:
    @echo "SCCACHE_DIR=D:\dev-storage\cache\sccache" | setx >nul 2>&1 || true
    @echo "RUSTC_WRAPPER=sccache" | setx >nul 2>&1 || true
```

**问题**：
1. `setx` 会写入 Windows 注册表，永久修改系统环境变量
2. 影响范围超出项目，切换分支/项目后仍然生效
3. 硬编码路径 `D:\dev-storage\cache\sccache` 假设特定机器布局
4. 无法通过 `git clean` 或项目重置恢复

### 执行步骤

#### 步骤 1：修改 Justfile 中的 setup-sccache

**文件**：`Justfile`

**修改前**：
```just
setup-sccache:
    @bun -e "const { spawnSync } = require('child_process'); const r = spawnSync(process.platform === 'win32' ? 'where' : 'which', ['sccache'], { shell: process.platform === 'win32' }); if (r.status !== 0) { console.log('Installing sccache via cargo...'); spawnSync('cargo', ['install', 'sccache', '--locked'], { stdio: 'inherit', shell: process.platform === 'win32' }); } else { console.log('sccache already installed: ' + spawnSync('sccache', ['--version'], { shell: process.platform === 'win32' }).stdout?.toString().trim()); }"
    @echo "Setting up sccache environment..."
    @echo "SCCACHE_DIR=D:\dev-storage\cache\sccache" | setx >nul 2>&1 || true
    @echo "RUSTC_WRAPPER=sccache" | setx >nul 2>&1 || true
    @echo "sccache configured — restart terminal or run: just setup-sccache-verify"
```

**修改后**：
```just
setup-sccache:
    @echo "=== Installing sccache ==="
    @bun -e "const { spawnSync } = require('child_process'); const r = spawnSync(process.platform === 'win32' ? 'where' : 'which', ['sccache'], { shell: process.platform === 'win32' }); if (r.status !== 0) { console.log('Installing sccache...'); spawnSync('cargo', ['install', 'sccache', '--locked'], { stdio: 'inherit', shell: process.platform === 'win32' }); } else { console.log('sccache already installed'); }"
    @echo ""
    @echo "=== sccache 安装完成 ==="
    @echo ""
    @echo "请配置环境变量（二选一）："
    @echo ""
    @echo "方案 A - 项目级 .env（推荐）："
    @echo "  1. 复制 .env.example 到 .env"
    @echo "  2. 取消注释 sccache 相关行"
    @echo ""
    @echo "方案 B - mise 环境变量："
    @echo "  mise set SCCACHE_DIR=<项目根目录>/.sccache"
    @echo "  mise set RUSTC_WRAPPER=sccache"
    @echo ""
    @echo "配置完成后运行：just setup-sccache-verify"
```

#### 步骤 2：更新 setup-sccache-verify

**修改前**：
```just
setup-sccache-verify:
    @echo "=== sccache status ==="
    @sccache --show-stats
    @echo ""
    @echo "=== .cargo/config.toml check ==="
    @bun -e "const fs = require('fs'); const c = fs.readFileSync('.cargo/config.toml', 'utf8'); console.log(c.includes('rustc-wrapper = \"sccache\"') ? '✓ rustc-wrapper enabled' : '✗ rustc-wrapper NOT set');"
```

**修改后**：
```just
setup-sccache-verify:
    @echo "=== sccache 验证 ==="
    @echo ""
    @echo "--- sccache 状态 ---"
    @bun -e "const { spawnSync } = require('child_process'); const r = spawnSync('sccache', ['--show-stats'], { stdio: 'inherit', shell: process.platform === 'win32' });"
    @echo ""
    @echo "--- 环境变量检查 ---"
    @bun -e "const wrapper = process.env.RUSTC_WRAPPER; console.log(wrapper === 'sccache' ? '✓ RUSTC_WRAPPER=sccache' : wrapper ? '⚠ RUSTC_WRAPPER=' + wrapper + ' (应该是 sccache)' : '✗ RUSTC_WRAPPER 未设置');"
    @echo ""
    @echo "--- .cargo/config.toml 检查 ---"
    @bun -e "const fs = require('fs'); const c = fs.readFileSync('.cargo/config.toml', 'utf8'); console.log(c.includes('rustc-wrapper') ? '✓ rustc-wrapper 已配置' : '✗ rustc-wrapper 未设置');"
```

#### 步骤 3：更新 .env.example

在 `.env.example` 中确保有 sccache 配置模板：
```env
# ── Build Cache (sccache) ──────────────────────────────────
# Install: scoop install sccache (Windows) or cargo install sccache --locked
# Cache location: 默认 ~/.cache/sccache 或自定义路径
# SCCACHE_DIR=./.sccache
# RUSTC_WRAPPER=sccache
```

### 验证方式

```bash
# 1. 运行安装提示
just setup-sccache

# 2. 手动配置 .env
echo "RUSTC_WRAPPER=sccache" >> .env

# 3. 验证配置
just setup-sccache-verify

# 4. 实际编译验证
cargo build -p runtime_server
sccache --show-stats  # 应显示缓存命中
```

### 影响评估

- **正面**：不再污染系统环境变量，项目隔离更好
- **风险**：低，只是配置方式改变
- **回滚**：`git revert <commit>`，如果已执行 `setx` 需手动清理：`setx SCCACHE_DIR ""`

---

## 方案 3：清理 moon.yml 中的占位符任务和重复项

### 问题描述

`moon.yml` 中存在：
1. **14 个占位符任务**：只输出 `echo "XXX — Phase N implementation"`，无实际功能
2. **dev-desktop-win 重复项**：与 `dev-desktop` 完全相同

### 需要删除的任务列表

#### 占位符任务（14 个）
```yaml
dev-extension:
  command: 'echo "Extension dev not yet implemented"'

dev-workers:
  command: 'echo "Workers dev not yet implemented"'

test-agent:
  command: 'echo "Agent tests — Phase 5 implementation"'

openapi-gen:
  command: 'echo "OpenAPI gen — Phase 2 implementation"'

fixtures-gen:
  command: 'echo "Fixtures gen — Phase 5 implementation"'

icons-gen:
  command: 'echo "Icons gen — as needed"'

tokens-gen:
  command: 'echo "Tokens gen — as needed"'

trace-open:
  command: 'echo "Trace open — Phase 7 implementation"'

evals-run:
  command: 'echo "Evals run — Phase 5 implementation"'

replay-protocol:
  command: 'echo "Replay protocol — Phase 7 implementation"'

release-dry-run:
  command: 'echo "Release dry run — Phase 9 implementation"'

release-desktop:
  command: 'echo "Release desktop — Phase 9 implementation"'

release-web:
  command: 'echo "Release web — Phase 9 implementation"'

release-server:
  command: 'echo "Release server — Phase 9 implementation"'

secrets-scan:
  command: 'echo "Secrets scan - use gitleaks or similar"'

licenses-check:
  command: 'echo "Licenses check - Phase 9 implementation"'
```

#### 重复任务（1 个）
```yaml
dev-desktop_win:  # 与 dev-desktop 完全相同
  script: 'bun run scripts/dev-desktop.ts'
  inputs:
    - 'apps/client/native/**/*'
    - 'servers/api/**/*'
    - 'scripts/dev-desktop.ts'
```

### 执行步骤

#### 步骤 1：删除占位符任务

从 `moon.yml` 中删除上述 14 个占位符任务

#### 步骤 2：删除 dev-desktop_win 重复项

删除 `dev-desktop_win` 任务，在 `dev-desktop` 添加注释说明跨平台兼容

#### 步骤 3：更新 Justfile（如有引用）

检查 Justfile 是否引用了这些任务，如有则同步删除

### 验证方式

```bash
# 1. 验证 moon.yml 语法正确
moon task repo:doctor

# 2. 验证任务列表清爽
moon task repo:verify  # 应正常显示依赖图

# 3. 验证核心功能不受影响
just dev
just verify
just typegen
```

### 影响评估

- **正面**：`moon task` 输出更清晰，减少认知负担
- **风险**：极低，占位符本身无功能
- **回滚**：`git revert <commit>`，风险低

### 后续建议

占位符任务代表的功能可以在真正需要时再实现，建议：
- 记录到 `docs/roadmap.md` 或项目 Issue 中
- 实现时直接在 `moon.yml` 添加真实任务

---

## 方案 4：简化 Justfile 中 1:1 的 moon 代理配方

### 问题描述

Justfile 中有大量配方只是简单转发到 moon，没有增加任何抽象价值：
```just
test-nextest:
    moon run repo:test-nextest

test-coverage:
    moon run repo:test-coverage
```

**原则**：Justfile 应该是**稳定接口层**，不是 moon 的复读机

### 执行步骤

#### 步骤 1：删除纯代理配方（9 个）

```just
# 删除以下配方（用户可直接用 moon run repo:xxx 调用）
test-nextest:
    moon run repo:test-nextest

test-e2e:
    moon run repo:test-e2e

test-hack:
    moon run repo:test-hack

test-mutants:
    moon run repo:test-mutants

test-all-rust:
    moon run repo:test-all-rust

test-all-frontend:
    moon run repo:test-all-frontend

fmt:
    moon run repo:fmt

lint:
    moon run repo:lint

typegen:
    moon run repo:typegen
```

#### 步骤 2：保留有价值的配方

**保留理由分类**：

1. **有前置检查**：
```just
test-coverage:
    just _require cargo-llvm-cov "cargo install cargo-llvm-cov"
    moon run repo:test-coverage

test-coverage-html:
    just _require cargo-llvm-cov "cargo install cargo-llvm-cov"
    moon run repo:test-coverage-html
```

2. **有用户友好别名**：
```just
dev:                  # 好记：just dev
dev-web:              # 比 moon run repo:dev-web 简洁
dev-api:
dev-desktop:
verify:               # 完整质量门禁
```

3. **有特殊逻辑**：
```just
test-desktop-fast:    # 有两步编译逻辑
deploy-api:           # 有 systemd 集成
```

#### 步骤 3：更新文档

更新 README 或 docs 中引用这些命令的示例

### 验证方式

```bash
# 1. 验证保留的配方正常工作
just dev
just verify
just test-coverage

# 2. 验证删除的功能可通过 moon 直接调用
moon run repo:test-nextest
moon run repo:lint
moon run repo:typegen

# 3. 验证质量门禁
just verify
```

### 影响评估

- **正面**：Justfile 更精简，职责更清晰
- **风险**：低，用户需要改用 `moon run repo:xxx` 调用部分命令
- **回滚**：`git revert <commit>`，风险低

### 迁移指南（提供给用户）

| 旧命令 | 新命令 |
|--------|--------|
| `just test-nextest` | `moon run repo:test-nextest` |
| `just test-e2e` | `moon run repo:test-e2e` |
| `just test-hack` | `moon run repo:test-hack` |
| `just fmt` | `moon run repo:fmt` |
| `just lint` | `moon run repo:lint` |

---

## 执行顺序建议

1. **方案 2**（修复 sccache）→ 配置管理优化，风险低
2. **方案 3**（清理占位符）→ 净化 moon.yml，无功能影响
3. **方案 4**（简化代理）→ 需要文档同步，影响用户习惯

每个方案执行后：
- 运行 `just verify` 确保质量门禁通过
- 运行 `git status && git diff` 确认更改
- 提交并推送

---

## 回滚方案

任何方案出现问题，可快速回滚：

```bash
# 查看最近提交
git log --oneline -5

# 回滚单个方案
git revert <commit-hash>

# 硬回滚（如果还没 push）
git reset --hard HEAD~1
```

所有操作都是**删除或简化**，不涉及功能重写，回滚风险极低。
