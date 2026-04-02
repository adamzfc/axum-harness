# 06. Rust / Tauri / SvelteKit / Svelte / TypeScript 工程基线

> **文档版本**: 1.2.0  
> **最后更新**: 2026-04-01

## 核心原则：技术栈演进

**技术栈应该持续演进，拥抱前沿技术。**

本章节定义的技术版本都是可演进的。我们的原则是：
1. **前沿导向**：选择代表最新技术思想的版本
2. **Rust 优先**：优先选择 Rust 生态中的技术
3. **接口隔离**：通过接口隔离技术实现细节
4. **渐进式升级**：避免大规模破坏性变更

## 6.1 Rust 基线

### 当前推荐版本

- **Rust 版本**: 2024 edition (或最新稳定版)
- **MSRV (Minimum Supported Rust Version)**: 1.85+ (支持最新特性)
- **工具链**: stable, 含 `rustfmt`、`clippy`
- **特性使用**: 积极使用 Rust 新特性，如 `async fn` in traits、`impl Trait` in return position

### 默认要求

- `cargo fmt --check`
- `clippy -D warnings`
- public API 要有文档注释
- 关键路径使用结构化 tracing
- 明确错误类型，不乱用 `anyhow` 穿透到边界
- feature flags 有命名规范
- 不在 runtime crate 写业务逻辑

### crate 设计建议

- core crates 只暴露稳定接口
- adapter crates 可以快迭代，但要被 tests / fixtures 约束
- 避免过度切分 crate；只有在复用或编译边界明显时拆

### Rust 开发红线

- 禁止把宿主特性放进 domain
- 禁止把 HTTP transport model 当 domain model
- 禁止把 secrets 直接序列化进可发往前端的 config

## 6.2 Tauri 基线

### 默认要求

- capability 最小授权
- command 只做 bridge
- config / secret / path 分离
- 深链、通知、文件系统、更新等系统能力通过 adapter 封装
- 桌面与移动端差异 feature-gated
- 插件引入必须说明目的、权限、回滚方案

### Tauri 配置基线

- 明确 dev / prod 配置差异
- CSP 不留空
- 按窗口拆 capability
- 路径使用 app-specific data/config/cache 目录
- 不写死本机绝对路径

### Tauri 开发红线

- 禁止 command 内写业务决策
- 禁止前端直接接触敏感 secret
- 禁止插件默认全开权限

## 6.3 SvelteKit 基线

### 默认要求

- `apps/client/web/app` 是 canonical app
- 页面仅组合 feature UI 与 data client
- 路由级 auth / layout / loading 模式一致
- 区分 server-side data 与 client state
- 宿主差异不直接散落在页面深处

### 数据流建议

- contracts 生成 types -> data client -> feature UI
- 表单、错误、重试、空态有统一模式
- host-specific bridge 通过 adapter 进入

## 6.4 Svelte 基线

### 当前推荐版本

- **Svelte 版本**: 5.x (最新稳定版，5.49+)
- **语法模式**: Runes 模式 (新代码必须使用)
- **响应式系统**: 细粒度信号-based 响应式

### 为什么选择 Svelte 5

- **性能领先**：2026 年基准测试显示 SvelteKit 2.x 领先其他框架（1200 RPS）
- **编译时优化**：Svelte 5 的编译器将响应式代码编译为高效的原生 JavaScript
- **Runes 系统**：显式的响应式原语，比 Svelte 4 的编译器魔法更可预测
- **类型安全**：与 TypeScript 深度集成，提供更好的类型推断

### 默认要求

- **必须使用 Runes 风格**：`$state`、`$derived`、`$effect`、`$props`
- **禁止使用旧语法**：不再使用 `$:` 响应式声明
- store 只承载跨组件 / 跨 session 共享状态
- 组件副作用可追踪、可清理
- design system 和业务组件分层清晰

### Svelte 5 Runes 最佳实践

```svelte
<script>
  // ✅ 正确：使用 Runes
  let count = $state(0);
  let doubled = $derived(count * 2);
  
  $effect(() => {
    console.log('count changed:', count);
  });
  
  // ✅ 组件 props
  let { name, age = 25 } = $props();
  
  // ❌ 错误：不再使用旧语法
  // let count = 0;
  // $: doubled = count * 2;
</script>
```

### Svelte 开发红线

- **禁止使用 `$:` 语法**：必须使用 Runes
- 禁止在组件内偷偷做全局副作用
- 禁止将宿主桥逻辑散落在任意业务组件中
- 禁止把远程状态和本地 UI 过度混合

### 选型已确定

**SvelteKit 2 + Svelte 5 是确定的前端选型，不再考虑其他替代方案。**

## 6.5 TypeScript 基线

### 默认要求

- `strict: true`
- project references
- path aliases 统一
- generated contract types 优先
- 不用 `any` 作为逃生舱
- shared utils 只能放与业务无关的通用工具

### TS 开发红线

- 禁止手写 mirror DTO
- 禁止 feature 间用隐式结构共享数据
- 禁止 host-specific typing 污染 canonical app

## 6.6 UI / Design System 基线

- tokens、icons、themes 集中管理
- 组件分层：primitive / pattern / feature
- 统一 a11y 规范
- 表单交互和反馈状态一致
- 文案、主题、i18n 预留位置
