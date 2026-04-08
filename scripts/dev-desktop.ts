import { spawn, spawnSync } from 'node:child_process';
import process from 'node:process';
import net from 'node:net';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import type { ChildProcess } from 'node:child_process';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '..');
const tauriDir = path.join(workspaceRoot, 'apps', 'client', 'native', 'src-tauri');

const API_PORT = 3001;
const API_WAIT_SECONDS = 180; // First compile may take long (improves with sccache)

interface OptimizationStatus {
  sccache: boolean;
  hakari: boolean;
}

function printOptimizationTips(status: OptimizationStatus): void {
  if (status.sccache) {
    console.log('[dev-desktop] ✓ sccache enabled (compilation caching active)');
  } else {
    console.log('[dev-desktop] ⚠ sccache NOT enabled — run: just setup-sccache');
  }

  if (status.hakari) {
    console.log('[dev-desktop] ✓ cargo-hakari enabled (unified dependency resolution)');
  } else {
    console.log('[dev-desktop] ⚠ cargo-hakari NOT enabled — run: just setup-hakari');
  }
  console.log('');
}

/**
 * Wait for a port to become available
 */
function waitForPort(port: number, maxSeconds: number, name: string): Promise<boolean> {
  return new Promise((resolve) => {
    const startTime = Date.now();
    const interval = setInterval(() => {
      const elapsed = (Date.now() - startTime) / 1000;
      if (elapsed >= maxSeconds) {
        clearInterval(interval);
        console.warn(`[dev-desktop] ${name} did not become ready within ${maxSeconds}s`);
        resolve(false);
        return;
      }

      const socket = net.createConnection({ port, host: 'localhost' }, () => {
        clearInterval(interval);
        socket.end();
        console.log(`[dev-desktop] ${name} ready on port ${port}`);
        resolve(true);
      });

      socket.on('error', () => {
        socket.destroy();
      });
    }, 1000);
  });
}

/**
 * Kill a process and its children (cross-platform)
 */
function killProcessTree(pid: number | undefined): void {
  if (!pid) return;

  if (process.platform === 'win32') {
    spawnSync('taskkill', ['/PID', String(pid), '/F', '/T'], {
      stdio: 'inherit',
    });
  } else {
    try {
      process.kill(-pid, 'SIGTERM');
    } catch {
      process.kill(pid, 'SIGTERM');
    }
  }
}

/**
 * Cleanup all child processes
 */
function cleanup(apiProcess: ChildProcess | null, tauriProcess: ChildProcess | null): void {
  console.log('\n[dev-desktop] Cleaning up...');

  if (tauriProcess) {
    console.log('[dev-desktop] Stopping Tauri (this will also stop its child processes)...');
    killProcessTree(tauriProcess.pid);
  }

  if (apiProcess) {
    console.log('[dev-desktop] Stopping API server...');
    killProcessTree(apiProcess.pid);
  }
}

async function main(): Promise<void> {
  let apiProcess: ChildProcess | null = null;
  let tauriProcess: ChildProcess | null = null;

  // Setup cleanup handlers
  const cleanupHandler = () => cleanup(apiProcess, tauriProcess);
  process.on('SIGINT', cleanupHandler);
  process.on('SIGTERM', cleanupHandler);
  if (process.platform === 'win32') {
    process.on('SIGBREAK', cleanupHandler);
  }

  console.log('[dev-desktop] === Starting Desktop Dev ===');
  console.log(`[dev-desktop] Workspace: ${workspaceRoot}`);
  console.log(`[dev-desktop] Tauri dir: ${tauriDir}`);
  console.log('');

  // Print optimization tips
  const optStatus: OptimizationStatus = {
    sccache: process.env.RUSTC_WRAPPER === 'sccache',
    hakari: process.env.CARGO_HAKARI !== '0',
  };
  printOptimizationTips(optStatus);

  // Step 1: Start Axum API server
  console.log('[dev-desktop] Step 1/2: Starting Axum API server...');
  console.log('[dev-desktop] (First run may take a while to compile surrealdb-core)');

  apiProcess = spawn('cargo', ['run', '-p', 'runtime_server'], {
    cwd: workspaceRoot,
    stdio: ['ignore', 'inherit', 'inherit'],
    shell: process.platform === 'win32',
  });

  apiProcess.on('error', (err) => {
    console.error('[dev-desktop] Failed to start API server:', err.message);
    process.exit(1);
  });

  // Wait for API server (generous timeout for first compile)
  const apiReady = await waitForPort(API_PORT, API_WAIT_SECONDS, 'API server');
  if (!apiReady) {
    console.warn('[dev-desktop] WARNING: API server not ready yet, but starting Tauri anyway...');
    console.warn('[dev-desktop] API server will continue compiling in background...');
  }
  console.log('');

  // Step 2: Start Tauri (automatically starts SvelteKit frontend via beforeDevCommand)
  console.log('[dev-desktop] Step 2/2: Starting Tauri desktop app...');
  console.log('[dev-desktop] Tauri will start the SvelteKit frontend automatically...');
  console.log('');

  tauriProcess = spawn('cargo', ['tauri', 'dev'], {
    cwd: tauriDir,
    stdio: ['ignore', 'inherit', 'inherit'],
    shell: process.platform === 'win32',
    env: { ...process.env },
  });

  tauriProcess.on('error', (err) => {
    console.error('[dev-desktop] Failed to start Tauri:', err.message);
    process.exit(1);
  });

  tauriProcess.on('exit', (code, signal) => {
    console.log(`\n[dev-desktop] Tauri exited with code ${code} (signal: ${signal})`);
  });

  console.log('[dev-desktop] === Services Started ===');
  console.log('[dev-desktop] API server:  http://localhost:3001');
  console.log('[dev-desktop] Frontend:    http://localhost:5173 (managed by Tauri)');
  console.log('[dev-desktop] Tauri window: should appear after Rust compilation');
  console.log('');
  console.log('[dev-desktop] Press Ctrl+C to stop all services');
  console.log('');

  // Wait for Tauri to exit
  await new Promise<void>((resolve) => {
    tauriProcess?.on('exit', () => resolve());
  });

  console.log('[dev-desktop] Tauri exited, shutting down API server...');
  process.exit(0);
}

main().catch((err) => {
  console.error('[dev-desktop] Fatal error:', err);
  process.exit(1);
});
