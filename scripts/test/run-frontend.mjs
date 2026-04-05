import { spawnSync } from 'node:child_process';
import process from 'node:process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '../..');
const webDir = path.join(workspaceRoot, 'apps', 'client', 'web', 'app');

const GREEN = '\x1b[0;32m';
const RED = '\x1b[0;31m';
const YELLOW = '\x1b[1;33m';
const BLUE = '\x1b[0;34m';
const NC = '\x1b[0m';

const log = (...args) => console.log(`${BLUE}[frontend]${NC}`, ...args);
const ok = (...args) => console.log(`${GREEN}[✓]${NC}`, ...args);
const fail = (...args) => console.log(`${RED}[✗]${NC}`, ...args);

function runBunScript(script, args = [], cwd = webDir) {
  const result = spawnSync('bun', ['run', '--cwd', cwd, script, ...args], {
    stdio: 'inherit',
    shell: process.platform === 'win32',
    cwd,
  });
  return result.status ?? 1;
}

function runCheck() {
  log('Running svelte-check...');
  const exitCode = runBunScript('check');
  if (exitCode === 0) ok('Type check passed');
  return exitCode;
}

function runLint() {
  log('Running biome lint...');
  const exitCode = runBunScript('lint');
  if (exitCode === 0) ok('Lint passed');
  return exitCode;
}

function runUnit() {
  log('Running vitest unit tests...');
  const exitCode = runBunScript('test:unit');
  if (exitCode === 0) ok('Unit tests passed');
  return exitCode;
}

function runE2E(project) {
  log('Running Playwright E2E tests...');
  const args = project ? ['test:e2e', '--project', project] : ['test:e2e'];
  const exitCode = runBunScript(args[0], args.slice(1));
  if (exitCode === 0) ok('E2E tests passed');
  return exitCode;
}

function runAll() {
  let failures = 0;

  if (runCheck() !== 0) failures++;
  if (runLint() !== 0) failures++;
  if (runUnit() !== 0) failures++;
  if (runE2E() !== 0) failures++;

  console.log('');
  log('═══════════════════════════════════════');
  if (failures === 0) {
    ok('All frontend checks passed');
  } else {
    fail(`${failures} check(s) had issues`);
  }
  log('═══════════════════════════════════════');

  return failures;
}

const commands = {
  check: runCheck,
  lint: runLint,
  unit: runUnit,
  e2e: () => runE2E(process.argv[3]),
  all: runAll,
};

const cmd = process.argv[2] || 'all';

if (!commands[cmd]) {
  console.log('Usage: bun run scripts/test/run-frontend.mjs {check|lint|unit|e2e|all}');
  process.exit(1);
}

const exitCode = commands[cmd]();
process.exit(typeof exitCode === 'number' ? exitCode : 0);
