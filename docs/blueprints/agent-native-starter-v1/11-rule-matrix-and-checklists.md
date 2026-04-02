# 11. 规则矩阵与检查清单

## 11.1 目录规则矩阵

| 改动目标 | 允许目录 | 禁止目录 | 必跑验证 |
|---|---|---|---|
| 修改 contracts | `packages/contracts/**` | 直接改 app 内 mirror types | `repo:typegen`, `repo:contracts-check`, tests |
| 修改 feature | `packages/features/**` | 直接改 host adapter 承载业务逻辑 | feature tests, `repo:verify` |
| 修改 core | `packages/core/**` | 依赖 host/protocol/chain | core tests, `repo:verify` |
| 修改 host adapter | `packages/adapters/hosts/**`, `apps/client/**` | 直接污染 core | host tests, E2E |
| 修改 protocol worker | `workers/protocols/**`, `packages/adapters/protocols/**`, `packages/contracts/events/**` | 绕过 event contracts | replay, worker tests |
| 修改 chain adapter | `packages/adapters/chains/**`, `workers/chains/**` | 在 feature 里写链协议细节 | adapter tests |
| 修改 UI | `packages/ui/**`, `apps/client/web/app/**` | 在组件里埋宿主桥与 secret | component tests |
| 修改 agent 规则 | `.agents/**`, `docs/**` | 无对应 rubric / playbook | evals |

## 11.2 PR / Patch 检查清单

### 所有改动都要检查
- [ ] 改动是否落在正确目录
- [ ] 是否引入了新的镜像类型
- [ ] 是否需要更新 contracts
- [ ] 是否需要运行 typegen
- [ ] 是否补了对应 tests
- [ ] 是否补了 docs / examples / playbooks
- [ ] 是否改变了 release / security 假设
- [ ] 是否会影响 agent skill 或 rubric

### contracts 改动额外检查
- [ ] TS 类型已同步
- [ ] schema / openapi 已同步
- [ ] fixture / eval dataset 已同步
- [ ] 兼容性变化已记录

### host 改动额外检查
- [ ] capability / 权限是否变化
- [ ] 是否新增宿主特定桥接
- [ ] 是否影响 canonical app 行为
- [ ] 是否需要 host-specific E2E

### worker 改动额外检查
- [ ] payload 是否进入 contracts/events
- [ ] replay fixture 是否更新
- [ ] 重试 / 死信 / 幂等性是否考虑
- [ ] 可观测性字段是否足够

## 11.3 发布前检查清单

- [ ] 所有 verify 通过
- [ ] 关键 E2E 通过
- [ ] eval suites 通过
- [ ] secrets scan 通过
- [ ] dependency audit 通过
- [ ] changelog 已更新
- [ ] release notes 已生成
- [ ] rollback 路径可执行

## 11.4 Agent 执行检查清单

- [ ] 读取 AGENTS.md
- [ ] 确认适用 skill / playbook
- [ ] 明确允许改动目录
- [ ] 先更新 contracts 再改调用方
- [ ] 运行必要验证
- [ ] 记录失败原因与补丁说明
- [ ] 必要时更新 rubric / eval dataset

## 11.5 决策检查清单

引入新技术前必须回答：

1. 它属于永久核心 / 高概率扩展 / 实验边车 哪一层？
2. 它进入哪个目录？
3. 它依赖哪些 contracts？
4. 它影响哪些任务图？
5. 它需要哪些测试与 evals？
6. 它失败时如何回滚？
7. 它是否会让 agent 更容易犯错？
