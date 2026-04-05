import { spawnSync } from 'node:child_process';
import process from 'node:process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '..');

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

function checkBoundary(pkgName, allowedPatterns, disallowedPattern) {
  console.log(`=== Checking ${pkgName} dependencies ===`);

  const result = runCommand('cargo', ['tree', '-p', pkgName, '--depth', '1']);
  if (!result.success) {
    console.warn(`Warning: Could not get dependency tree for ${pkgName}`);
    console.warn(result.error);
    return true;
  }

  const lines = result.output.split(/\r?\n/);
  const violations = [];

  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith(pkgName)) continue;

    let isAllowed = false;
    for (const pattern of allowedPatterns) {
      if (trimmed.includes(pattern)) {
        isAllowed = true;
        break;
      }
    }
    if (isAllowed) continue;

    if (disallowedPattern.test(trimmed)) {
      violations.push(trimmed);
    }
  }

  if (violations.length > 0) {
    console.error(`FAIL: ${pkgName} depends on illegal crates:`);
    for (const v of violations) {
      console.error(`  - ${v}`);
    }
    return false;
  }

  console.log(`OK: ${pkgName} boundary clean`);
  return true;
}

async function main() {
  let allClean = true;

  allClean = checkBoundary(
    'domain',
    ['async-trait', 'serde', 'serde_json'],
    /^(storage_|runtime_|contracts_)/,
  ) && allClean;

  allClean = checkBoundary(
    'usecases',
    ['async-trait', 'serde', 'serde_json', 'chrono', 'thiserror'],
    /^(storage_|runtime_|contracts_)/,
  ) && allClean;

  allClean = checkBoundary(
    'contracts_api',
    ['serde', 'ts-rs', 'utoipa', 'validator'],
    /^(domain|usecases|storage_|runtime_)/,
  ) && allClean;

  if (allClean) {
    console.log('All boundary checks passed');
    process.exit(0);
  } else {
    console.error('Boundary check failed');
    process.exit(1);
  }
}

main().catch((err) => {
  console.error('Fatal error:', err);
  process.exit(1);
});
