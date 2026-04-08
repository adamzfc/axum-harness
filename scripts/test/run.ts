import { spawn } from 'node:child_process';
import process from 'node:process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '../..');

const GREEN = '\x1b[0;32m';
const RED = '\x1b[0;31m';
const YELLOW = '\x1b[1;33m';
const BLUE = '\x1b[0;34m';
const NC = '\x1b[0m';

const log = (...args: string[]) => console.log(`${BLUE}[test]${NC}`, ...args);
const ok = (...args: string[]) => console.log(`${GREEN}[✓]${NC}`, ...args);
const fail = (...args: string[]) => console.log(`${RED}[✗]${NC}`, ...args);
const warn = (...args: string[]) => console.log(`${YELLOW}[!]${NC}`, ...args);

/**
 * Run a command asynchronously
 */
function runCommand(cmd: string, args: string[], options = {}): Promise<number> {
  return new Promise((resolve) => {
    const child = spawn(cmd, args, {
      stdio: 'inherit',
      shell: process.platform === 'win32',
      cwd: workspaceRoot,
      ...options,
    });

    child.on('close', (code) => {
      resolve(code ?? 1);
    });

    child.on('error', () => {
      resolve(1);
    });
  });
}

/**
 * Check if a tool is available
 */
async function checkTool(name: string, installCmd: string): Promise<boolean> {
  const checkCmd = process.platform === 'win32' ? 'where' : 'command';
  const checkArgs = process.platform === 'win32' ? [name] : ['-v', name];

  return new Promise((resolve) => {
    const child = spawn(checkCmd, checkArgs, {
      stdio: 'pipe',
      shell: process.platform === 'win32',
    });

    child.on('close', (code) => {
      if (code === 0) {
        resolve(true);
      } else {
        warn(`${name} not found — install with: ${installCmd}`);
        resolve(false);
      }
    });

    child.on('error', () => {
      warn(`${name} not found — install with: ${installCmd}`);
      resolve(false);
    });
  });
}

async function runNextest(extraArgs: string[] = []): Promise<number> {
  log('Running cargo-nextest...');
  if (!(await checkTool('cargo-nextest', 'cargo install cargo-nextest --locked'))) return 1;

  const profile = process.env.PROFILE || 'default';
  log(`Profile: ${profile}`);

  const exitCode = await runCommand('cargo', ['nextest', 'run', '--workspace', '--profile', profile, ...extraArgs]);
  if (exitCode === 0) {
    ok('All nextest tests passed');
  } else {
    fail(`nextest tests failed (exit code: ${exitCode})`);
  }
  return exitCode;
}

async function runCoverage(extraArgs: string[] = []): Promise<number> {
  log('Running cargo-llvm-cov...');
  if (!(await checkTool('cargo-llvm-cov', 'cargo install cargo-llvm-cov --locked'))) return 1;

  const outputFormat = process.env.COV_FORMAT || 'lcov';
  const outputPath = 'target/lcov.info';

  log(`Format: ${outputFormat} → ${outputPath}`);

  const exitCode = await runCommand('cargo', [
    'llvm-cov', '--workspace',
    `--${outputFormat}`,
    '--output-path', outputPath,
    '--ignore-filename-regex', 'tests/',
    ...extraArgs,
  ]);

  if (exitCode === 0) {
    ok(`Coverage report generated: ${outputPath}`);
    await runCommand('cargo', ['llvm-cov', '--workspace', '--summary-only'], { stdio: 'pipe' });
  } else {
    fail(`Coverage run failed (exit code: ${exitCode})`);
  }
  return exitCode;
}

async function runHack(extraArgs: string[] = []): Promise<number> {
  log('Running cargo-hack feature powerset...');
  if (!(await checkTool('cargo-hack', 'cargo install cargo-hack --locked'))) return 1;

  const exitCode = await runCommand('cargo', ['hack', 'check', '--workspace', '--feature-powerset', ...extraArgs]);
  if (exitCode === 0) {
    ok('All feature combinations compile');
  } else {
    fail(`Some feature combinations failed (exit code: ${exitCode})`);
  }
  return exitCode;
}

async function runMutants(extraArgs: string[] = []): Promise<number> {
  log('Running cargo-mutants...');
  if (!(await checkTool('cargo-mutants', 'cargo install cargo-mutants --locked'))) return 1;

  const exitCode = await runCommand('cargo', ['mutants', '--workspace', ...extraArgs]);
  if (exitCode === 0) {
    ok('All mutants caught by tests');
  } else {
    warn('Some mutants survive — tests may need strengthening');
  }
  return 0;
}

async function runQuick(): Promise<number> {
  log('Running quick smoke test...');

  let exitCode = await runCommand('cargo', ['check', '--workspace', '--quiet']);
  if (exitCode === 0) {
    ok('cargo check');
  } else {
    fail('cargo check');
    return 1;
  }

  exitCode = await runCommand('cargo', ['test', '--workspace', '--lib', '--quiet']);
  if (exitCode === 0) {
    ok('cargo test --lib');
  } else {
    fail('cargo test --lib');
    return 1;
  }

  ok('Quick smoke test passed');
  return 0;
}

async function runAll(): Promise<number> {
  let failures = 0;

  if (await runQuick() !== 0) failures++;
  if (await runNextest() !== 0) failures++;
  if (await runCoverage() !== 0) failures++;
  if (await runHack() !== 0) failures++;
  await runMutants();

  console.log('');
  log('═══════════════════════════════════════');
  if (failures === 0) {
    ok('All test suites passed');
  } else {
    fail(`${failures} suite(s) had issues`);
  }
  log('═══════════════════════════════════════');

  return failures;
}

const commands: Record<string, (args: string[]) => Promise<number>> = {
  nextest: runNextest,
  coverage: runCoverage,
  hack: runHack,
  mutants: runMutants,
  quick: runQuick,
  all: runAll,
};

const cmd = process.argv[2] || 'nextest';
const extraArgs = process.argv.slice(3);

if (!commands[cmd]) {
  console.log(`Usage: bun run scripts/test/run.ts {nextest|coverage|hack|mutants|quick|all}`);
  console.log('');
  console.log('Commands:');
  console.log('  nextest    Run tests with cargo-nextest (default)');
  console.log('  coverage   Run tests with cargo-llvm-cov');
  console.log('  hack       Run cargo-hack feature powerset check');
  console.log('  mutants    Run cargo-mutants mutation testing');
  console.log('  quick      Quick smoke test (unit only)');
  console.log('  all        Run all test suites');
  process.exit(1);
}

commands[cmd](extraArgs)
  .then((code) => process.exit(code))
  .catch((err) => {
    console.error('Fatal error:', err);
    process.exit(1);
  });
