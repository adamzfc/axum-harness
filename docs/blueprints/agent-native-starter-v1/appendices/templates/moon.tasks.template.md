# moon 任务模板

## repo 级任务建议

```yaml
tasks:
  setup:
    command: "just setup"

  doctor:
    command: "just doctor"

  dev-web:
    command: "bun --cwd apps/client/web/app dev"

  dev-desktop:
    command: "bun --cwd apps/client/desktop dev"

  dev-api:
    command: "bun --cwd servers/api dev"

  dev-fullstack:
    deps: ["repo:dev-api", "repo:dev-web"]

  typegen:
    command: "just typegen"

  verify:
    deps:
      - "repo:fmt"
      - "repo:lint"
      - "repo:typecheck"
      - "repo:test-unit"
      - "repo:test-integration"

  evals-run:
    command: "just evals"
```

## 设计要点

- repo 级任务应只暴露稳定入口
- 复杂编排交给 moon 的依赖图
- 人类和 agent 都应优先调用这些任务
