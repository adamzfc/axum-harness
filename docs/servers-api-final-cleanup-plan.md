# servers/api 最终重构收尾计划

> **创建时间**: 2026-04-12
> **前置条件**: Phase 0-3 已完成（web-bff 和 admin-bff 功能完整，gateway 路由更新）
> **目标**: 安全删除 servers/api，更新所有外部引用

---

## 📊 当前状态概览

### 已完成 ✅
- web-bff (3010) 完全替代 servers/api 的所有业务端点
- admin-bff (3020) 功能完整
- Gateway (3000) 正确路由到所有 upstream
- 21 个测试已迁移到 web-bff 和 admin-bff
- `Cargo.toml` 中 servers/api 已标记为 DEPRECATED
- `openapi.yaml` 已迁移到 `docs/contracts/api-routes.yaml`

### 待解决 ⚠️
- **54+ 处外部引用**仍指向 servers/api 或 runtime_server
- **E2E 测试链路**依赖启动 runtime_server
- **基础设施配置**（Docker、K8s）仍定义 api-server
- **测试覆盖差距**：servers/api 有 6+ 测试文件，web-bff 仅有 1 个

---

## 🎯 执行计划（按优先级排序）

### Phase 4.1: 修复 E2E 测试链路（最关键）

**风险**: 🔴 高 — 不修复将破坏所有 E2E 测试

#### 4.1.1 更新 Web E2E 测试夹具

**文件**: `apps/web/tests/fixtures/runtime.ts`

**当前行为**: 启动 `cargo run -p runtime_server` (端口 3001)

**应改为**: 启动 `cargo run -p web-bff` (端口 3010)

**改动**:
```typescript
// 替换所有 runtime_server 为 web-bff
const binary = process.platform === 'win32' ? 'web-bff' : 'web-bff';
ownedApiProcess = spawn('cargo', ['run', '-p', 'web-bff'], {
  // ... 保持其他配置不变
});
// 更新健康检查端口从 3001 到 3010
```

**验证**: `cd apps/web && bun run test:e2e` 应能通过

---

#### 4.1.2 更新 Desktop E2E 租户隔离测试

**文件**: `apps/desktop/tests/e2e/tests/specs/tenant-isolation.spec.ts`

**当前行为**: 在 L125-152 启动 `runtime_server`

**应改为**: 启动 `web-bff`

**改动**:
```typescript
// L125: 更新二进制名称
const binary = process.platform === 'win32' ? 'web-bff.exe' : 'web-bff';

// L142: 更新启动命令
ownedApiProcess = spawn('cargo', ['run', '-p', 'web-bff'], {
  // ... 保持其他配置
});

// L152: 健康检查端口改为 3010
```

**验证**: `cd apps/desktop/tests/e2e && bun run test:desktop:core` 应能通过

---

#### 4.1.3 更新 E2E 预检查脚本

**文件**: `scripts/e2e/runtime-preflight.ts`

**当前行为**: 检查端口 3001 的 runtime_server 是否就绪

**应改为**: 检查端口 3010 的 web-bff

**改动**:
```typescript
// L99: 更新错误消息
'start web-bff first: rtk cargo run -p web-bff',

// L123: 更新启动日志检查
'confirm web-bff startup logs and retry'
```

---

#### 4.1.4 更新 dev-desktop 脚本

**文件**: `scripts/dev-desktop.ts`

**当前行为**: L102 启动 `cargo run -p runtime_server`

**应改为**: 启动 `cargo run -p web-bff`

**改动**:
```typescript
// L102
apiProcess = spawn("cargo", ["run", "-p", "web-bff"], {
  // ... 保持其他配置
});
```

---

### Phase 4.2: 更新构建和任务配置

**风险**: 🟡 中 — 影响开发工作流

#### 4.2.1 更新根 moon.yml

**文件**: `moon.yml`

**改动**:
```yaml
dev-desktop:
  script: 'bun run scripts/dev-desktop.ts'
  inputs:
    - 'apps/desktop/**/*'
    # 删除这行:
    # - 'servers/api/**/*'
    # 替换为:
    - 'servers/bff/web-bff/**/*'
    - 'scripts/dev-desktop.ts'
    - 'scripts/lib/spawn.ts'
```

