# counter-service 完整链路核对清单

> 目标：把 `counter-service` 提升为仓库中唯一权威的后端参考实现链路，让 agent 在看完该链路后，可以自然复刻新的 service/server/worker/platform 实践，而不依赖冗余散文。

## 1. 文档定位

本清单不追求把 counter 做复杂，而追求把它做完整。

“100% 完整链路”指的是：

1. 从模型到实现
2. 从同步入口到异步推进
3. 从 contracts 到 verification
4. 从本地开发到 gate/CI
5. 从 DDD 分层到分布式语义

## 2. 目标态定义

`counter-service` 必须成为以下能力的参考样例：

1. service-local distributed semantics
2. DDD 分层实现
3. contract-first
4. optimistic concurrency / CAS
5. idempotency
6. outbox pattern
7. relay publish
8. projector rebuild/replay
9. platform model 对齐
10. verification 与 gate 接入
11. 本地开发、脚本、CI、drift discipline

## 3. 现状判断

当前 `counter-service` 是仓库中最接近完整链路的 reference module，但仍未达到 100% 权威样例标准。

已具备：

1. `services/counter-service/model.yaml`
2. `domain/application/ports/infrastructure`
3. migration
4. BFF handlers
5. outbox event write
6. relay worker 与 projector worker 对接雏形
7. 部分 platform model 对齐

仍需补齐：

1. relay 生产链路闭环
2. projector 真实订阅与真实 read model 闭环
3. verification 体系围绕 counter 明确成套化
4. 文档与代码状态统一
5. gate/CI 明确把 counter 作为 reference admission object

## 4. 完整链路核对清单

### 4.1 Service 模型层

1. 确认 `services/counter-service/model.yaml` 字段完整、命名统一、符合 schema。
2. 确认 owner entity、accepted commands、published events、served queries、cross-service reads 均有明确声明。
3. 确认 consistency、idempotency、partitioning、failure behavior 有明确 completeness 状态。
4. 确认模型中的命名与 contracts、事件主题、handler 路由保持一致。
5. 确认该模型足够成为新 service 的模板，而不是只够给 counter 自己用。

### 4.2 DDD 结构层

1. 确认 `domain/` 只承载领域对象与规则。
2. 确认 `application/` 只承载 use case orchestration。
3. 确认 `ports/` 只定义抽象依赖。
4. 确认 `infrastructure/` 只实现 adapter，不反向污染 domain。
5. 确认目录结构与 `codemap.yml` required files 一致。

### 4.3 Contracts 层

1. 确认 counter 对外 DTO、event、error code 已进入 `packages/contracts/**` 的正确位置。
2. 确认 service 内部 contracts 与 shared contracts 职责边界清晰。
3. 确认 BFF handler 没有跳过 contracts 直接暴露内部结构。
4. 确认 contract drift 可以被脚本与 CI 检测。
5. 确认 counter 的 contracts 可以作为新增服务 contract-first 模板。

### 4.4 Repository 与持久化层

1. 确认 repository trait 清晰表达 `load`、mutation、outbox、idempotency 职责。
2. 确认 libsql/turso adapter 只做存储翻译，不承载业务规则。
3. 确认 CAS 更新、回读、冲突处理逻辑完整。
4. 确认 migration 是单一真理源，不存在 SQL 定义重复。
5. 确认 idempotency table、outbox table、counter table 的职责边界清楚。

### 4.5 同步入口层

1. 确认 `servers/bff/web-bff` 的 counter handlers 只做协议适配。
2. 确认 handler 不写领域逻辑、不直接写数据库。
3. 确认 tenant context、auth、cache invalidation 行为有明确边界。
4. 确认路由、OpenAPI、contracts、response shape 一致。
5. 确认该同步入口足以成为“server 如何接 service”的参考样例。

### 4.6 Outbox Relay 层

1. 确认 relay 具备真实 outbox 读取能力，不以 in-memory fallback 作为主路径。
2. 确认 relay 的 checkpoint、dedupe、retry、publish ack 流程完整。
3. 确认 relay 对 event bus 的发布路径是参考实现，而不是临时 stub。
4. 确认 relay 的健康检查、配置加载、故障语义明确。
5. 确认 relay 可以作为后续所有 owner service 的异步发布模板。

### 4.7 Projector 层

1. 确认 projector 真实消费 replayable event source。
2. 确认 projector 具备 checkpoint、replay、rebuild 能力。
3. 确认 read model 不被误当 source of truth。
4. 确认 projector 的 read model 至少有一个真实持久化实现。
5. 确认 projector 可以成为 projection worker 的标准模板。

### 4.8 Platform Model 对齐层

1. 确认 `platform/model/services/counter-service.yaml` 与 service model 不冲突。
2. 确认 deployable、ownership map、topology、workflow 相关声明与 counter 实现一致。
3. 确认 schema 与 model 字段命名一致，不存在 `kind/type`、`services/hosts_services` 等漂移。
4. 确认 counter 相关 deployable 都可以通过 validator 检测。
5. 确认 platform model 不是纸面描述，而能反向约束 counter 链路。

### 4.9 Verification 层

1. 确认有覆盖 counter 的 service tests。
2. 确认有覆盖 BFF 的 integration 或 smoke tests。
3. 确认有覆盖 outbox/relay/projector 的 reference verification。
4. 确认 replay/rebuild/compatibility/drift 至少对 counter 链路有最小闭环。
5. 确认 counter 是 verification 层的主参考对象，而不是只有零散测试。

### 4.10 Tooling 与 Admission 层

1. 确认本地开发命令可以只围绕 counter 链路启动与验证。
2. 确认 `just`、`moon`、scripts、CI 都能覆盖 counter 参考链路。
3. 确认 counter 相关 drift、imports、state、workflow 校验都能独立执行。
4. 确认 counter 的 reference chain 文档只写真实状态，不把 skeleton 写成成熟实现。
5. 确认 agent 可以只读 counter reference chain 与 A 类文档就理解仓库开发方式。

## 5. 缺口分级

### P0

1. relay 主路径不能再依赖 in-memory fallback 充当成熟参考。
2. projector 不能继续停留在“保活循环 + stub 订阅”的状态。
3. counter reference chain 文档必须改成真实状态说明。
4. counter 相关 platform model / schema / codemap 命名漂移必须修正。

### P1

1. 补齐 counter 相关 verification 闭环。
2. 补齐 counter scoped gate 入口。
3. 补齐本地开发与 CI 的统一执行路径。

### P2

1. 把 counter 样例提炼成新 service 引导模板。
2. 引入更强的 replay、compat、golden 样例。

## 6. 完成标准

当以下条件全部满足时，才可宣称 `counter-service` 是 100% 完整参考链路：

1. 模型、contracts、service、server、worker、platform、verification 全部闭环。
2. 真实链路跑通，不依赖临时 in-memory stub 充当主要路径。
3. 文档与代码状态一致。
4. gate/CI 能强制检查关键约束。
5. agent 看完 counter 链路即可照着扩展新服务。

## 7. 交付产物

建议最终交付：

1. 一份真实、精简的 `docs/operations/counter-service-reference-chain.md`。
2. 一组围绕 counter 的 scoped gates。
3. 一组围绕 counter 的 verification fixtures 与 reference tests。
4. 一组与 counter 对齐的 platform model 与 ownership 声明。
