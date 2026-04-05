import { spawnSync } from 'node:child_process';
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

const log = (...args) => console.log(`${BLUE}[test]${NC}`, ...args);
const ok = (...args) => console.log(`${GREEN}[✓]${NC}`, ...args);
const fail = (...args) => console.log(`${RED}[✗]${NC}`, ...args);
const warn = (...args) => console.log(`${YELLOW}[!]${NC}`, ...args);

function runCommand(cmd, args, options = {}) {
  const result = spawnSync(cmd, args, {
    stdio: 'inherit',
    shell: process.platform === 'win32',
    cwd: workspaceRoot,
    ...options,
  });
  return result.status ?? 1;
}

function checkTool(name, installCmd) {
  const result = spawnSync(
    process.platform === 'win32' ? 'where' : 'command',
    process.platform === 'win32' ? [name] : ['-v', name],
    { stdio: 'pipe', shell: process.platform === 'win32' },
  );
  const found = process.platform === 'win32'
    ? result.status === 0
    : result.status === 0;

  if (!found) {
    warn(`${name} not found — install with: ${installCmd}`);
    return false;
  }
  return true;
}

function runNextest(extraArgs = []) {
  log('Running cargo-nextest...');
  if (!checkTool('cargo-nextest', 'cargo install cargo-nextest --locked')) return 1;

  const profile = process.env.PROFILE || 'default';
  log(`Profile: ${profile}`);

  const exitCode = runCommand('cargo', ['nextest', 'run', '--workspace', '--profile', profile, ...extraArgs]);
  if (exitCode === 0) {
    ok('All nextest tests passed');
  } else {
    fail(`nextest tests failed (exit code: ${exitCode})`);
  }
  return exitCode;
}

function runCoverage(extraArgs = []) {
  log('Running cargo-llvm-cov...');
  if (!checkTool('cargo-llvm-cov', 'cargo install cargo-llvm-cov --locked')) return 1;

  const outputFormat = process.env.COV_FORMAT || 'lcov';
  const outputPath = 'target/lcov.info';

  log(`Format: ${outputFormat} → ${outputPath}`);

  const exitCode = runCommand('cargo', [
    'llvm-cov', '--workspace',
    `--${outputFormat}`,
    '--output-path', outputPath,
    '--ignore-filename-regex', 'tests/',
    ...extraArgs,
  ]);

  if (exitCode === 0) {
    ok(`Coverage report generated: ${outputPath}`);
    runCommand('cargo', ['llvm-cov', '--workspace', '--summary-only'], { stdio: 'pipe' });
  } else {
    fail(`Coverage run failed (exit code: ${exitCode})`);
  }
  return exitCode;
}

function runHack(extraArgs = []) {
  log('Running cargo-hack feature powerset...');
  if (!checkTool('cargo-hack', 'cargo install cargo-hack --locked')) return 1;

  const exitCode = runCommand('cargo', ['hack', 'check', '--workspace', '--feature-powerset', ...extraArgs]);
  if (exitCode === 0) {
    ok('All feature combinations compile');
  } else {
    fail(`Some feature combinations failed (exit code: ${exitCode})`);
  }
  return exitCode;
}

function runMutants(extraArgs = []) {
  log('Running cargo-mutants...');
  if (!checkTool('cargo-mutants', 'cargo install cargo-mutants --locked')) return 1;

  const exitCode = runCommand('cargo', ['mutants', '--workspace', ...extraArgs]);
  if (exitCode === 0) {
    ok('All mutants caught by tests');
  } else {
    warn('Some mutants survived — tests may need strengthening');
  }
  return 0;
}

function runQuick() {
  log('Running quick smoke test...');

  let exitCode = runCommand('cargo', ['check', '--workspace', '--quiet']);
  if (exitCode === 0) {
    ok('cargo check');
  } else {
    fail('cargo check');
    return 1;
  }

  exitCode = runCommand('cargo', ['test', '--workspace', '--lib', '--quiet']);
  if (exitCode === 0) {
    ok('cargo test --lib');
  } else {
    fail('cargo test --lib');
    return 1;
  }

  ok('Quick smoke test passed');
  return 0;
}

function runAll() {
  let failures = 0;

  if (runQuick() !== 0) failures++;
  if (runNextest() !== 0) failures++;
  if (runCoverage() !== 0) failures++;
  if (runHack() !== 0) failures++;
  runMutants();

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

const commands = {
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
  console.log(`Usage: bun run scripts/test/run.mjs {nextest|coverage|hack|mutants|quick|all}`);
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

const exitCode = commands[cmd](extraArgs);
process.exit(typeof exitCode === 'number' ? exitCode : 0);
