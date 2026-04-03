# 测试基建与 QA/UAT/E2E 执行手册

## 已完成：测试基建

### Rust 测试栈 ✅

| 工具 | 版本 | 用途 | 状态 |
|------|------|------|------|
| cargo-nextest | 0.9.132 | 主测试执行器 | ✅ 136 passed, 3 skipped |
| cargo-llvm-cov | 0.8.5 | LLVM 覆盖率 | ✅ 36.1% lines (核心模块 >90%) |
| cargo-hack | 0.6.44 | feature powerset | ✅ 18/18 组合通过 |
| cargo-mutants | 27.0.0 | 变异测试 | ✅ 已安装 |
| proptest | 1.11.0 | 属性测试 | ✅ 3 个属性测试 |
| testcontainers | 0.27.2 | Docker 集成测试 | ✅ 已配置 (#[ignore]) |
| tracing-test | 0.2.6 | tracing 断言 | ✅ 3 个 tracing 测试 |
| trybuild | 1.0.116 | 编译错误测试 | ✅ 已配置 (#[ignore]) |
| resolver | 3 | 依赖解析 v3 | ✅ 已升级 |

### 前端测试栈 ✅

| 工具 | 版本 | 用途 | 状态 |
|------|------|------|------|
| Playwright | 1.58.2 | E2E 测试 | ✅ 6 个测试文件 (34 tests) |
| Vitest | 4.1.2 | 单元测试 | ✅ 已配置 |

### 统一入口

```bash
# Rust 测试
./scripts/test/run.sh nextest      # cargo-nextest
./scripts/test/run.sh coverage     # cargo-llvm-cov
./scripts/test/run.sh hack         # cargo-hack
./scripts/test/run.sh mutants      # cargo-mutants
./scripts/test/run.sh quick        # 快速冒烟
./scripts/test/run.sh all          # 全部

# 前端测试
./scripts/test/run-frontend.sh check    # svelte-check
./scripts/test/run-frontend.sh lint     # biome lint
./scripts/test/run-frontend.sh unit     # vitest
./scripts/test/run-frontend.sh e2e      # playwright
./scripts/test/run-frontend.sh all      # 全部

# Just 快捷入口
just test-nextest
just test-coverage
just test-hack
just test-mutants
just test-all-rust
just test-all-frontend
```

---

## 当前测试结果

### Rust 测试 (136 passed, 3 skipped)

```
Summary: 136 tests run: 136 passed, 3 skipped

Skipped (intentional):
  - runtime_server::containers postgres_container_starts      (requires Docker)
  - runtime_server::containers postgres_can_create_and_query  (requires Docker)
  - runtime_server::ui ui_tests                               (add compile-fail cases first)
```

### 覆盖率

```
TOTAL: 36.10% lines (3898 total, 2491 uncovered)

核心覆盖:
  ✅ tenant.rs:        94.64% lines
  ✅ lib.rs:           98.33% lines
  ✅ middleware/tenant: 100.00% lines
  ✅ health.rs:        92.00% lines
  ⚠️  agent.rs:         8.44% lines (需要 HTTP E2E 测试)
  ⚠️  counter.rs:      14.15% lines (需要 HTTP E2E 测试)
  ⚠️  admin.rs:        16.67% lines (需要 HTTP E2E 测试)
```

### Feature Powerset

```
18/18 feature combinations compile successfully
  - runtime_server: default, http3, default+http3
  - All other crates: single feature set
```

---

## Phase 4: QA/UAT/E2E 带跑手册

### 前置准备（一次性）

```bash
# 1. 安装 Playwright 浏览器
bun run --cwd apps/client/web/app playwright install chromium

# 2. 确认 Docker 运行（可选，用于 testcontainers）
docker ps
```

### QA 自动化检查（你来执行，贴结果给我）

| # | 检查项 | 命令 | 预期 | 状态 |
|---|--------|------|------|------|
| QA-1 | Rust 编译 | `cargo check --workspace` | 0 errors | ⏳ |
| QA-2 | Rust 测试 | `cargo nextest run --workspace` | 136 passed | ⏳ |
| QA-3 | 覆盖率 | `cargo llvm-cov --workspace --summary-only` | >35% lines | ⏳ |
| QA-4 | Feature 组合 | `cargo hack check --workspace --feature-powerset` | 全部通过 | ⏳ |
| QA-5 | 前端类型 | `bun run --cwd apps/client/web/app check` | 0 errors | ⏳ |
| QA-6 | 前端 lint | `bun run --cwd apps/client/web/app lint` | 0 errors | ⏳ |

### 服务启动（你执行）

```bash
# 终端 A — 启动后端 API (port 3001)
cargo run -p runtime_server

# 终端 B — 启动 Web 前端 (port 5173)
bun run --cwd apps/client/web/app dev
```

### QA 手工检查（服务启动后）

| # | 检查项 | 命令/操作 | 预期 | 关注点 |
|---|--------|-----------|------|--------|
| QA-7 | 健康检查 | `curl http://localhost:3001/healthz` | 200 `{"status":"ok"}` | 响应时间 <100ms |
| QA-8 | 就绪检查 | `curl http://localhost:3001/readyz` | 200 `{"status":"ready"}` | DB 连接正常 |
| QA-9 | 未认证 401 | `curl -X POST http://localhost:3001/api/tenant/init -H "Content-Type: application/json" -d '{}'` | 401 | 中间件生效 |
| QA-10 | Swagger UI | 浏览器打开 `http://localhost:3001/swagger-ui` | API 文档可见 | OpenAPI 生成正常 |

### UAT 手工验收

| # | 流程 | 操作 | 预期 | 关注点 |
|---|------|------|------|--------|
| UAT-1 | 登录页 | 打开 `http://localhost:5173/login` | Google 登录按钮 + "Welcome back" | 样式、动画正常 |
| UAT-2 | 认证守卫 | 直接访问 `/counter` | 重定向到 `/login` | 守卫生效 |
| UAT-3 | 认证守卫 | 直接访问 `/admin` | 重定向到 `/login` | 守卫生效 |
| UAT-4 | 认证守卫 | 直接访问 `/agent` | 重定向到 `/login` | 守卫生效 |
| UAT-5 | 主题切换 | 点击 Dark mode toggle | 明暗主题切换 | CSS 变量生效 |
| UAT-6 | 响应式 | 窗口缩到 375px | 底部 tab bar 出现 | 移动端布局 |
| UAT-7 | 导航 | 点击侧边栏各链接 | 页面切换正常 | 路由守卫 |

### E2E 自动化执行

```bash
# 确保后端 + 前端都在运行

# 运行 E2E（单浏览器，最快）
bun run --cwd apps/client/web/app test:e2e --project=desktop-chrome

# 运行 E2E（全浏览器）
bun run --cwd apps/client/web/app test:e2e

# 运行单个测试文件
bun run --cwd apps/client/web/app test:e2e tests/e2e/login.test.ts
bun run --cwd apps/client/web/app test:e2e tests/e2e/agent.test.ts

# 查看 HTML 报告
open apps/client/web/app/playwright-report/index.html
```

### E2E 测试覆盖矩阵

| 测试文件 | Tests | 覆盖场景 | 状态 |
|----------|-------|----------|------|
| `login.test.ts` | 5 | 登录页、OAuth mock、响应式 | ✅ |
| `admin.test.ts` | 7 | Admin 面板、认证守卫、统计卡片 | ✅ |
| `counter.test.ts` | 8 | Counter CRUD、守卫、响应式 | ✅ |
| `tenant-isolation.test.ts` | 4 | 多租户隔离、双 context | ✅ |
| `token-refresh.test.ts` | 5 | Token 刷新、过期处理 | ✅ |
| `agent.test.ts` | 7 | Agent 聊天、守卫、SSE 准备 | ✅ 新增 |
| **总计** | **36** | | |

---

## 已知问题与后续优化

### P0 — 阻塞 E2E 通过
- **路由前缀不一致**：后端 `/counter/*` `/admin/*` `/agent/*`，前端调 `/api/counter/*` `/api/admin/*` `/api/agent/*`
  - 影响：E2E 中涉及 API 调用的测试可能 404
  - 修复方案：统一后端路由前缀为 `/api/*`

### P1 — 覆盖率提升
- `agent.rs`: 8.44% → 需要 HTTP E2E 测试覆盖 SSE 流
- `counter.rs`: 14.15% → 需要 HTTP E2E 测试
- `admin.rs`: 16.67% → 需要 HTTP E2E 测试

### P2 — 工具增强
- `cargo mutants`: 待运行（当前测试套件稳定后可跑）
- `testcontainers`: 待 Docker 环境就绪后启用
- `trybuild`: 待添加 compile-fail 测试用例后启用

---

## 执行顺序建议

```
Step 1: 你执行 QA-1 ~ QA-6（自动化检查）
Step 2: 你启动后端 + 前端
Step 3: 你执行 QA-7 ~ QA-10（服务检查）
Step 4: 你执行 UAT-1 ~ UAT-7（手工验收）
Step 5: 你执行 E2E 自动化
Step 6: 把每一步的结果贴给我，我来分析修复
```
