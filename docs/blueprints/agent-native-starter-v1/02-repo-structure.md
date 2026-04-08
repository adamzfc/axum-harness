# 02. 仓库结构蓝图

## 目标目录

```text
repo/
  apps/
    client/
      desktop/
      browser-extension/
      web/
        app/
        hosts/
          telegram-miniapp/
          farcaster-miniapp/
          base-app/

    ops/
      docs-site/
      storybook/

  servers/
    api/
    gateway/
    realtime/

  workers/
    protocols/
      atproto/
      farcaster/
      nostr/
    chains/
      evm/
      base/
      solana/
      ton/
    jobs/
      notifications/
      media/
      search/
      sync/

  packages/
    core/
      domain/
      usecases/
      state/

    features/
      auth/
      profile/
      feed/
      social-graph/
      wallet/
      payments/
      notifications/
      admin/

    adapters/
      hosts/
        tauri/
        browser-extension/
        telegram-miniapp/
        farcaster-miniapp/
        base-app/
      protocols/
        atproto/
        farcaster/
        nostr/
      chains/
        evm/
        base/
        solana/
        ton/
      storage/
        indexeddb/
        extension-storage/
        tauri-store/
        sqlite/
        libsql/
      auth/
        oauth/
        passkey/
        dpop/
      telemetry/
        tracing/
        otel/

    contracts/
      api/
      auth/
      events/
      errors/
      protocols/
      ui/
      codegen/

    ui/
      design-system/
      web/
      icons/
      tokens/

    shared/
      config/
      env/
      utils/
      testing/
      types/

  tools/
    scripts/
    generators/
    mcp/
      servers/
      clients/
    evals/
      datasets/
      graders/
      suites/

  .agents/
    skills/
    prompts/
    playbooks/
    rubrics/
```

## 分层解释

### apps
可直接运行的终端产品壳层。  
原则：不承载核心业务规则，只负责宿主组合与启动。

### servers
对外暴露网络服务的执行单元。  
原则：API / gateway / realtime 分角色，不把 jobs 混进来。

### workers
后台任务、协议消费、链索引、异步 job。  
原则：worker 生命周期、故障模型、扩缩容方式都与 server 不同，必须单独分层。

### packages/core
纯核心规则。  
这里不能依赖 host、protocol、chain。

### packages/features
业务能力模块化。  
每个 feature 组合 core + contracts + adapters，而不是直接依赖某个 app。

### packages/adapters
一切外部世界接入的翻译层。  
任何宿主/协议/链/存储/Auth/Telemetry 都应通过 adapter 进入。

### packages/contracts
整个系统的单一真理源。

### packages/ui
设计系统、组件基元、图标和 token。

### packages/shared
不属于业务规则的共享工具，例如 config、testing helpers、env 处理等。

### tools
生成器、MCP、评估数据与 grader、repo 脚本。

### .agents
agent 的制度层，而不是普通文档附件。

## 目录边界红线

- `core` **不得** 依赖 `apps`、`servers`、`workers`
- `features` **不得** 依赖 `usecases`（feature 定义 trait，usecases 实现 trait）
- `features` **不得** 直接依赖具体 host app
- `adapters` **不得** 承载业务策略或业务逻辑
- `contracts` **必须** 是共享类型的唯一真理源；feature 内部类型如与 contracts 重叠，必须引用 contracts 的类型
- `contracts` **不得** 被实现细节污染
- `workers` **不得** 直接绕过 contracts 自定义 event schema
- `apps/client/web/hosts/*` 只能做宿主适配，不能复制 canonical app 业务逻辑
- 同一业务概念在不同层的类型**不得**出现字段差异（如 `i64` vs `u64`）
