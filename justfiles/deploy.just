# Deploy — production deployment (systemd + axum binary)

# 在目标服务器上部署并启动 axum API 服务
deploy-api:
    @systemctl is-active --quiet axum-api.service && echo "API service is running" || echo "API service will be started"
    sudo systemctl daemon-reload
    sudo systemctl enable axum-api.service
    sudo systemctl restart axum-api.service
    sudo systemctl status axum-api --no-pager

# 停止 API 服务
stop-api:
    sudo systemctl stop axum-api.service
    @echo "API service stopped"

# 查看 API 服务日志
logs-api:
    sudo journalctl -u axum-api.service -f --no-pager

# 检查 storage-turso 编译
check-storage-turso:
    cargo check -p runtime_server

# 构建桌面 E2E 二进制
build-desktop-bin:
    cargo build -p native-tauri --profile e2e --features e2e-testing

# 生成 systemd service 文件（需在目标服务器执行）
# 需要 BIN_PATH, ENV_FILE, USER, GROUP 变量
generate-service BIN_PATH='' ENV_FILE='' USER='root' GROUP='root':
    @test -n "{{BIN_PATH}}" || (echo 'Usage: just generate-service BIN_PATH=/path/to/binary ENV_FILE=/path/to/.env USER=appuser GROUP=appgroup' && exit 1)
    @test -n "{{ENV_FILE}}" || (echo 'Usage: just generate-service BIN_PATH=/path/to/binary ENV_FILE=/path/to/.env USER=appuser GROUP=appgroup' && exit 1)
    bash scripts/deploy/generate-service.sh "{{BIN_PATH}}" "{{ENV_FILE}}" "{{USER}}" "{{GROUP}}"
    @echo "Generated axum-api.service — copy to /etc/systemd/system/ and run: just deploy-api"