---

#### 4.2.2 更新 justfiles/test.just

**文件**: `justfiles/test.just`

**当前**: L75-84 有 `test-api-e2e` 和 `test-api-e2e-only` 引用 servers/api 路径

**应改为**: 更新为 web-bff 测试路径或删除（如果已有等效测试）

**改动**:
```just
# 选项 1: 更新路径
test-api-e2e:
    bun {{justfile_directory()}}/servers/bff/web-bff/tests/e2e/run-web-e2e.ts

# 选项 2: 删除（如果测试已迁移到其他地方）
# 删除 test-api-e2e 和 test-api-e2e-only
```

---

#### 4.2.3 更新 justfiles/migrate.just

**文件**: `justfiles/migrate.just`

**当前**: L21, L29, L50 引用 `servers/api/.data/runtime_server.db`

**应改为**: 使用 web-bff 的数据库路径

**改动**:
```just
# L21
@echo "Database: servers/bff/web-bff/.data/web-bff.db"

# L29
sqlite3 servers/bff/web-bff/.data/web-bff.db ".tables" 2>/dev/null || echo "  (database not yet created)"; \

# L50
@rm -f servers/bff/web-bff/.data/web-bff.db
```

---

#### 4.2.4 更新 justfiles/deploy.just

**文件**: `justfiles/deploy.just`

**当前**: L72 有 `cargo check -p runtime_server`

**应改为**: 
```just
cargo check -p web-bff -p admin-bff
```

---

#### 4.2.5 更新 justfiles/processes.just

**文件**: `justfiles/processes.just`

**当前**: L40-47 有停止 runtime_server 的命令

**应改为**:
```bash
# L40 (PowerShell)
powershell -NoProfile -Command "Get-Process | Where-Object { $_.CommandLine -match 'cargo run' -or $_.ProcessName -match 'web-bff' } | Stop-Process -Force -ErrorAction SilentlyContinue" 2>/dev/null; echo "  [Web BFF stopped]"; \

# L46-47 (Unix)
pkill -f "cargo run -p web-bff" 2>/dev/null && echo "  ✓ Web BFF stopped" || echo "  - Web BFF not running"; \
pkill -f "cargo run -p admin-bff" 2>/dev/null && echo "  ✓ Admin BFF stopped" || echo "  - Admin BFF not running"; \
```

---

### Phase 4.3: 更新基础设施配置

**风险**: 🟡 中 — 影响部署流程

#### 4.3.1 更新 Docker Compose 配置

**文件**: `infra/docker/compose.dev.yml`

**当前**: L20-26 定义 api 服务使用 servers/api

**应改为**: 替换为 web-bff 和 admin-bff

**改动**:
```yaml
services:
  # 删除旧的 api 服务
  # api:
  #   build:
  #     context: ../..
  #     dockerfile: infra/docker/Dockerfile.api
  #   ...

  # 新增 web-bff 服务
  web-bff:
    build:
      context: ../..
      dockerfile: infra/docker/Dockerfile.web-bff
    ports:
      - "3010:3010"
    environment:
      - WEB_BFF_DATABASE__URL=file:data/web-bff.db
      - RUST_LOG=info,web-bff=debug
    volumes:
      - web-bff-data:/app/servers/bff/web-bff/.data

  # 新增 admin-bff 服务
  admin-bff:
    build:
      context: ../..
      dockerfile: infra/docker/Dockerfile.admin-bff
    ports:
      - "3020:3020"
    environment:
      - ADMIN_BFF_DATABASE__URL=file:data/admin-bff.db
      - RUST_LOG=info,admin-bff=debug
    volumes:
      - admin-bff-data:/app/servers/bff/admin-bff/.data

volumes:
  web-bff-data:
  admin-bff-data:
```

---

#### 4.3.2 创建 Dockerfile.web-bff

**文件**: `infra/docker/Dockerfile.web-bff`（新建）

