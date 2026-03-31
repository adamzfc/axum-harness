# 测试检查清单 - 2026年3月

## 启动项目后需要检查和测试的内容

### 1. 开发环境验证 ✅

- [ ] Rust工具链正确安装 (`rustc --version`)
- [ ] Node.js/Bun正确安装 (`bun --version`)
- [ ] 依赖正确安装 (`cargo build`, `bun install`)

### 2. 后端API测试 ✅

#### 2.1 健康检查
```bash
# 测试健康检查端点
curl http://localhost:3001/healthz
# 预期: 200 OK

curl http://localhost:3001/readyz
# 预期: 200 OK
```

#### 2.2 Tenant API
```bash
# 初始化租户
curl -X POST http://localhost:3001/api/tenant/init \
  -H "Content-Type: application/json" \
  -d '{"user_sub": "test-user", "user_name": "Test User"}'
# 预期: 200 OK, 返回 tenant_id
```

#### 2.3 错误处理
```bash
# 测试空请求
curl -X POST http://localhost:3001/api/tenant/init \
  -H "Content-Type: application/json" \
  -d '{}'
# 预期: 422 Unprocessable Entity (验证错误)

# 测试无效JSON
curl -X POST http://localhost:3001/api/tenant/init \
  -H "Content-Type: application/json" \
  -d 'invalid'
# 预期: 400 Bad Request
```

### 3. 前端测试 ✅

#### 3.1 页面加载
- [ ] 首页正常加载 (http://localhost:5173)
- [ ] 登录页正常加载
- [ ] 管理后台正常加载

#### 3.2 交互测试
- [ ] 登录表单提交
- [ ] 导航切换
- [ ] 模态框打开/关闭

#### 3.3 响应式设计
- [ ] 桌面端 (1920x1080)
- [ ] 平板端 (768x1024)
- [ ] 移动端 (375x667)

### 4. Tauri桌面应用测试 ✅

#### 4.1 应用启动
- [ ] 应用正常启动
- [ ] 窗口居中显示
- [ ] 系统托盘图标显示
- [ ] 窗口控件正常工作 (最小化/最大化/关闭)

#### 4.2 功能测试
- [ ] 本地存储正常工作
- [ ] 文件系统访问正常
- [ ] 深度链接正常工作

#### 4.3 性能测试
- [ ] 启动时间 < 3秒
- [ ] 内存占用 < 200MB
- [ ] CPU占用 < 10% (空闲时)

### 5. 安全测试 ✅

#### 5.1 认证
- [ ] 未登录无法访问API
- [ ] JWT过期处理
- [ ] 错误信息不泄露敏感信息

#### 5.2 CSP
- [ ] 内联脚本被阻止
- [ ] 外部资源加载受限

#### 5.3 输入验证
- [ ] SQL注入防护
- [ ] XSS防护
- [ ] CSRF防护

### 6. E2E测试 ✅

```bash
# 运行E2E测试
cd apps/client/web/app
bun run test:e2e
```

- [ ] 登录流程
- [ ] 登出流程
- [ ] 数据创建/读取/更新/删除
- [ ] 错误处理流程

### 7. 性能基准测试 ✅

#### 7.1 API响应时间
```bash
# 测试响应时间
time curl http://localhost:3001/healthz
# 目标: < 100ms
```

#### 7.2 并发测试
```bash
# 并发请求测试
ab -n 100 -c 10 http://localhost:3001/healthz
# 目标: 无错误, 平均响应 < 200ms
```

#### 7.3 前端加载时间
- [ ] 首次内容绘制 (FCP) < 1.5s
- [ ] 最大内容绘制 (LCP) < 2.5s
- [ ] 首次输入延迟 (FID) < 100ms

### 8. 浏览器兼容性测试 ✅

- [ ] Chrome (最新)
- [ ] Firefox (最新)
- [ ] Safari (最新)
- [ ] Edge (最新)

### 9. CI/CD验证 ✅

- [ ] GitHub Actions工作流正常运行
- [ ] 所有质量门禁通过
- [ ] 测试报告生成

### 10. 监控和日志 ✅

- [ ] 日志正确输出
- [ ] 错误追踪可用
- [ ] 性能指标收集

---

## 快速验证命令

### 本地快速测试
```bash
# 启动后端
cd servers/api && cargo run

# 启动前端
cd apps/client/web/app && bun run dev

# 运行测试
./scripts/quick-test.sh

# 完整质量门禁
./scripts/test-verify.sh
```

### 生产构建前检查
```bash
# 1. 格式化
cargo fmt --all -- --write

# 2. Lint
cargo clippy --workspace --all-targets -- -D warnings

# 3. 测试
cargo test --workspace

# 4. 构建
cargo build --workspace --release
```

---

## 问题报告模板

如果发现问题，请报告:

1. **环境信息**
   - OS版本:
   - Rust版本:
   - Node/Bun版本:

2. **问题描述**
   - 预期行为:
   - 实际行为:

3. **复现步骤**
   - 步骤1:
   - 步骤2:

4. **日志/截图**
   - 错误日志:
   - 截图:

---

## 联系支持

- 问题: 请创建GitHub Issue
- 文档: 查看 `docs/` 目录
- 讨论: 加入项目Discord/Slack