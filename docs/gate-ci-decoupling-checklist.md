# gate / CI / just / moon / scripts 解耦重构清单

> 目标：建立一套后端 admission 架构，使 gate 成为 repo 内真理源，CI 只负责调用 gate，`just`、`moon`、scripts 各司其职，并按 DDD 分类统一命名。

## 1. 目标态原则

### 1.1 分层职责

1. `scripts/**`：规则执行与校验逻辑。
2. `moon`：执行图、缓存、并行、affected orchestration。
3. `just`：人类与 agent 的稳定入口别名。
4. `agent/manifests/gate-matrix.yml`：subagent -> gate 真理源。
5. `.github/workflows/**`：CI 编排壳层，只调用 repo 内 gate。

### 1.2 解耦原则

1. gate 不能只存在于 GitHub Actions 里。
2. CI 不得再维护一套独立规则语言。
3. 本地执行与 CI 执行必须同源。
4. gate 分类必须反映 DDD 边界，而不是历史目录习惯。
5. gate matrix 必须能映射到真实命令与真实脚本。

## 2. 当前问题摘要

1. GitHub workflow 中仍内嵌 package 列表与流程判断。
2. `just`、`moon`、scripts、workflow 的命令语言没有完全统一。
3. gate 的命名与 DDD 分类尚未完全对齐。
4. 一些 drift、validation、quality 逻辑分散在不同层，归属不够清楚。
5. 本地与 CI 虽然方向接近，但还不是完全同源执行链。

## 3. 目标 gate 分类

建议最终 gate 只保留以下分类：

1. `contracts`
2. `services`
3. `servers`
4. `workers`
5. `platform`
6. `boundaries`
7. `drift`
8. `coverage` 作为独立信号，不作为主 admission gate

### 3.1 每类 gate 关注点

`contracts`

1. contract drift
2. schema compatibility
3. generated alignment

`services`

1. service-local semantics
2. DDD 结构完整性
3. service tests

`servers`

1. protocol adaptation
2. contracts alignment
3. smoke tests

`workers`

1. checkpoint
2. replay
3. retry
4. idempotency

`platform`

1. schema/model validation
2. topology validation
3. deployable validation

`boundaries`

1. import restrictions
2. write boundary restrictions
3. generated readonly restrictions

`drift`

1. generated drift
2. sdk drift
3. catalog drift

## 4. 重构主清单

### 4.1 gate matrix 对齐

1. 审计 `agent/manifests/gate-matrix.yml` 当前映射是否与真实命令一致。
2. 把每个 subagent 映射到上述 DDD 分类 gate。
3. 确保每个 gate 都有 repo 内真实执行入口。
4. 确保 `scripts/run-scoped-gates.ts` 直接消费 gate matrix，而不是旁路实现。
5. 删除无法落地执行的 gate 名称与虚假门禁。

### 4.2 scripts 层收口

1. 将规则判断统一收口到 `scripts/**`。
2. 规范脚本命名，使其与 gate 分类一致。
3. 为每个 gate 提供标准退出码与结构化错误输出。
4. 将 workflow 中的 ad hoc shell 检查下沉到 scripts。
5. 将 package 列表解析、crate 选择等逻辑下沉到 scripts 或 moon。

### 4.3 moon 层收口

1. 为每类 gate 提供明确 `moon` task。
2. 用 `moon` 管理依赖顺序、缓存、affected execution。
3. 把全量执行与 scoped execution 都做成 `moon` 可调任务。
4. 禁止在 `moon` 中重复编写大量规则逻辑。
5. 将 `moon` 作为本地与 CI 的共同执行图。

### 4.4 just 层收口

1. `just` 只提供稳定命令别名。
2. `just` 命名与 DDD gate 命名对齐。
3. 删除历史命名与重复入口。
4. 任何复杂逻辑都跳转给 `moon` 或 `scripts`。
5. 让 agent 与人类都能通过少量统一命令触发 admission。

### 4.5 GitHub Actions 瘦身

1. 将 workflow 改造成薄编排层。
2. workflow 只负责 setup、cache、调用 repo gate、汇总结果。
3. 移除 workflow 内直接维护的业务规则与 package 细节。
4. coverage 与 admission 分离，避免混成一套信号。
5. 将 `primary/secondary` 分层重新映射到更清楚的 DDD gate 分类。

## 5. 推荐命令体系

建议最终只暴露这类命令语言：

1. `just gate-contracts`
2. `just gate-services`
3. `just gate-servers`
4. `just gate-workers`
5. `just gate-platform`
6. `just gate-boundaries`
7. `just gate-drift`
8. `just gate-backend`
9. `just gate-counter-reference`

其中：

1. `just` 只转发
2. `moon` 负责编排
3. `scripts` 负责真正的规则执行

## 6. CI 目标结构

建议最终 workflow 只保留：

1. `backend-ci.yml`
2. `backend-quality-gate.yml`
3. `backend-coverage.yml`

### 6.1 `backend-ci.yml`

负责：

1. 后端主 admission
2. `contracts/services/servers/workers/platform/boundaries/drift` 聚合执行

### 6.2 `backend-quality-gate.yml`

负责：

1. 更重的治理类验证
2. 安全、drift、replay、resilience、compat 等非主通道信号

### 6.3 `backend-coverage.yml`

负责：

1. 覆盖率采集与上传
2. 不阻塞主 admission，除非明确设定阈值

## 7. counter reference 专项 gate

需要单独建立 `counter-service` 参考链路 gate，覆盖：

1. model/schema alignment
2. service tests
3. BFF smoke
4. outbox/relay path
5. projector replayability
6. platform model alignment
7. reference document freshness

这个专项 gate 的意义不是“偏爱 counter”，而是把它作为 reference module 的 admission anchor。

## 8. 迁移顺序

### 第一阶段

1. 统一 gate 命名。
2. 让 gate matrix 与真实命令对齐。
3. 修复 workflow 调用入口。

### 第二阶段

1. 将 package 列表与业务逻辑从 workflow 下沉。
2. 用 `moon` 接管依赖关系与执行图。
3. 用 `just` 统一对外入口。

### 第三阶段

1. 引入 counter reference 专项 gate。
2. 引入更精确的 scoped gates 与 affected execution。
3. 引入结构化 gate 结果，增强 agent 自主判断能力。

## 9. 验收标准

完成重构后，应满足：

1. 本地与 CI 使用同一套 gate。
2. GitHub Actions 不再是规则真理源。
3. `just`、`moon`、scripts 各司其职。
4. gate 分类能直接映射 DDD 边界。
5. agent 看 gate matrix 就能知道该跑什么验证。
6. `counter-service` 有独立、可信的 reference admission gate。

## 10. 交付产物

建议最终交付：

1. 一版精简后的 `gate-matrix.yml`。
2. 一组按 DDD 分类命名的 gate 脚本与 `moon` 任务。
3. 一组薄编排、同源调用的 GitHub workflows。
4. 一组稳定、简短、对人和 agent 友好的 `just` 入口命令。
