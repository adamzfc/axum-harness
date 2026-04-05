import { spawnSync } from 'node:child_process';
import process from 'node:process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { existsSync, mkdirSync, cpSync, rmSync } from 'node:fs';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '..');

const contractDirs = [
  { src: 'packages/contracts/api/bindings/api', dest: 'packages/contracts/generated/api' },
  { src: 'packages/contracts/auth/bindings/auth', dest: 'packages/contracts/generated/auth' },
  { src: 'packages/contracts/events/bindings/events', dest: 'packages/contracts/generated/events' },
];

const frontendDest = 'apps/client/web/app/src/lib/generated';

function runCommand(cmd, args) {
  const result = spawnSync(cmd, args, {
    stdio: 'pipe',
    encoding: 'utf8',
    shell: process.platform === 'win32',
    cwd: workspaceRoot,
  });
  return {
    success: result.status === 0,
    output: result.stdout?.trim() || '',
    error: result.stderr?.trim() || '',
  };
}

function safeCopy(src, dest) {
  if (!existsSync(src)) return;
  mkdirSync(dest, { recursive: true });
  try {
    cpSync(src, dest, { recursive: true });
  } catch {
    console.warn(`Warning: Could not copy ${src} to ${dest}`);
  }
}

function listDirectory(dir) {
  if (!existsSync(dir)) {
    console.log(`  (directory does not exist: ${dir})`);
    return;
  }
  const result = runCommand(process.platform === 'win32' ? 'dir' : 'ls', [
    process.platform === 'win32' ? dir : '-la',
  ]);
  console.log(result.output || result.error);
}

async function main() {
  console.log('=== Running typegen ===');

  runCommand('cargo', [
    'test',
    '-p', 'contracts_api',
    '-p', 'contracts_auth',
    '-p', 'contracts_events',
  ]);

  console.log('=== Cleaning old generated files ===');

  for (const { dest } of contractDirs) {
    const fullPath = path.join(workspaceRoot, dest);
    if (existsSync(fullPath)) {
      rmSync(fullPath, { recursive: true, force: true });
    }
  }

  console.log('=== Copying generated types ===');

  for (const { src, dest } of contractDirs) {
    const fullSrc = path.join(workspaceRoot, src);
    const fullDest = path.join(workspaceRoot, dest);
    safeCopy(fullSrc, fullDest);
  }

  console.log('=== Syncing to frontend ===');

  const fullFrontendDest = path.join(workspaceRoot, frontendDest);
  mkdirSync(fullFrontendDest, { recursive: true });

  for (const { dest } of contractDirs) {
    const fullSrc = path.join(workspaceRoot, dest);
    safeCopy(fullSrc, fullFrontendDest);
  }

  console.log('=== Typegen complete ===');

  listDirectory(fullFrontendDest);
}

main().catch((err) => {
  console.error('Fatal error:', err);
  process.exit(1);
});
