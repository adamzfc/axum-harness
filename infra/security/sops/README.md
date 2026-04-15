# SOPS Secrets Management

> **Backend Configuration Policy**: `Kustomize + SOPS + age + Flux` is the ONLY configuration path.
> **`.env` files are NOT used for backend services.**

---

## Quick Start

### First-Time Setup

```bash
# 1. Install tools (age + sops)
mise install

# 2. Generate age key
just sops-gen-age-key

# 3. Update .sops.yaml with your public key
just sops-show-age-key
# Copy the output to .sops.yaml

# 4. Encrypt secrets for dev
just sops-encrypt-dev web-bff
just sops-encrypt-dev outbox-relay-worker

# 5. Run services
just sops-run web-bff              # No cluster needed
just sops-run outbox-relay-worker
```

### Daily Development

```bash
# Quick inner loop (no cluster)
just sops-run web-bff

# With cluster
just sops-reconcile dev
just deploy-prod dev
```

---

## Directory Structure

```
infra/security/sops/
├── templates/              # Plaintext templates (DO NOT commit with real values)
│   ├── dev/
│   │   ├── web-bff.yaml
│   │   ├── outbox-relay-worker.yaml
│   │   └── counter-service.yaml
│   └── staging/
│       ├── web-bff.yaml
│       └── outbox-relay-worker.yaml
├── dev/                    # Encrypted secrets (dev environment)
│   ├── web-bff.enc.yaml
│   ├── outbox-relay-worker.enc.yaml
│   └── counter-service.enc.yaml
├── staging/                # Encrypted secrets (staging environment)
├── prod/                   # Encrypted secrets (production environment)
├── scripts/                # Helper scripts
│   ├── apply-secrets.sh    # Apply secrets to Kubernetes cluster
│   └── sops-run.sh         # Run binary with decrypted env vars
├── AGE-KEY-MANAGEMENT.md   # Detailed age key management guide
└── README.md               # This file
```

---

## Available Commands

| Command | Description |
|---------|-------------|
| `just sops-gen-age-key` | Generate new age key pair |
| `just sops-show-age-key` | Show age public key |
| `just sops-encrypt-dev <deployable>` | Encrypt secrets for dev |
| `just sops-encrypt-staging <deployable>` | Encrypt secrets for staging |
| `just sops-edit <deployable> <env>` | Edit encrypted secrets |
| `just sops-run <deployable>` | Run binary with decrypted env vars |
| `just sops-reconcile <env>` | Apply secrets to cluster |
| `just sops-setup-flux-secret` | Create Flux SOPS secret |
| `just sops-validate` | Validate SOPS configuration |

---

## Available Deployables

| Deployable | Description | Status |
|---|---|---|
| `web-bff` | Web BFF server (Axum) | ✅ Active |
| `outbox-relay-worker` | Outbox relay worker | ✅ Active |
| `counter-service` | Counter service (Phase 1+) | 🔄 Planned |

---

## Workflows

### Adding a New Deployables

1. Create template:
   ```bash
   cp infra/security/sops/templates/dev/web-bff.yaml \
      infra/security/sops/templates/dev/<new-deployable>.yaml
   $EDITOR infra/security/sops/templates/dev/<new-deployable>.yaml
   ```

2. Encrypt:
   ```bash
   just sops-encrypt-dev <new-deployable>
   ```

3. Commit:
   ```bash
   git add infra/security/sops/dev/<new-deployable>.enc.yaml
   git commit -m "Add encrypted secrets for <new-deployable>"
   ```

### Rotating Keys

```bash
# 1. Generate new key
just sops-gen-age-key

# 2. Update .sops.yaml

# 3. Re-encrypt all secrets
for file in infra/security/sops/**/*.enc.yaml; do
  sops updatekeys --yes "$file"
done

# 4. Update Flux secret
just sops-setup-flux-secret

# 5. Commit
git add .sops.yaml infra/security/sops/
git commit -m "Rotate SOPS encryption key"
```

---

## Troubleshooting

See [AGE-KEY-MANAGEMENT.md](./AGE-KEY-MANAGEMENT.md) for detailed troubleshooting.

Quick fixes:

| Problem | Solution |
|---------|----------|
| "No matching key" | `just sops-show-age-key` and update `.sops.yaml` |
| "Decryption failed" | `export SOPS_AGE_KEY_FILE=~/.config/sops/age/key.txt` |
| Want to use `.env` | **Not allowed.** Use `just sops-run <deployable>` instead |

---

## Policy

See [docs/operations/backend-config-policy.md](../../../docs/operations/backend-config-policy.md) for the complete backend configuration policy.

**Key points:**

1. Backend binaries consume standard environment variables
2. Environment variables are injected via SOPS/Kustomize/Flux
3. `.env` files are NOT used for backend services
4. Local dev uses the same config path as production (just different overlay)
5. The only non-cluster dev path is `sops exec-env` (does not create `.env` files)