**基于**: `infra/docker/Dockerfile.api` 修改

**关键改动**:
```dockerfile
# 构建阶段
FROM rust:1.85-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --package web-bff

# 运行阶段
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/web-bff /app/web-bff

ENV RUST_LOG=info,web-bff=debug
EXPOSE 3010

CMD ["/app/web-bff"]
```

---

#### 4.3.3 创建 Dockerfile.admin-bff

**文件**: `infra/docker/Dockerfile.admin-bff`（新建）

**类似 Dockerfile.web-bff，但针对 admin-bff**

---

#### 4.3.4 更新 Podman Compose

**文件**: `infra/docker/compose/app.yaml`

**改动**: 类似 compose.dev.yml，替换 api 服务为 web-bff 和 admin-bff

---

#### 4.3.5 更新 Kubernetes 配置

**需要更新的文件**:
1. `infra/k3s/base/deployment-api.yaml` → 重命名为 `deployment-web-bff.yaml` 和 `deployment-admin-bff.yaml`
2. `infra/k3s/base/service.yaml` → 更新选择器
3. `infra/k3s/base/ingress.yaml` → 更新后端服务
4. `infra/k3s/overlays/dev/kustomization.yaml` → 删除 runtime_server 日志补丁
5. `infra/k3s/overlays/prod/hpa-api.yaml` → 重命名为 `hpa-web-bff.yaml`

**关键改动**:
- 镜像名称从 `api-server` 改为 `web-bff` / `admin-bff`
- 端口从 3001 改为 3010 / 3020
- 环境变量前缀从 `APP_` 改为 `WEB_BFF_` / `ADMIN_BFF_`
- 日志目标从 `runtime_server=trace` 改为 `web-bff=debug,admin-bff=debug`

---

### Phase 4.4: 更新文档和注释

**风险**: 🟢 低 — 纯文档更新

#### 4.4.1 更新环境配置示例

**文件**: `.env.example`

**改动**: 删除或更新 L30-31 的 servers/api 特定注释

---

#### 4.4.2 更新包文档

**文件**: 
- `packages/core/domain/src/ports/surreal_db.rs` (L3-14)
- `packages/core/domain/src/lib.rs` (L4)
- `packages/adapters/telemetry/README.md` (L16)
- `packages/adapters/telemetry/otel/src/lib.rs` (L4)

**改动**: 将 `runtime_server` 引用改为 `web-bff` 或 `servers/bff/`

---

#### 4.4.3 更新 BFF 文档

**文件**: `servers/bff/OPENAPI-STRATEGY.md`

**改动**: 将 `servers/api/openapi.yaml` 引用改为 `docs/contracts/api-routes.yaml`

---

#### 4.4.4 更新 Agent 模板

**文件**: `agent/templates/bff-endpoint/README.md`

**改动**: L11 删除 `servers/api` 引用

---

#### 4.4.5 更新脚本引用

**文件**: `scripts/gen-directory-categories.ts`

**改动**: L140 删除历史引用

---

### Phase 4.5: 测试覆盖补充

**风险**: 🟡 中 — 影响测试质量

#### 4.5.1 审查并迁移高价值测试

**从 servers/api/tests/ 迁移**:

| 测试文件 | 优先级 | 动作 |
|---------|-------|------|
| `http_e2e_test.rs` (928 行) | 🔴 高 | 已部分迁移，审查是否有遗漏 |
| `integration_test.rs` | 🟡 中 | 迁移 SQL 注入防护和 proptest 测试 |
| `tracing_test.rs` | 🟢 低 | 仅在需要时迁移 |
| `containers.rs` | ❌ 不迁移 | 过于通用，可放弃 |
| `ui.rs` + `ui/placeholder.rs` | ❌ 不迁移 | 空占位符，可放弃 |
| `tests/e2e/collection.json` | 🟡 中 | 审查是否有端点覆盖缺口 |

---

#### 4.5.2 添加缺失的测试场景

基于原始 servers/api 测试，检查 web-bff 是否缺失：

