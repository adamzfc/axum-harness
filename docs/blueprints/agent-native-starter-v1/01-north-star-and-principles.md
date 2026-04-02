# 01. 北极星与原则

## 北极星

把仓库建设成一个 **Agent-Native Cross-Platform Application Engineering Base**：

- 对独立开发者足够省脑力
- 对 AI agent 足够低歧义
- 对大多数跨平台项目足够通用
- 对未来技术变化保持吸收能力
- 让正确路径变便宜，让错误路径变昂贵

## 一阶原则

### 1. 单一真理源优先于框架优雅

如果一个系统里：

- Rust 类型是一套
- TS 类型是一套
- OpenAPI 是一套
- 测试 fixture 又是一套

那 agent 一定会漂移。  
所以必须先建立单一真理源，再谈运行时优雅。

### 2. 核心业务规则不依赖宿主

host、protocol、chain 都是“外部世界”。

- Tauri 是宿主
- browser extension 是宿主
- Telegram / Farcaster miniapp 是宿主
- ATProto / Farcaster / Nostr 是协议
- EVM / Solana / TON 是链

这些都不能进入 core。

### 3. 所有高级能力都必须通过边界进入系统

任何“新技术”都要先回答三个问题：

- 它进入哪个目录层？
- 它依赖哪些 contracts？
- 它对现有运行时是否可选？

答不出来，就不能进默认基线。

### 4. 任务图优先于脚本堆积

agent 不应该直接靠记忆调用散落命令。  
所有关键动作都必须在 moon / Just 顶层可发现。

### 5. 反馈闭环优先于一次性生成

agent-first 项目最重要的不是“首轮生成正确率”，而是：

- 失败是否可追踪
- 回归是否可复现
- 补丁是否可验证
- 新经验是否能沉淀成规则

## 三层稳定性模型

### 永久核心

几乎所有项目都需要：

- workspace / task graph
- contracts / typegen
- logging / tracing / metrics
- testing / release / docs / security
- agent rules / evals / skill system

### 高概率扩展

很多项目需要，但不是默认全部启用：

- auth / session / proof
- offline sync
- realtime
- updater
- worker orchestration
- protocol ingest
- chain indexing

### 实验边车

有前瞻性，但不应默认塞进基线：

- HTTP/3
- DPoP
- passkeys-only auth
- ATProto federation runtime
- UCAN
- autonomous multi-agent planning

## 成功标准

这套 starter 成功的标准不是“看起来先进”，而是：

- 新 agent 一小时内能安全开始工作
- 改 contracts 后 TS / schema / fixtures 自动同步
- 改 host 不会污染 core
- 引入新协议不需要推翻 feature 结构
- 失败能被 trace / eval / regression 抓住
