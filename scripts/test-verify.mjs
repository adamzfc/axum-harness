import { spawnSync } from 'node:child_process';
import process from 'node:process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '..');
const webDir = path.join(workspaceRoot, 'apps', 'client', 'web', 'app');

const GREEN = '\x1b[0;32m';
const RED = '\x1b[0;31m';
const YELLOW = '\x1b[1;33m';
const BLUE = '\x1b[0;34m';
const NC = '\x1b[0m';

let totalChecks = 0;
let passedChecks = 0;

function runCheck(name, cmd, args, cwd = workspaceRoot) {
  totalChecks++;
  console.log(`${YELLOW}[${totalChecks}] ${name}${NC}`);
  console.log(`  Command: ${cmd} ${args.join(' ')}`);

  const result = spawnSync(cmd, args, {
    stdio: 'pipe',
    shell: process.platform === 'win32',
    cwd,
  });

  if (result.status === 0) {
    console.log(`  ${GREEN}✅ PASSED${NC}`);
    passedChecks++;
  } else {
    console.log(`  ${RED}❌ FAILED${NC}`);
    if (result.stderr) {
      const lines = result.stderr.toString().split('\n').slice(-5);
      for (const line of lines) {
        if (line.trim()) console.log(`    ${line.trim()}`);
      }
    }
  }
  console.log('');
  return result.status === 0;
}

async function main() {
  console.log(`${BLUE}========================================${NC}`);
  console.log(`${BLUE}  Quality Gate Verification${NC}`);
  console.log(`${BLUE}========================================${NC}`);
  console.log('');

  console.log(`${BLUE}=== Phase 1: Rust Code Quality ===${NC}`);
  console.log('');

  runCheck('Rust Format Check', 'cargo', ['fmt', '--all', '--', '--check']);
  runCheck('Rust Clippy Lint', 'cargo', ['clippy', '--workspace', '--all-targets', '--', '-D', 'warnings']);
  runCheck('Rust Build', 'cargo', ['build', '--workspace', '--release']);

  console.log(`${BLUE}=== Phase 2: Tests ===${NC}`);
  console.log('');

  runCheck('Rust Unit Tests', 'cargo', ['test', '--workspace', '--all-features']);
  runCheck('Rust Doc Tests', 'cargo', ['test', '--workspace', '--doc']);

  console.log(`${BLUE}=== Phase 3: Frontend ===${NC}`);
  console.log('');

  runCheck('Frontend Type Check', 'bun', ['run', 'check'], webDir);
  runCheck('Frontend Lint', 'bun', ['run', 'lint'], webDir);
  runCheck('Frontend Unit Tests', 'bun', ['run', 'test:unit'], webDir);

  console.log(`${BLUE}========================================${NC}`);
  console.log(`${BLUE}  Summary${NC}`);
  console.log(`${BLUE}========================================${NC}`);
  console.log('');
  console.log(`Total Checks: ${totalChecks}`);
  console.log(`Passed: ${GREEN}${passedChecks}${NC}`);
  console.log(`Failed: ${RED}${totalChecks - passedChecks}${NC}`);
  console.log('');

  if (passedChecks === totalChecks) {
    console.log(`${GREEN}✅ All quality gates passed!${NC}`);
    process.exit(0);
  } else {
    console.log(`${RED}❌ Some quality gates failed!${NC}`);
    process.exit(1);
  }
}

main().catch((err) => {
  console.error('Fatal error:', err);
  process.exit(1);
});
