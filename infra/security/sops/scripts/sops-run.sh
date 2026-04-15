#!/usr/bin/env bash
# Run a binary with SOPS-decrypted environment variables
#
# Usage: bash infra/security/sops/scripts/sops-run.sh <deployable> <environment> -- <command>
# Example: bash infra/security/sops/scripts/sops-run.sh web-bff dev -- cargo run -p web-bff
#
# This script:
# 1. Decrypts the secrets file for the specified deployable/environment
# 2. Exports all stringData values as environment variables
# 3. Runs the specified command with those env vars
# 4. No .env file is created

set -euo pipefail

DEPLOYABLE="${1:?Usage: sops-run.sh <deployable> <env> -- <command>}"
ENV="${2:-dev}"
shift 2

# Remove the -- separator if present
if [ "${1:-}" = "--" ]; then
  shift
fi

if [ $# -eq 0 ]; then
  echo "ERROR: No command specified"
  echo "Usage: sops-run.sh <deployable> <env> -- <command>"
  echo "Example: sops-run.sh web-bff dev -- cargo run -p web-bff"
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOPS_DIR="$(dirname "$SCRIPT_DIR")/$ENV"
ENC_FILE="$SOPS_DIR/${DEPLOYABLE}.enc.yaml"

if [ ! -f "$ENC_FILE" ]; then
  echo "ERROR: Encrypted secrets file not found: $ENC_FILE"
  echo ""
  echo "Available deployables for $ENV:"
  ls -1 "$SOPS_DIR"/*.enc.yaml 2>/dev/null | xargs -n1 basename | sed 's/\.enc\.yaml//' || echo "  (none)"
  exit 1
fi

if ! command -v sops &> /dev/null; then
  echo "ERROR: sops is not installed. Install with: mise install"
  exit 1
fi

echo "Running: $*"
echo "Environment: $ENV"
echo "Deployable: $DEPLOYABLE"
echo ""

# Use sops exec-env to decrypt and run with env vars
sops exec-env "$ENC_FILE" "$@"
