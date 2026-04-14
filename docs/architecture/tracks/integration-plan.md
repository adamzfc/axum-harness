# 集成轨道计划

> 状态：等待前端轨道稳定后启动
> 目标：在前后端都具备自治边界后，完成 monorepo 级 gate、CI、release、drift、impact、scaffold 的组合收尾。

---

## 0. 边界

### 0.1 本轨道负责

- `repo/backend/frontend/integration` gate 组合关系
- CI / release 分层收尾
- contracts/generated/sdk/app 的 drift 与组合校验
- impact 分析与 scaffold 的 monorepo 级收尾

### 0.2 本轨道不负责

- 重做前端目录结构
- 重做后端 service / worker / server 结构
- 在前端未稳定前提前抽象 monorepo 组合层

---

## 1. 启动前提

只有同时满足下面条件，本轨道才应启动：

- 后端轨道已进入冻结维护状态
- 前端轨道已建立独立规则真相源
- `gate-frontend` 已可运行
- Web 客户端主要结构边界已稳定

---

## 2. 工作项

### I1. gate 组合

- 明确 `gate-repo`
- 明确 `gate-backend`
- 明确 `gate-frontend`
- 明确 `gate-integration`
- 决定 `gate-local / gate-prepush / gate-ci / gate-release` 如何组合调用

### I2. CI / release 分层收尾

- backend workflow
- frontend workflow
- integration workflow
- release 前组合门禁

### I3. drift / impact / scaffold

- contracts drift
- generated drift
- integration smoke
- impact report
- frontend/backend 对称 scaffold

---

## 3. 完成标准

- monorepo 组合层不再偏后端或偏前端
- 各轨道自治规则清晰，集成层只负责组合正确性
- CI / release / drift / impact / scaffold 都能映射到清楚的分层责任
