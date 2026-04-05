import { existsSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import process from 'node:process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '..');

function runCommand(cmd, args, options = {}) {
  const result = spawnSync(cmd, args, {
    stdio: 'pipe',
    encoding: 'utf8',
    shell: process.platform === 'win32',
    ...options,
  });
  return {
    success: result.status === 0,
    output: result.stdout?.trim() || '',
    error: result.stderr?.trim() || '',
  };
}

function checkVersion(name, cmd, args = ['--version']) {
  const result = runCommand(cmd, args);
  if (result.success) {
    console.log(`✅ ${name}: ${result.output}`);
    return true;
  }
  console.log(`❌ MISSING: ${name} — run: just setup`);
  return false;
}

async function main() {
  console.log('=== Toolchain Check ===');

  checkVersion('bun', 'bun');
  checkVersion('node', 'node');
  checkVersion('cargo', 'cargo');
  checkVersion('rustc', 'rustc');
  checkVersion('moon', 'moon');

  console.log('=== Env Check ===');

  const envPath = path.join(workspaceRoot, '.env');
  if (existsSync(envPath)) {
    console.log('.env exists');
  } else {
    console.log('MISSING: .env');
  }

  console.log('=== Done ===');
}

main().catch((err) => {
  console.error('Fatal error:', err);
  process.exit(1);
});
