# 前端轨道交接单

handoff:
  status: next-up
  summary:
    - `apps/web` 有可保留的桥接层和 auth store，但整体目录结构仍是历史演进结果
    - 当前前端最需要的是独立治理边界，而不是立刻全面重写页面
  first_step:
    - 先执行 F1：前端规则真相源、`gate-frontend`、`validate-web-structure`
  do_not_do:
    - 不直接大规模改 `routes/` 页面
    - 不把 frontend 结构硬套成 backend service 模板
    - 不顺手改 monorepo 总体 gate 组合方式
  references:
    - docs/architecture/tracks/frontend-plan.md
    - docs/architecture/tracks/frontend-progress.md
