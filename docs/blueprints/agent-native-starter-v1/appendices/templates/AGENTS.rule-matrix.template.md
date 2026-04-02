# AGENTS 规则矩阵模板

## 目标

本文件用于定义 agent 在仓库中的行为边界、可修改目录、必须验证项和失败回滚路径。

## 全局规则

1. 必须优先使用已有任务，不直接发明临时命令。
2. 必须先读相关 docs、playbooks、skills。
3. 修改 contracts 后必须运行 typegen。
4. 修改 host / protocol / chain 相关内容时，不得直接污染 core。
5. 任何新目录、新任务、新规则都必须写入 docs。

## 按目录规则

### `packages/core/**`
- 允许：domain / usecases / state 的内部演化
- 禁止：引入宿主桥接、协议细节、链细节
- 必跑：core tests

### `packages/contracts/**`
- 允许：DTO、events、errors、auth schema 变更
- 禁止：把运行时实现塞进 contracts
- 必跑：typegen、contracts-check

### `packages/adapters/**`
- 允许：host/protocol/chain/storage/auth/telemetry 接入
- 禁止：新增核心业务规则
- 必跑：adapter tests / relevant replay

### `apps/**`
- 允许：启动、组合、路由、宿主配置
- 禁止：把业务规则写进启动层
- 必跑：app tests / E2E

### `workers/**`
- 允许：消费、索引、同步、异步作业
- 禁止：绕过 contracts/events
- 必跑：worker tests / replay

## 回滚规则

当出现以下情况时应立即停止继续改动并回滚到最近稳定点：

- contracts 与生成产物不一致
- verify 无法通过
- 边界被明显污染
- secrets / 安全基线被破坏
