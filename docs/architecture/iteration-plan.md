# 迭代计划总纲与索引

> 目标：以 monorepo 全局视角组织后端、前端、集成三条独立重构轨道，避免多 agent 并行推进时互相踩边界。
> 角色：**本文件只负责总纲、索引、执行顺序、阅读协议**，不承载各轨的详细计划、进度记录和 handoff。
> 规则：后续 agent 先读本文件，再只进入当前被激活的那一条轨道文档；禁止同时消费多条轨道的详细计划。

---

## 0. Monorepo 立场

### 0.1 第一原则

- monorepo 的第一公民不是后端，也不是前端，而是跨边界事实
- 仓库级规则只约束契约、生成物、依赖方向、集成正确性
- 后端和前端都以自治轨道推进，最后由 integration 轨道组合收尾

### 0.2 当前现实

- 后端主架构已经基本完成，当前重点是门禁、CI、lefthook、治理链路收口
- 前端 `apps/web` 仍带有明显历史遗留结构，目录分层、调用边界、gate 都未系统优化
- integration 轨道现在还不应成为主战场，因为前端自治边界尚未稳定

### 0.3 总策略

按严格串行顺序执行：

1. 先收口后端治理轨道到“冻结可维护”状态
2. 再启动前端治理与 Web 客户端重构轨道
3. 最后执行 integration / monorepo 组合收尾轨道

不允许的并行方式：

- 一边改前端目录结构，一边改仓库级 gate 入口
- 一边改 integration CI 组合，一边改 frontend 真相源
- 一边继续扩大后端治理半径，一边让前端补自治边界

---

## 1. 文档地图

### 1.1 后端轨道

- 计划：`docs/architecture/tracks/backend-plan.md`
- 进度：`docs/architecture/tracks/backend-progress.md`
- 交接：`docs/architecture/tracks/backend-handoff.md`

### 1.2 前端轨道

- 计划：`docs/architecture/tracks/frontend-plan.md`
- 进度：`docs/architecture/tracks/frontend-progress.md`
- 交接：`docs/architecture/tracks/frontend-handoff.md`

### 1.3 集成轨道

- 计划：`docs/architecture/tracks/integration-plan.md`
- 进度：`docs/architecture/tracks/integration-progress.md`
- 交接：`docs/architecture/tracks/integration-handoff.md`

---

## 2. 执行顺序

### 2.1 当前激活顺序

必须按下面顺序推进：

1. 后端轨道：仅做收口与冻结，不再继续扩大定义
2. 前端轨道：先治理边界，再改 Web 客户端结构
3. 集成轨道：最后做 monorepo gate / CI / release / drift / impact / scaffold 收尾

### 2.2 每条轨道的内部顺序

#### 后端轨道

- 只允许继续收口门禁、CI、hook、规则分层
- 不允许重新打开大规模后端架构重构

#### 前端轨道

- Step F1：前端治理边界与 gate-frontend
- Step F2：typegen、typed client、目录结构、guards、tokens、tests

#### 集成轨道

- Step I1：repo/backend/frontend/integration gate 组合
- Step I2：CI / release 分层收尾
- Step I3：drift / impact / scaffold / E2E 集成收尾

---

## 3. 当前状态

### 3.1 后端轨道状态

- 状态：`active-maintenance`
- 说明：Phase 1 已基本收口，`validate-existence` 已清零，`validate-imports` 已在 pre-push 进入 strict
- 下一步：只做冻结与收尾，不再扩大战线

### 3.2 前端轨道状态

- 状态：`next-up`
- 说明：这是当前下一条应该激活的主轨道
- 原因：`apps/web` 仍然缺少独立治理边界与系统性目录优化

### 3.3 集成轨道状态

- 状态：`blocked-by-frontend`
- 说明：在前端轨道稳定前，不应提前做 monorepo 级组合收尾

---

## 4. Agent 阅读协议

后续任何 agent 开始工作时，必须按这个顺序读文档：

1. `AGENTS.md`
2. `docs/architecture/repo-layout.md`
3. `agent/codemap.yml`
4. `docs/architecture/iteration-plan.md`
5. 当前激活轨道的 `plan`
6. 当前激活轨道的 `progress`
7. 当前激活轨道的 `handoff`

禁止：

- 同时读取 frontend 和 integration 的详细计划后混合推进
- 把 backend 的已验证约束直接投射成 frontend 的目录真相
- 在未完成当前轨道前，擅自推进下一轨道文档中的实施项

---

## 5. 轨道切换规则

只有满足下面条件，才能从一条轨道切到下一条轨道：

### 5.1 后端 -> 前端

- 后端 handoff 中明确标记为“冻结可维护”
- 当前后端只剩阻塞式小修或 CI 收尾，不再需要架构级扩展

### 5.2 前端 -> 集成

- 前端已有独立规则真相源
- `gate-frontend` 已落地
- Web 客户端的主要目录与调用边界已稳定

---

## 6. 更新规则

- 本文件只更新总纲、顺序、轨道激活状态与文档索引
- 具体实施变化必须写回对应轨道的 `progress` 和 `handoff`
- 如果某项变更同时影响多条轨道，优先写入当前激活轨道，并在其他轨道 handoff 中记录影响

---

## 7. 当前指令

当前后续 agent 的默认入口应为：

- 若任务涉及后端门禁、CI、hook 收口：
  - `docs/architecture/tracks/backend-plan.md`
  - `docs/architecture/tracks/backend-progress.md`
  - `docs/architecture/tracks/backend-handoff.md`
- 若任务涉及前端治理与 `apps/web` 结构重构：
  - `docs/architecture/tracks/frontend-plan.md`
  - `docs/architecture/tracks/frontend-progress.md`
  - `docs/architecture/tracks/frontend-handoff.md`
- 若任务涉及 monorepo 组合收尾、gate 组合、release/CI/integration：
  - `docs/architecture/tracks/integration-plan.md`
  - `docs/architecture/tracks/integration-progress.md`
  - `docs/architecture/tracks/integration-handoff.md`

前提：一次只进入一条轨道的详细文档；若需要跨轨，只能在当前轨道的 progress 或 handoff 中记录影响，不能直接并行实施。

---

## 8. 参考文件

- `AGENTS.md`
- `docs/architecture/repo-layout.md`
- `agent/codemap.yml`
- `docs/architecture/backend-ci.md`
- `docs/architecture/tracks/backend-plan.md`
- `docs/architecture/tracks/frontend-plan.md`
- `docs/architecture/tracks/integration-plan.md`
- `justfiles/gates.just`
- `scripts/gate.ts`
- `scripts/validate-existence.ts`
- `scripts/validate-imports.ts`