- [ ] JWT 过期行为测试
- [ ] CORS 预检完整头测试
- [ ] 计数器租户隔离测试
- [ ] SQL 注入弹性测试（已完成）
- [ ] 特殊字符处理测试（已完成）

---

### Phase 4.6: 最终删除

**风险**: 🔴 高 — 不可逆操作

#### 4.6.1 删除前检查清单

```bash
# 1. 确认无编译时引用
cargo check --workspace 2>&1 | grep -i "runtime_server"
# 应该无输出

# 2. 确认无运行时引用
grep -r "runtime_server" --include="*.rs" --include="*.ts" --include="*.yaml" --include="*.yml" --include="*.just" .
# 应该只有文档和已弃用标记

# 3. 确认所有 E2E 测试通过
just test-e2e

# 4. 确认所有单元测试通过
cargo test --workspace

# 5. 确认网关路由正常
# 手动测试: curl http://localhost:3000/healthz
# 应该返回所有 upstream 的配置状态
```

---

#### 4.6.2 执行删除

```bash
# 1. 从 Cargo.toml 删除注释（已注释掉的部分）
# 编辑 Cargo.toml，完全删除 servers/api 相关行

# 2. 删除 servers/api 目录
rm -rf servers/api/

# 3. 删除 servers/api 的 moon.yml（已无用）
# 已随目录删除

# 4. 清理 Cargo.lock
cargo update -p runtime_server --precise 0.0.0 2>/dev/null || true
cargo check --workspace  # 重新生成 lock

# 5. 删除平台模型中的弃用标记
rm platform/model/deployables/api-server.yaml

# 6. 提交删除
git add -A
git commit -m "refactor(servers): delete servers/api - fully replaced by web-bff and admin-bff

- Remove servers/api directory (runtime_server package)
- Clean up Cargo.toml workspace members
- Remove deprecated api-server deployable
- All endpoints migrated to web-bff (3010) and admin-bff (3020)
- Gateway (3000) routes updated: /api/* → web-bff, /admin/* → admin-bff
- 21 tests migrated and passing

Co-authored-by: Qwen-Coder <qwen-coder@alibabacloud.com>"
```

---

#### 4.6.3 删除后验证

```bash
# 1. 全量编译
cargo check --workspace

# 2. 全量测试
cargo test --workspace

# 3. 端到端测试
just test-e2e

# 4. 验证网关路由
curl http://localhost:3000/healthz
# 应返回: {"status":"ok","upstreams":{"api":"configured","admin":"configured","web":"configured"}}

# 5. 验证 web-bff 端点
curl http://localhost:3010/healthz
curl http://localhost:3010/readyz

# 6. 验证 admin-bff 端点
curl http://localhost:3020/healthz
curl http://localhost:3020/readyz
```

---

## 📋 执行顺序总结

```
Phase 4.1: 修复 E2E 测试链路（4 个子任务）
  ├─ 4.1.1 更新 Web E2E 夹具
  ├─ 4.1.2 更新 Desktop E2E 测试
  ├─ 4.1.3 更新预检查脚本
  └─ 4.1.4 更新 dev-desktop 脚本
  ↓
Phase 4.2: 更新构建和任务配置（5 个子任务）
  ├─ 4.2.1 更新 moon.yml
  ├─ 4.2.2 更新 test.just
  ├─ 4.2.3 更新 migrate.just
  ├─ 4.2.4 更新 deploy.just
  └─ 4.2.5 更新 processes.just
  ↓
Phase 4.3: 更新基础设施配置（5 个子任务）
  ├─ 4.3.1 更新 Docker Compose
  ├─ 4.3.2 创建 Dockerfile.web-bff
  ├─ 4.3.3 创建 Dockerfile.admin-bff
  ├─ 4.3.4 更新 Podman Compose
  └─ 4.3.5 更新 K8s 配置
  ↓
Phase 4.4: 更新文档和注释（5 个子任务）
  ├─ 4.4.1 更新 .env.example
  ├─ 4.4.2 更新包文档
  ├─ 4.4.3 更新 BFF 文档
  ├─ 4.4.4 更新 Agent 模板
  └─ 4.4.5 更新脚本引用
  ↓
Phase 4.5: 测试覆盖补充（2 个子任务）
  ├─ 4.5.1 审查并迁移高价值测试
  └─ 4.5.2 添加缺失测试场景
  ↓
Phase 4.6: 最终删除（3 个子任务）
  ├─ 4.6.1 删除前检查清单
  ├─ 4.6.2 执行删除
  └─ 4.6.3 删除后验证
```

