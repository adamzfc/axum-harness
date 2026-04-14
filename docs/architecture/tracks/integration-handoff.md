# 集成轨道交接单

handoff:
  status: blocked-by-frontend
  summary:
    - 集成轨道现在只应做规划，不应提前施工
    - 等前端轨道完成独立 gate 与主要目录稳定后，再开始 monorepo 收尾
  prerequisites:
    - frontend gate 已建立
    - frontend 目录与调用边界稳定
    - backend 仅剩维护类小修
  do_not_do:
    - 不提前修改全局 gate 组合
    - 不在 frontend 结构未定前写死 CI / release 组合策略
