# Docs Index

> 目标：把 `docs/` 收敛成 agent 与开发者都能快速进入后端开发轨道的唯一文档入口。
>
> 本仓库当前只关注后端默认学习路径，不把 `apps/**` 作为默认上下文。

## 1. 默认入口

默认阅读顺序直接跟随 `AGENTS.md`，本文只补 `docs/` 内部入口：

1. `docs/architecture/repo-layout.md`
2. `docs/operations/counter-service-reference-chain.md`
3. 按任务需要再读对应 owner 文档，例如 `services/*/README.md`、`workers/*/README.md`、`ops/runbooks/**`

文档与代码冲突时，以代码、schema、validator、gate 为准。

## 2. 文档分层

A 类：仓库级事实与边界

1. `AGENTS.md`
2. `agent/codemap.yml`
3. `agent/manifests/routing-rules.yml`
4. `agent/manifests/gate-matrix.yml`
5. `docs/architecture/repo-layout.md`

B 类：局部 owner 文档与 reference chain

1. `docs/operations/counter-service-reference-chain.md`
2. `packages/contracts/STRUCTURE.md`
3. `services/*/README.md`
4. `workers/*/README.md`
5. `infra/local/README.md`
6. `ops/runbooks/**`

不进入默认上下文的内容：未来态散文、和当前代码脱节的说明、未挂接到 `counter-service` 参考链的独立工具链描述。

## 3. 默认锚点

`counter-service` 是默认后端参考锚点，不是最小 demo。

它同时承载两条链：

1. 业务主链：`service -> contracts -> server -> outbox -> relay -> projector`
2. 工程横切链：`platform model -> secrets -> deploy -> GitOps -> promotion -> drift -> runbook`

如果一个横切能力没有挂接到这条 reference chain，它就还不是默认开发惯性。

## 4. 当前收敛原则

1. 文档只保留当前稳定事实、关键概念、默认变更顺序和入口路径。
2. 规则优先下沉到 schema、validator、scripts、gate，不继续膨胀散文。
3. 新文档如果不能帮助 agent 更稳定地改代码，就不应进入默认入口。
4. 高漂移、低价值、历史性内容进入 `docs/archive/`。

## 5. 一句话结论

`docs/README.md` 只负责告诉 agent 和开发者：默认读什么、默认别读什么、后端参考锚点在哪里。更细的规则回到 `AGENTS.md`、`codemap.yml` 和真实代码。
