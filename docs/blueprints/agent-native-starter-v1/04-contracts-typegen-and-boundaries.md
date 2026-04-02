# 04. Contracts、类型闭环与边界

## 为什么 contracts 是系统中心

agent 开发最怕四套真相并存：

- Rust 类型
- TS 类型
- API schema
- 测试 fixture

这会导致：

- 改后端不改前端
- 改 TS 不改 schema
- mock data 与真实类型脱节
- evals 对错误类型做出错误判断

## 建议子包

```text
packages/contracts/
  api/
  auth/
  events/
  errors/
  protocols/
  ui/
  codegen/
```

## 每类 contracts 的职责

### api
面向 REST / HTTP / IPC / command-query 的请求与响应模型。

### auth
session、proof、credential、principal、capability 与 auth error。

### events
内部事件、队列消息、后台作业 payload。

### errors
系统级标准错误形状与错误码。

### protocols
对接外部协议时的统一 schema wrapper。
注意：不要把原生协议结构直接散落到 feature 与 worker。

### ui
前端可跨 feature 复用的 UI-facing state schema、form schema、token schema。

### codegen
代码生成配置与导出规则。

## 单一真理源原则

必须做到：

- TS 类型由 contracts 自动生成
- OpenAPI / schema 从 contracts 派生
- 测试 fixture builder 与 contracts 对齐
- 前端禁止手写 Rust mirror types
- worker 禁止私自定义事件协议

## 推荐 codegen 产物

```text
generated/
  ts/
  openapi/
  json-schema/
  fixtures/
```

这些产物可以进入：

- `apps/client/web/app/src/generated`
- `servers/api/generated`
- `tools/evals/generated`

## 变更规则

### 修改 contracts 时必须做

1. 运行 `repo:typegen`
2. 运行 `repo:contracts-check`
3. 运行契约测试
4. 检查前端与服务端生成产物 diff
5. 更新必要的 docs / examples / eval fixtures

### 不允许的做法

- 在 feature 内定义“临时 DTO”绕过 contracts
- 在 app 内复制 contracts 类型
- 在 worker 内定义独有 schema 且不回流到 contracts
- 在 adapter 中混入 transport-specific 脏数据结构到 core

## 边界规则

### core
只依赖 contracts 中与自身相关的抽象，不依赖具体传输细节。

### features
消费 contracts，但不负责定义协议底层细节。

### adapters
实现协议、存储、链、宿主翻译，但不得发明新的业务真相。

### apps / servers / workers
只组合 contracts，不重新定义数据事实。

## DTO 与 domain 的关系

不要把 DTO 当 domain model。  
推荐规则：

- domain model：表达业务不变量
- DTO / schema：表达边界传输形状
- mapper / translator：放在 adapter 或 feature 边界

## 最低落地要求

V1 至少做到：

- API DTO 自动生成 TS 类型
- Error schema 统一
- Event payload 收敛到 contracts/events
- 前端和 worker 不再手写 mirror types