---

## ⚠️ 风险矩阵

| Phase | 风险级别 | 影响范围 | 回滚难度 | 缓解措施 |
|-------|---------|---------|---------|---------|
| 4.1 E2E 测试 | 🔴 高 | 所有 E2E 测试 | 低 | 逐步验证每个夹具 |
| 4.2 任务配置 | 🟡 中 | 开发工作流 | 低 | 本地测试 just 命令 |
| 4.3 基础设施 | 🟡 中 | 部署流程 | 中 | 在本地 Docker 测试 |
| 4.4 文档 | 🟢 低 | 仅文档 | 极低 | 审查 PR |
| 4.5 测试 | 🟡 中 | 测试覆盖 | 低 | 比较测试矩阵 |
| 4.6 删除 | 🔴 高 | 不可逆 | 高 | 完整检查清单 |

---

## 🎯 验收标准

完成后：

| 验证项 | 预期结果 |
|--------|---------|
| `servers/api/` 不存在 | ✅ 已删除 |
| `cargo check --workspace` | ✅ 无错误 |
| `cargo test --workspace` | ✅ 无失败 |
| `just dev-api` | ✅ 启动 web-bff (3010) |
| `just dev-admin-bff` | ✅ 启动 admin-bff (3020) |
| Gateway 端口 3000 | ✅ 正确代理到所有 upstream |
| `/healthz` 端点 | ✅ 返回所有 upstream 状态 |
| E2E 测试 | ✅ 全部通过 |
| 无 runtime_server 引用 | ✅ 仅在历史文档中 |
| Docker Compose | ✅ 启动 web-bff 和 admin-bff |
| K8s 部署 | ✅ web-bff 和 admin-bff running |

---

## 📝 后续工作（本计划范围外）

1. **Moka 缓存层** — 如果 web-bff 需要缓存，添加 moka 依赖
2. **HTTP/3 支持** — 当前为脚手架代码，暂不实现
3. **SurrealDB 完全移除** — 评估是否仍需要 SurrealDB sidecar
4. **OpenTelemetry 集成** — 更新遥测配置以指向新服务
5. **性能基准测试** — 比较 servers/api vs web-bff 的性能差异

---

## 📊 预估工作量

| Phase | 复杂度 | 预估时间 | 依赖 |
|-------|-------|---------|------|
| 4.1 E2E 测试 | 中 | 2-3 小时 | 无 |
| 4.2 任务配置 | 低 | 1 小时 | 无 |
| 4.3 基础设施 | 高 | 4-6 小时 | Docker/K8s 环境 |
| 4.4 文档 | 低 | 1-2 小时 | 无 |
| 4.5 测试 | 中 | 3-4 小时 | Phase 4.1 完成 |
| 4.6 删除 | 低 | 30 分钟 | 所有前置 Phase 完成 |
| **总计** | | **11-16 小时** | |

---

## 🚀 快速执行建议

如果要快速完成，建议按以下优先级执行：

### 第一优先级（必须）:
1. Phase 4.1 — 修复 E2E 测试（否则 CI/CD 会失败）
2. Phase 4.2 — 更新任务配置（否则开发命令会失败）
3. Phase 4.6 — 删除 servers/api

### 第二优先级（重要）:
4. Phase 4.3 — 更新基础设施（否则部署会失败）
5. Phase 4.5 — 补充测试覆盖（否则可能遗漏边界情况）

### 第三优先级（可延后）:
6. Phase 4.4 — 文档更新（不影响功能）

**最小可行删除**: Phase 4.1 + 4.2 + 4.6 ≈ **4-5 小时**
