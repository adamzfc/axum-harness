# 后端轨道计划

> 状态：冻结收口阶段
> 目标：在后端主架构已基本完成的前提下，收口门禁、CI、lefthook 和治理链路，不再继续扩大后端重构范围。

---

## 0. 边界

### 0.1 本轨道负责

- `services/`、`servers/`、`workers/`、`platform/` 的结构治理收口
- `validate-existence`、`validate-imports`、`gate-prepush` 等后端/仓库边界门禁
- backend CI 分层、workflow 触发、质量门禁语义
- lefthook 与本地 gate 统一入口

### 0.2 本轨道不再负责

- 新一轮大规模后端目录重构
- 扩展 service 模板种类到更复杂抽象
- 把 frontend 规则塞回 backend gate
- 替 frontend 设计目录或页面结构

---

## 1. 当前判断

- 后端主架构已经基本成型
- `validate-existence --mode warn` 已清零
- `validate-imports` 已在 strict 模式通过，并进入 pre-push strict
- 后端当前最值钱的工作，不是继续重构业务模块，而是让治理链路稳定、解释清楚、可长期维护

---

## 2. 当前任务清单

### B1. gate 与规则收口

- 维持 `validate-existence` 的 planned/partial/active 分层语义
- 评估 `gate-ci` 是否显式组合 `gate-imports strict`
- 防止 `validate-existence` 继续膨胀成全仓库统一模板检查器

### B2. backend CI 收口

- 按 `docs/architecture/backend-ci.md` 的主线 / 治理 / 替补 / 实验分层继续整理 workflow
- 确保 backend CI 只看后端路径，不把 web 检查混入后端 admission

### B3. hook / 本地开发入口收口

- 维持 `just` 和 `lefthook` 的统一入口
- 保持本地 warn-only 与 CI/release strict 的可解释性

---

## 3. 完成标准

- 后端轨道进入“冻结可维护”状态
- 后端 gate 不再持续扩大战线
- backend CI 的职责、触发范围、阻塞语义稳定
- 后续只需要阻塞式小修，不再需要架构级继续改写

---

## 4. 移交条件

满足以下条件后，默认把主战场移交给前端轨道：

- 后端只剩小修和 CI 细节
- 不再需要新增后端治理真相源
- frontend gate 的缺口成为当前仓库的主要治理短板
