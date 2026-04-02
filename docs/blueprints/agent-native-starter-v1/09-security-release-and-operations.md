# 09. 安全、发布与运维基线

## 9.1 安全不是补丁层

对 agent-first 仓库来说，安全问题更危险，因为错误可能被快速复制。

所以安全必须体现在：

- 默认配置
- 目录边界
- secret 管理
- 权限最小化
- 测试与扫描
- 发布前门禁

## 9.2 Secret 管理原则

- secret 与普通 config 分离
- secret 不暴露到前端可序列化对象
- dev secret 也不硬编码进长期配置
- 路径、token、client secret 不写死在源码
- 日志中禁止打印 secret 与敏感 payload

## 9.3 Tauri 权限基线

- capability 按窗口最小授权
- 插件引入需要白名单
- CSP 不留空
- file system / shell / updater 等能力需要显式说明
- deep link / protocol handler 明确边界与回退机制

## 9.4 发布基线

### 必备能力

- semver 版本策略
- changelog
- 产物命名规范
- 多平台构建策略
- dry-run release
- release verification
- 回滚路径

### 产物分类

- desktop app bundles
- web build artifacts
- server / worker containers or binaries
- codegen artifacts（内部）
- docs / release notes

## 9.5 供应链与审计

V1 建议至少有：

- Rust 依赖审计
- Bun/NPM 依赖审计
- license 检查
- secret 扫描
- 基本 provenance / build metadata

## 9.6 运维文档要求

docs 至少包含：

- 环境变量清单
- 本地开发依赖说明
- release 手册
- 回滚手册
- worker replay 手册
- 事故排查手册

## 9.7 事故排查最小信息集

每次严重失败事件，至少能拿到：

- trace id
- build/release version
- host / platform
- relevant config profile
- contract version
- recent patch summary
- test/eval result snapshot

## 9.8 V1 最低落地

- secrets scan
- dependency audit
- release dry-run
- changelog discipline
- desktop / web / server 的分离产物策略
- 事故排查模板
