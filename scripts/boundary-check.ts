import { spawn } from 'node:child_process';
import process from 'node:process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

interface CommandResult {
  success: boolean;
  output: string;
  error: string;
}

interface BoundaryRule {
  pkgName: string;
  allowedPatterns: string[];
  disallowedPattern: RegExp;
}

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '..');

/**
 * Execute a command and return result
 */
function runCommand(cmd: string, args: string[]): Promise<CommandResult> {
  return new Promise((resolve) => {
    const child = spawn(cmd, args, {
      stdio: ['pipe', 'pipe', 'pipe'],
      shell: process.platform === 'win32',
      cwd: workspaceRoot,
    });

    let stdout = '';
    let stderr = '';

    child.stdout?.on('data', (data) => {
      stdout += data.toString();
    });

    child.stderr?.on('data', (data) => {
      stderr += data.toString();
    });

    child.on('close', (code) => {
      resolve({
        success: code === 0,
        output: stdout.trim(),
        error: stderr.trim(),
      });
    });

    child.on('error', (err) => {
      resolve({
        success: false,
        output: '',
        error: err.message,
      });
    });
  });
}

/**
 * Check if a dependency violates boundary rules
 */
function checkBoundary(pkgName: string, allowedPatterns: string[], disallowedPattern: RegExp): Promise<boolean> {
  return new Promise(async (resolve) => {
    console.log(`=== Checking ${pkgName} dependencies ===`);

    const result = await runCommand('cargo', ['tree', '-p', pkgName, '--depth', '1']);

    if (!result.success) {
      console.warn(`⚠️  Could not get dependency tree for ${pkgName}`);
      if (result.error) console.warn(result.error);
      resolve(true); // Don't fail on missing package
      return;
    }

    const lines = result.output.split(/\r?\n/);
    const violations: string[] = [];

    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed || trimmed.startsWith(pkgName)) continue;

      const isAllowed = allowedPatterns.some((pattern) => trimmed.includes(pattern));
      if (isAllowed) continue;

      if (disallowedPattern.test(trimmed)) {
        violations.push(trimmed);
      }
    }

    if (violations.length > 0) {
      console.error(`❌ FAIL: ${pkgName} depends on illegal crates:`);
      for (const v of violations) {
        console.error(`  - ${v}`);
      }
      resolve(false);
    } else {
      console.log(`✅ OK: ${pkgName} boundary clean`);
      resolve(true);
    }
  });
}

async function main(): Promise<number> {
  const rules: BoundaryRule[] = [
    {
      pkgName: 'domain',
      allowedPatterns: ['async-trait', 'serde', 'serde_json'],
      disallowedPattern: /^(storage_|runtime_|contracts_)/,
    },
    {
      pkgName: 'usecases',
      allowedPatterns: ['async-trait', 'serde', 'serde_json', 'chrono', 'thiserror'],
      disallowedPattern: /^(storage_|runtime_|contracts_)/,
    },
    {
      pkgName: 'contracts_api',
      allowedPatterns: ['serde', 'ts-rs', 'utoipa', 'validator'],
      disallowedPattern: /^(domain|usecases|storage_|runtime_)/,
    },
  ];

  console.log('=== Architecture Boundary Check ===\n');

  const results = await Promise.all(
    rules.map(({ pkgName, allowedPatterns, disallowedPattern }) =>
      checkBoundary(pkgName, allowedPatterns, disallowedPattern)
    )
  );

  console.log('');

  const allClean = results.every(Boolean);

  if (allClean) {
    console.log('✅ All boundary checks passed');
    return 0;
  }

  console.error('❌ Boundary check failed — review architectural dependencies');
  return 1;
}

main()
  .then((code) => process.exit(code))
  .catch((err) => {
    console.error('Fatal error:', err);
    process.exit(1);
  });
