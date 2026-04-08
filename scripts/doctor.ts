import { existsSync } from 'node:fs';
import { spawn } from 'node:child_process';
import process from 'node:process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

interface CommandResult {
  success: boolean;
  output: string;
  error: string;
}

interface ToolCheck {
  name: string;
  cmd: string;
  args?: string[];
}

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '..');

/**
 * Execute a command asynchronously and return result
 */
function runCommand(cmd: string, args: string[] = [], options = {}): Promise<CommandResult> {
  return new Promise((resolve) => {
    const child = spawn(cmd, args, {
      stdio: ['pipe', 'pipe', 'pipe'],
      shell: process.platform === 'win32',
      ...options,
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
 * Check if a tool exists and get its version
 */
async function checkVersion({ name, cmd, args = ['--version'] }: ToolCheck): Promise<boolean> {
  try {
    const result = await runCommand(cmd, args);
    if (result.success) {
      console.log(`✅ ${name}: ${result.output}`);
      return true;
    }
    console.log(`❌ MISSING: ${name} — run: just setup`);
    return false;
  } catch {
    console.log(`❌ MISSING: ${name} — run: just setup`);
    return false;
  }
}

/**
 * Check if a file exists
 */
function checkFileExists(filePath: string, label: string): boolean {
  if (existsSync(filePath)) {
    console.log(`✅ ${label}: exists`);
    return true;
  }
  console.log(`❌ MISSING: ${label}`);
  return false;
}

async function main(): Promise<number> {
  console.log('=== Toolchain Check ===\n');

  const tools: ToolCheck[] = [
    { name: 'bun', cmd: 'bun' },
    { name: 'node', cmd: 'node' },
    { name: 'cargo', cmd: 'cargo' },
    { name: 'rustc', cmd: 'rustc' },
    { name: 'moon', cmd: 'moon' },
  ];

  const results = await Promise.all(tools.map(checkVersion));

  console.log('\n=== Config Files Check ===\n');

  const configFiles = [
    { path: path.join(workspaceRoot, '.env'), label: '.env' },
    { path: path.join(workspaceRoot, '.env.example'), label: '.env.example' },
    { path: path.join(workspaceRoot, '.tool-versions'), label: '.tool-versions' },
    { path: path.join(workspaceRoot, 'rust-toolchain.toml'), label: 'rust-toolchain.toml' },
  ];

  const fileResults = configFiles.map(({ path: p, label }) => checkFileExists(p, label));

  console.log('\n=== Summary ===\n');

  const toolPass = results.filter(Boolean).length;
  const filePass = fileResults.filter(Boolean).length;

  console.log(`Tools: ${toolPass}/${tools.length} installed`);
  console.log(`Configs: ${filePass}/${configFiles.length} present`);

  const allPassed = results.every(Boolean) && fileResults.every(Boolean);

  if (allPassed) {
    console.log('\n✅ All checks passed');
    return 0;
  }

  console.log('\n⚠️  Some checks failed — run: just setup');
  return 1;
}

main()
  .then((code) => process.exit(code))
  .catch((err) => {
    console.error('Fatal error:', err);
    process.exit(1);
  });
