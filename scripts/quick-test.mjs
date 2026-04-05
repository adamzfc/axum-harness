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
const NC = '\x1b[0m';

function runSilent(cmd, args, cwd = workspaceRoot) {
  const result = spawnSync(cmd, args, {
    stdio: 'pipe',
    shell: process.platform === 'win32',
    cwd,
  });
  return result.status === 0;
}

function runVisible(cmd, args, cwd = workspaceRoot) {
  const result = spawnSync(cmd, args, {
    stdio: 'inherit',
    shell: process.platform === 'win32',
    cwd,
  });
  return result.status === 0;
}

async function main() {
  console.log(`${YELLOW}🚀 Quick Test Runner${NC}`);
  console.log('');
  console.log('Running fast checks...');
  console.log('');

  const checks = [
    {
      name: 'Format check',
      run: () => runSilent('cargo', ['fmt', '--all', '--', '--check']),
    },
    {
      name: 'Clippy',
      run: () => runSilent('cargo', ['clippy', '--workspace', '--all-targets', '--', '-D', 'warnings']),
    },
    {
      name: 'Build',
      run: () => runSilent('cargo', ['build', '--workspace']),
    },
    {
      name: 'Tests',
      run: () => runSilent('cargo', ['test', '--workspace']),
    },
    {
      name: 'Frontend check',
      run: () => runSilent('bun', ['run', 'check'], webDir),
    },
  ];

  for (const check of checks) {
    process.stdout.write(`${check.name}: `);
    const passed = check.run();
    console.log(passed ? `${GREEN}✅${NC}` : `${RED}❌${NC}`);
  }

  console.log('');
  console.log('Done!');
}

main().catch((err) => {
  console.error('Fatal error:', err);
  process.exit(1);
});
