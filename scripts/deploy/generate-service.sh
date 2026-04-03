#!/usr/bin/env bash
# Generate systemd service file for axum-api
# Usage: generate-service.sh BIN_PATH ENV_FILE USER GROUP
set -euo pipefail

BIN_PATH="${1:?BIN_PATH required}"
ENV_FILE="${2:?ENV_FILE required}"
USER="${3:-root}"
GROUP="${4:-root}"
WORK_DIR="$(pwd)"

cat > axum-api.service <<SERVICE_EOF
[Unit]
Description=Axum API Service
After=network.target

[Service]
Type=simple
User=${USER}
Group=${GROUP}
WorkingDirectory=${WORK_DIR}
EnvironmentFile=${ENV_FILE}
ExecStart=${BIN_PATH}
Restart=on-failure
RestartSec=5
# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ReadWritePaths=/var/lib/axum-api

[Install]
WantedBy=multi-user.target
SERVICE_EOF

echo "Generated axum-api.service"
