/**
 * Verify Replay — ensure replay/rebuild reference hooks exist for worker and golden lanes.
 *
 * Usage:
 *   bun run scripts/verify-replay.ts [--mode warn|strict]
 */

import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';

type Mode = 'warn' | 'strict';

function parseMode(argv: string[]): Mode {
  let mode: Mode = 'warn';
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--mode') {
      const value = argv[index + 1];
      if (value === 'warn' || value === 'strict') {
        mode = value;
        index += 1;
        continue;
      }
    }

    console.error(`Unknown argument: ${arg}`);
    console.error('Usage: bun run scripts/verify-replay.ts [--mode warn|strict]');
    process.exit(1);
  }
  return mode;
}

function main(): number {
  const mode = parseMode(process.argv.slice(2));
  const checks: Array<[string, string, RegExp | null]> = [
    ['workers/projector/src/replay/mod.rs', 'projector replay module missing', null],
    ['workers/projector/README.md', 'projector README missing replay/rebuild guidance', /replay|rebuild/i],
    ['services/counter-service/README.md', 'counter reference missing projection/replay note', /projection|replay/i],
    ['platform/model/deployables/projector-worker.yaml', 'projector deployable missing async projection profile', /async-projection/],
    ['verification/golden/README.md', 'golden README missing replay lane documentation', /replay/i],
  ];

  const failures: string[] = [];

  for (const [relativePath, message, contentRule] of checks) {
    const absolutePath = path.join(process.cwd(), relativePath);
    if (!existsSync(absolutePath)) {
      failures.push(`${relativePath}: ${message}`);
      continue;
    }
    if (contentRule) {
      const content = readFileSync(absolutePath, 'utf-8');
      if (!contentRule.test(content)) {
        failures.push(`${relativePath}: ${message}`);
      }
    }
  }

  if (failures.length === 0) {
    console.log('Replay verification passed');
    return 0;
  }

  for (const failure of failures) {
    console.log(`WARN ${failure}`);
  }

  return mode === 'strict' ? 1 : 0;
}

process.exit(main());
