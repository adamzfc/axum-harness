# Agent-Native Starter 文档集

> **文档版本**: 1.2.0  
> **最后更新**: 2026-04-01  
> **状态**: 活跃维护中

这套文档用于指导一个 **业务无关、跨平台、agent-first** 的 monorepo 长期演化。

## 技术选型哲学

### 核心理念：Rust 优先，前沿导向，Agent-Friendly

**我们选择 Rust 不是因为保守，而是因为它能创造出更好的产品。**

- **Rust 生态优先**：从 Web 框架（Axum）到桌面应用（Tauri），Rust 生态已经成熟
- **前沿技术拥抱**：不选择"大厂保守实践"，选择因为"它是更好的解决方案"而选择的技术
- **解决长期痛点**：选择融合旧技术优秀思想并产生新思想的技术
- **Agent-Friendly 开发**：基建要让 agent 开发更友好，文档清晰、API 设计良好、类型系统完善

### 反对"大厂保守实践"

- **大厂实践不一定适合独立开发者**：大厂有大量人力和资源，独立开发者需要更高效的工具
- **大厂实践往往滞后**：大厂技术栈更新慢，往往使用几年前的"最佳实践"
- **我们的选择标准**：性能优先、开发效率、前沿但不实验、Rust 生态

详细信息请参考 `00-index.md` 中的技术选型哲学部分。

## 文档版本管理

### 版本号规则

- **主版本号**：架构级变更，不兼容的修改
- **次版本号**：新增功能或重大改进，向后兼容
- **修订号**：小的修正和改进，向后兼容

### 更新流程

1. 识别需要更新的部分
2. 评估影响范围
3. 更新文档版本号
4. 更新"最后更新"日期
5. 记录变更历史

### 文档与代码的关系

**文档是规范，代码是实现。**

- 文档定义目标状态和规范
- 代码应该向文档靠拢
- 不应该反向修改文档以适应当前代码状态
- 当代码与文档不一致时，应该优先更新代码

## 建议阅读顺序

### 第一部分：核心理念

1. `blueprints/agent-native-starter-v1/00-index.md` - 文档总览和核心理念
2. `blueprints/agent-native-starter-v1/01-north-star-and-principles.md` - 北极星和设计原则

### 第二部分：架构设计

3. `blueprints/agent-native-starter-v1/02-repo-structure.md` - 仓库结构蓝图
4. `blueprints/agent-native-starter-v1/03-toolchain-and-taskgraph.md` - 工具链与任务图
5. `blueprints/agent-native-starter-v1/04-contracts-typegen-and-boundaries.md` - 契约、类型闭环与边界

### 第三部分：运行时设计

6. `blueprints/agent-native-starter-v1/05-runtime-features-and-adapters.md` - 运行时、功能与适配器
7. `blueprints/agent-native-starter-v1/06-engineering-standards-rust-tauri-svelte.md` - 工程基线与技术演进

### 第四部分：质量与观测

8. `blueprints/agent-native-starter-v1/07-observability-testing-and-evals.md` - 可观测性、测试与评估

### 第五部分：Agent 运行层

9. `blueprints/agent-native-starter-v1/08-agent-runtime-and-feedback-loops.md` - Agent 运行时与反馈闭环

### 第六部分：安全与运维

10. `blueprints/agent-native-starter-v1/09-security-release-and-operations.md` - 安全、发布与运维基线

### 第七部分：路线图与实施

11. `blueprints/agent-native-starter-v1/10-roadmap-v1-v3.md` - 路线图：V1 / V2 / V3
12. `blueprints/agent-native-starter-v1/11-rule-matrix-and-checklists.md` - 规则矩阵与检查清单
13. `blueprints/agent-native-starter-v1/12-migration-path.md` - 渐进式迁移路径
14. `blueprints/agent-native-starter-v1/13-subdomain-selection-guide.md` - 子领域选型研究指南

## 附录

- `appendices/templates/AGENTS.rule-matrix.template.md` - Agent 规则矩阵模板
- `appendices/templates/moon.tasks.template.md` - moon 任务模板
- `appendices/templates/just.recipes.template.md` - Just 命令模板
- `appendices/templates/repo.docs.map.template.md` - 仓库文档地图模板

## 文档维护指南

### 贡献流程

1. 识别需要更新的部分
2. 创建分支进行修改
3. 更新文档版本号和日期
4. 提交 Pull Request
5. 代码审查
6. 合并到主分支

### 文档标准

- 使用 Markdown 格式
- 包含版本号和最后更新日期
- 提供清晰的标题和结构
- 包含具体的示例和检查清单
- 保持语言简洁明了

### 文档审查清单

- [ ] 版本号已更新
- [ ] 最后更新日期已更新
- [ ] 内容准确无误
- [ ] 示例可运行
- [ ] 链接有效
- [ ] 格式一致
- [ ] 语言清晰

## 技术选型解耦原则

**所有技术选型都是独立、解耦、可替代的。**

本蓝图的核心理念是：技术选型不应该成为系统的耦合点。每个技术选择都应该：

1. **独立决策**：每个技术选型可以独立评估和替换
2. **接口隔离**：通过清晰的接口/契约隔离技术实现细节
3. **渐进演进**：技术栈可以渐进式升级，不需要大规模重构
4. **选项价值**：保持对未来更好技术的开放性

### 选型已确定

以下选型已确定，不再频繁变更：

- **前端**：SvelteKit 2 + Svelte 5（确定）
- **桌面**：Tauri v2（确定）
- **后端**：Axum（确定）
- **任务编排**：moon（确定）
- **包管理器**：Bun（确定）

### 子领域选型

当需要深入子领域时，参考 `13-subdomain-selection-guide.md` 进行研究和选型。

详细信息请参考 `00-index.md` 中的技术选型部分。

## 快速开始

如果你是第一次接触这套文档：

1. 从 `00-index.md` 开始，了解核心理念
2. 阅读 `01-north-star-and-principles.md`，理解设计原则
3. 根据你的需求，选择性阅读其他文档
4. 参考 `12-migration-path.md` 开始实施

如果你已经熟悉这套文档：

1. 查看 `10-roadmap-v1-v3.md` 了解当前阶段
2. 参考 `11-rule-matrix-and-checklists.md` 进行实施
3. 使用 `12-migration-path.md` 规划迁移
4. 定期检查文档更新

## 常见问题

### Q: 文档与代码不一致怎么办？

A: 文档是规范，代码是实现。应该优先更新代码以符合文档规范。如果文档确实需要更新，请按照文档维护指南进行。

### Q: 如何选择技术栈？

A: 参考 `03-toolchain-and-taskgraph.md` 中的工具选型决策框架。核心选型已确定，子领域选型参考 `13-subdomain-selection-guide.md`。

### Q: 如何开始迁移？

A: 参考 `12-migration-path.md` 中的渐进式迁移路径。从阶段 0 开始，逐步完成每个阶段。

### Q: 文档多久更新一次？

A: 文档应该持续更新。每次技术栈变更、架构调整或发现文档问题时都应该更新文档。建议至少每季度检查一次文档的准确性。
