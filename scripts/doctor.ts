import { hasTool } from './lib/spawn.ts';
import { existsSync } from 'node:fs';
import process from 'node:process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '..');

async function main(): Promise<number> {
  console.log('=== Toolchain Check ===\n');

  const tools = [
    { name: 'bun', cmd: 'bun' },
    { name: 'node', cmd: 'node' },
    { name: 'cargo', cmd: 'cargo' },
    { name: 'rustc', cmd: 'rustc' },
    { name: 'moon', cmd: 'moon' },
  ];

  for (const tool of tools) {
    const available = await hasTool(tool.cmd);
    if (available) {
      const { run } = await import('./lib/spawn.ts');
      const result = await run(tool.cmd, ['--version']);
      console.log(`✓ ${tool.name}: ${result.output}`);
    } else {
      console.log(`✗ MISSING: ${tool.name}`);
    }
  }

  console.log('\n=== Config Files Check ===\n');

  const configs = ['.env', '.env.example', '.tool-versions', 'rust-toolchain.toml'];
  for (const config of configs) {
    const fullPath = path.join(workspaceRoot, config);
    if (existsSync(fullPath)) {
      console.log(`✓ ${config}: exists`);
    } else {
      console.log(`✗ MISSING: ${config}`);
    }
  }

  console.log('\n=== Done ===');
  return 0;
}

main()
  .then((code) => process.exit(code))
  .catch((err) => {
    console.error('Fatal error:', err);
    process.exit(1);
  });
