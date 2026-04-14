# 后端轨道交接单

handoff:
  status: active-maintenance
  summary:
    - 后端主架构已基本完成，当前重点是门禁、CI、hook 收口
    - 不建议再开启大规模后端结构重构
  next_actions:
    - 收口 backend CI 与 workflow 分层
    - 只处理阻塞当前开发的后端治理问题
  do_not_do:
    - 不继续扩大战线到新的后端架构重构
    - 不替 frontend 定义目录真相源
    - 不把 frontend 规则塞进 backend gate
