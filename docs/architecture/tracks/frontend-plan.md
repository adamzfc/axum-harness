# 前端轨道计划

> 状态：下一主轨道
> 目标：让 `apps/web` 从历史遗留结构演进到“有明确目录边界、有独立 gate、有稳定调用边界”的前端自治层。

---

## 0. 当前现状

### 0.1 当前 `apps/web` 的事实目录

当前主要结构如下：

```text
apps/web/src/
├── routes/
│   ├── +layout.svelte
│   ├── +layout.ts
│   ├── +page.svelte
│   ├── +error.svelte
│   ├── (auth)/+layout.svelte
│   ├── (auth)/login/+page.svelte
│   └── (app)/
│       ├── +layout.svelte
│       ├── admin/+page.svelte
│       ├── agent/+page.svelte
│       ├── counter/+page.svelte
│       └── settings/+page.svelte
├── lib/
│   ├── components/
│   │   ├── index.ts
│   │   └── ui/*.svelte
│   ├── generated/
│   │   ├── api/*.ts
│   │   ├── auth/*.ts
│   │   └── events/*.ts
│   ├── ipc/
│   │   ├── bridge.ts
│   │   ├── auth.ts
│   │   └── agent.ts
│   ├── stores/
│   │   ├── auth.svelte.ts
│   │   └── theme.ts
│   └── utils/
│       ├── cn.ts
│       └── id.ts
├── app.css
├── app.html
├── hooks.client.ts
└── hooks.server.ts
```

### 0.2 当前问题判断

- 目录不是完全混乱，但明显是历史演进结果，不是经过系统设计的前端结构
- `ipc/bridge.ts` 与 `stores/auth.svelte.ts` 是当前较稳定的好抽象
- 页面层仍存在硬编码 API/IPC 调用与 layout 级 hardcoded guard/nav 逻辑
- `generated/` 能用，但链路不完整，且消费边界不够清晰
- 当前没有前端一等公民级别的 gate，只通过 `typecheck` 被动纳管

---

## 1. 本轨道的核心原则

### 1.1 不推翻已有有效抽象

优先保留并围绕这些能力演进：

- `$lib/ipc/bridge.ts`
- `$lib/ipc/auth.ts`
- `$lib/stores/auth.svelte.ts`
- 当前可用的 generated type 消费链

### 1.2 不把前端改造成“后端镜像”

前端不应机械复制 `domain/application/ports` 目录结构。

前端更合理的关注点是：

- routes
- api client
- runtime bridge
- guards / route metadata
- components / ui
- stores
- generated contracts

### 1.3 先立治理边界，再改实现

先建立前端规则真相源和 `gate-frontend`，再做大规模结构调整。

---

## 2. 目标目录与职责指导

### 2.1 建议目标结构

`apps/web/src/lib/` 后续建议收敛为：

```text
lib/
├── api/               # typed client 与资源级 API 封装
├── components/        # 可复用 UI 组件与组合组件
├── generated/         # contracts 生成物，只读消费
├── guards/            # auth / tenant / feature guard
├── ipc/               # 运行时桥接与桌面能力适配
├── route-meta/        # 导航/权限/标题等元数据，不是路由存在性真相源
├── stores/            # 前端状态管理
├── utils/             # 轻量工具函数
└── runtime/           # 可选：base url、env、platform 解析
```

### 2.2 每层职责

- `routes/`：页面与布局，只做展示、交互编排、load/redirect 级逻辑
- `lib/api/`：统一 HTTP/IPC typed 调用边界
- `lib/generated/`：只读生成物，不写手工业务逻辑
- `lib/ipc/`：只做运行时桥接，不承载页面业务流程
- `lib/guards/`：守卫逻辑和路由元信息查询
- `lib/route-meta/`：导航、标题、权限元数据；不是路由文件系统的替代
- `lib/stores/`：状态存储，不承担底层 transport 细节
- `lib/components/`：组件层，不直接依赖后端实现

### 2.3 当前明确禁止项

- 页面层直接 import 后端实现
- 页面层直接手写跨运行时 `invoke` / `fetch` 细节
- 在 `generated/` 中混入手写业务代码
- 用 route metadata 取代 SvelteKit 文件系统路由真相源
- 把前端目录规则写成后端 service 模板的翻版

---

## 3. 前端轨道执行顺序

### F1. 先建立前端治理边界

要做：

- 前端规则真相源文档
- `gate-frontend` 最小入口
- `validate-web-structure` 最小骨架

不做：

- 大规模页面改造
- typed client 大改
- tokens 抽取

### F2. 再做 Web 客户端结构重构

顺序建议：

1. 修 typegen 链路
2. 建统一 typed client
3. 抽离 guard / route metadata
4. 收敛 design tokens 真相源
5. 补最小组件测试与 E2E smoke

---

## 4. 关键技术判断

### 4.1 关于 typed client

- 必须建立统一 client
- 但不允许依赖“HTTP path 自动推导 Tauri command”这种脆弱约定
- 应使用显式 adapter 或显式 mapping

### 4.2 关于 route metadata

- 允许建立 route metadata 注册表
- 但它只能是导航/权限/标题元数据真相源
- 路由存在性仍以 SvelteKit 文件系统为真相源

### 4.3 关于 design tokens

- 当前 `app.css` 内联 tokens 是现实基础
- 只有在确实需要多 app / shared UI 复用时，才提升到 `packages/ui/tokens/`
- 不提前为了“看起来更架构化”而上升共享层

---

## 5. 完成标准

- `apps/web` 拥有独立的前端规则真相源
- `gate-frontend` 可单独运行
- 页面层调用边界统一
- routes / api / guards / generated / stores / components 职责清晰
- 前端重构完成后，不需要继续一边改页面、一边改全局 gate 定义
