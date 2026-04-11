# Scripts

TypeScript build, test, and utility scripts.

## Structure

```
scripts/
├── lib/
│   └── spawn.ts          # Cross-platform process spawner
├── e2e/
│   ├── run-e2e-gate.ts   # Full E2E gate runner
│   ├── runtime-preflight.ts  # Pre-E2E environment check
│   └── runtime-preflight.test.ts
├── deploy/
│   └── generate-service.sh  # ⚠️ ONLY shell script — service skeleton generator
├── boundary-check.ts     # Architecture boundary violation checker
├── typegen.ts            # Contract → TypeScript type generator
├── doctor.ts             # Toolchain health diagnostic
├── dev-desktop.ts        # Desktop dev environment launcher
└── CROSS-PLATFORM.md     # Cross-platform development guidelines
```

## Usage

Scripts are invoked via `just` recipes, not directly:

```bash
just typegen      # → bun run scripts/typegen.ts
just doctor       # → bun run scripts/doctor.ts
just boundary-check  # → bun run scripts/boundary-check.ts
```

## Rules

- **TypeScript only** for cross-platform compatibility (run via Bun)
- **No `.sh` scripts** except `deploy/generate-service.sh` (service skeleton generator)
- All scripts use `scripts/lib/spawn.ts` for process management
