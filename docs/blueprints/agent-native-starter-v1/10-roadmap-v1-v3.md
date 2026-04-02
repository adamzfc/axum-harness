# 10. 路线图：V1 / V2 / V3

## V1：工程母体成形

目标：把仓库从“可跑模板”升级为“可长期演化的制度化工程”。

### V1 必做
- 确认目录结构
- 工具链职责定稿（Bun + moon + proto + Just）
- 建立 contracts/typegen 闭环
- 补齐任务图
- 建立 tracing + tests + evals 最小闭环
- 建立 AGENTS / skills / playbooks / rubrics
- 确立 Rust / Tauri / SvelteKit / TS 默认基线
- 文档集入库

### V1 不做
- 默认上 HTTP/3
- 默认上多协议 federation runtime
- 默认上复杂多 agent 自主协作
- 默认把所有 auth 新范式装入基线

## V2：高概率扩展能力

目标：把 starter 变成真正能承载多宿主、多协议、多 worker 的平台。

### V2 建议内容
- host adapter 体系做实
- browser extension / miniapp host 实装
- worker replay / fixture 体系做实
- offline sync / retry / reconnect 策略
- tracing / otel sink 完整化
- release automation 强化
- protocol/chains adapter 骨架完成

### V2 可选能力
- passkey-ready
- DPoP-ready
- realtime server
- richer docs site / component docs

## V3：实验边车与前沿能力

目标：在不破坏核心的前提下，接入前沿能力。

### V3 候选
- HTTP/3 runtime lane
- DPoP 真正落地
- passkeys-first auth
- ATProto worker / federation lane
- Farcaster / Nostr 深度 worker
- UCAN / DID 等实验授权模型
- 更复杂的 multi-agent orchestration

## 升级判据

只有在以下条件满足时，才从 V1 升 V2 / V3：

- contracts 稳定
- task graph 稳定
- core / feature / adapter 边界稳定
- E2E 与 evals 能兜住回归
- 发布和回滚具备可操作性

## 禁止的升级方式

- 用前沿技术替代制度建设
- 在无 typegen / tests / evals 的情况下直接引入高级 auth/protocol
- 让某个新宿主或协议直接主导目录形态
