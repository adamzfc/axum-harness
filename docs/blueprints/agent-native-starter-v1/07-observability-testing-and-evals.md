# 07. 可观测性、测试与 Evals

## 7.1 可观测性目标

对 agent-first 项目来说，可观测性不仅用于运行监控，也用于迭代诊断。

必须同时覆盖：

- 应用链路
- 异步作业链路
- 前端交互链路
- agent 执行链路

## 7.2 Telemetry 分层

### App Telemetry
- API request / response trace
- usecase 执行
- database / storage 操作
- external API latency
- queue publish / consume

### UX Telemetry
- route transition
- render / hydration 异常
- form error
- crash boundary
- slow interaction

### Infra Telemetry
- worker lag
- job retry
- index delay
- webhook success/failure
- sync drift

### Agent Telemetry
- prompt version
- plan id
- tool usage
- generated patch hash
- verify result
- eval suite result
- rollback reason

## 7.3 Logging 原则

- 关键路径必须结构化日志
- 不能只打字符串，不打上下文
- 不能把 secret 记进日志
- 日志级别与用途区分明确
- trace id 要贯穿 request / job / agent run

## 7.4 测试金字塔

### Contracts Tests
- schema stability
- backward compatibility
- typegen output diff
- serialization roundtrip

### Core Tests
- domain invariant tests
- usecase tests
- state transition tests

### Adapter Tests
- protocol parser tests
- auth proof tests
- storage semantics tests
- chain normalization tests

### UI Tests
- component tests
- interaction tests
- route guard tests
- host-specific render compatibility tests

### E2E Tests
- web happy path
- desktop critical path
- extension critical path
- auth bootstrap
- sync / retry / reconnect path

### Agent Evals
- scaffold correctness
- rule compliance
- forbidden mutation detection
- contract update workflow correctness
- regression replay

## 7.5 何时需要 Evals

当仓库进入“agent 经常自动修改代码”的阶段，就必须有 evals。

最低要求：

- 典型 feature 新增场景
- contracts 修改场景
- host adapter 修改场景
- protocol worker 修改场景
- 回归 bug 复现场景

## 7.6 测试与观测的关系

观测不是测试替代。  
测试回答“预期是否成立”，观测回答“失败时到底发生了什么”。

最佳做法：

- E2E 失败时附带 trace / screenshot / logs
- worker replay 失败时输出 payload fixture
- agent eval 失败时输出 plan、patch、verify 摘要

## 7.7 V1 最低落地

- 关键 usecase 结构化日志
- API / worker / app trace id
- contracts tests
- core tests
- 至少一条 web + desktop E2E
- 至少一套 agent eval suite
