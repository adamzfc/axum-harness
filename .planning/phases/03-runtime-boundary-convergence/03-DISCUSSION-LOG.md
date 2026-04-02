# Phase 3: Runtime 边界收敛 - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-02
**Phase:** 03-runtime-boundary-convergence
**Areas discussed:** Port 实现归属, usecases 对 contracts_api 的耦合, runtime_tauri adapter 职责, 边界强制机制, features/ 模块定位

---

## Port 实现归属

| Option | Description | Selected |
|--------|-------------|----------|
| 迁移到 packages/adapters/storage/ | 按蓝图对齐：servers/api 只做 HTTP runtime，adapter 实现放 packages/adapters/ | ✓ |
| 保留在 servers/api，文档标注 | 当前项目规模小，port 实现和 server 强关联，迁移成本高于收益 | |
| 折中：定义 trait 在 domain，实现按宿主分散 | Port trait 留在 domain，实现按 runtime 分散，统一命名规范 | |

**User's choice:** 迁移到 packages/adapters/storage/
**Follow-up:** 每个实现一个独立 crate（adapters/storage/surrealdb 和 adapters/storage/libsql）
**Notes:** 严格按蓝图对齐，每个 adapter 实现独立编译隔离

---

## usecases 对 contracts_api 的耦合

| Option | Description | Selected |
|--------|-------------|----------|
| 保持现状 | usecases 直接用 contracts_api 类型，减少映射代码 | |
| 解耦：usecases 定义内部类型 | usecases 定义自己的类型，route handler 做映射 | ✓ |
| 你决定 | 根据 hexagonal 原则和项目规模选择 | |

**User's choice:** 解耦：usecases 定义内部类型
**Notes:** usecases 只依赖 domain，route handler 负责 contracts_api ↔ usecases 类型映射

---

## runtime_tauri adapter 职责

| Option | Description | Selected |
|--------|-------------|----------|
| Tauri command 桥接层 | runtime_tauri 定义 command handler 调用 usecases，native-tauri 只做 bootstrap | ✓ |
| 和 native-tauri 合并 | 项目规模小，native-tauri 直接做 command 桥接 | |
| 你决定 | 根据蓝图和实际复杂度选择 | |

**User's choice:** Tauri command 桥接层
**Notes:** runtime_tauri 承载所有 #[tauri::command] 函数，native-tauri 只做 builder/bootstrap

---

## 边界强制机制

| Option | Description | Selected |
|--------|-------------|----------|
| CI check + cargo-deny | repo:verify 增加边界检查，cargo-deny 限制非法依赖 | |
| rubric 文档 + agent 检查 | .agents/rubrics/ 写边界规范，agent code review 检查 | |
| 两者结合 | CI 做硬性检查，rubric 做语义检查，双保险 | ✓ |

**User's choice:** 两者结合
**Notes:** CI cargo-deny 做硬性门禁 + .agents/rubrics/boundary-compliance.md 做语义规范

---

## features/ 模块定位

| Option | Description | Selected |
|--------|-------------|----------|
| 独立 Cargo crate | 每个 feature 独立 crate，组合 core + contracts + adapters | ✓ |
| 纯目录组织，代码在宿主中 | features/ 只做分类，代码在宿主中按 feature 分目录 | |
| 折中：features 定义 trait/接口 | features crate 定义接口，宿主实现 | |
| 你决定 | 根据蓝图和 hexagonal 原则选择 | |

**User's choice:** 独立 Cargo crate
**Notes:** Feature crates 组合 core + contracts + adapters，不依赖宿主 runtime

---

## Agent's Discretion

- adapter crate 内部文件组织
- cargo-deny 具体规则配置
- runtime_tauri command handler 命名和分组
- Feature crates 具体命名规范
- 依赖方向检查实现方式

## Deferred Ideas

- Phase 4 做 features 实际业务实现
- adapters/auth/ (OAuth, passkey) 实现 — Phase 4
- telemetry adapters — 按需
- 完整 cargo-deny 配置 — 后续迭代
