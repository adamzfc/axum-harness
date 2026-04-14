# Gates — unified quality entrypoints for humans and agents

# Local gate: keep warn-only during Phase 0 to avoid blocking evolution
gate-local:
    bun run scripts/gate.ts local --mode warn

# Pre-push gate: broader checks but still warn-only in Phase 0
gate-prepush:
    bun run scripts/gate.ts prepush --mode warn

# Phase 1 existence/import validation gate
gate-existence MODE='warn':
    bun run scripts/validate-existence.ts --mode {{MODE}}

# Phase 1 import dependency validation gate
gate-imports MODE='warn':
    bun run scripts/validate-imports.ts --mode {{MODE}}

# CI gate: strict once checks leave local warn-only mode
gate-ci:
    bun run scripts/gate.ts ci --mode strict

# Release gate: final strict entry before packaging/release
gate-release:
    bun run scripts/gate.ts release --mode strict
