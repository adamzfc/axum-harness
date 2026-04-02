# 05. Runtime、Features 与 Adapters

## 核心抽象

你的系统不应按“前端 / 后端”划分，而应按：

- core
- features
- adapters
- runtimes / executable apps

## Core

### domain
- entity
- value object
- invariant
- policy
- pure ports

### usecases
- orchestration
- transaction boundary
- command / query handlers
- feature composition
- domain service coordination

### state
- session state
- cache policy
- retry / backoff metadata
- offline queue state
- sync markers
- hydration strategy

`state` 单独成层是必要的，因为未来多宿主、离线同步、协议摄入都会把“状态”从 UI 细节抬升成工程问题。

## Features

每个 feature 建议固定骨架：

```text
packages/features/<feature-name>/
  model/
  usecases/
  contracts/
  ui/
  adapters/
  tests/
  README.md
```

### Feature 原则

- feature 是业务能力，不是页面集合
- feature 可以有 UI，但不等于某个 app route
- feature 不得把 host-specific 逻辑写死
- feature 与 feature 之间只通过 usecases / contracts 协作

## Adapters

### hosts
把 canonical app 接到不同宿主：

- tauri
- browser extension
- telegram miniapp
- farcaster miniapp
- base app

### protocols
把外部协议接入系统：

- atproto
- farcaster
- nostr

### chains
把链上系统接入：

- evm
- base
- solana
- ton

### storage
把不同存储接入：

- indexeddb
- extension storage
- tauri store
- sqlite / libsql

### auth
把不同鉴权模型接入：

- oauth
- passkey
- dpop

### telemetry
把 tracing / otel / sink 接入：

- tracing
- otel

## 可执行层职责

### apps
负责启动、路由、宿主容器能力与最终产品组装。

### servers
负责 API / gateway / realtime 网络暴露。

### workers
负责 ingest、sync、index、异步处理。

## 组合模型

推荐执行关系：

- app / server / worker 只做组合与启动
- feature 承载业务能力组合
- core 承载不变量与用例
- adapter 负责所有外部世界翻译

## 容易犯的错误

### 错误 1：在 host adapter 写业务规则
结果：Tauri 和 web 逻辑分裂。

### 错误 2：把协议当成 feature
结果：feature 被协议绑死，未来迁移困难。

### 错误 3：把链当成 wallet feature 的内部实现
结果：钱包与结算、索引、链状态无法复用。

### 错误 4：worker 直接操作 feature 内部结构
结果：异步链路脱离 contracts。

## 规则摘要

- 宿主差异 -> hosts adapter
- 协议差异 -> protocols adapter
- 链差异 -> chains adapter
- 存储差异 -> storage adapter
- 业务能力 -> features
- 真正的业务规则 -> core
