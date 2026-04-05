import { spawn, spawnSync } from 'node:child_process';
import process from 'node:process';
import net from 'node:net';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { existsSync } from 'node:fs';

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '..');
const tauriDir = path.join(workspaceRoot, 'apps', 'client', 'native', 'src-tauri');

const API_PORT = 3001;
const MAX_WAIT_SECONDS = 30;

function waitForPort(port, maxSeconds) {
  return new Promise((resolve) => {
    const startTime = Date.now();
    const interval = setInterval(() => {
      const elapsed = (Date.now() - startTime) / 1000;
      if (elapsed >= maxSeconds) {
        clearInterval(interval);
        resolve(false);
        return;
      }
      const socket = net.createConnection({ port, host: 'localhost' }, () => {
        clearInterval(interval);
        socket.end();
        resolve(true);
      });
      socket.on('error', () => {
        socket.destroy();
      });
    }, 1000);
  });
}

function cleanup(apiProcess, tauriProcess) {
  console.log('\n[dev-desktop] Cleaning up processes...');
  if (apiProcess) {
    console.log('[dev-desktop] Stopping API server...');
    if (process.platform === 'win32') {
      spawnSync('taskkill', ['/PID', String(apiProcess.pid), '/F', '/T'], {
        stdio: 'inherit',
      });
    } else {
      apiProcess.kill('SIGTERM');
    }
  }
  if (tauriProcess) {
    console.log('[dev-desktop] Stopping Tauri dev...');
    if (process.platform === 'win32') {
      spawnSync('taskkill', ['/PID', String(tauriProcess.pid), '/F', '/T'], {
        stdio: 'inherit',
      });
    } else {
      tauriProcess.kill('SIGTERM');
    }
  }
}

async function main() {
  let apiProcess = null;
  let tauriProcess = null;

  const cleanupHandler = () => cleanup(apiProcess, tauriProcess);
  process.on('SIGINT', cleanupHandler);
  process.on('SIGTERM', cleanupHandler);
  if (process.platform === 'win32') {
    process.on('SIGBREAK', cleanupHandler);
  }
  process.on('exit', cleanupHandler);

  // Start Axum API server
  console.log('[dev-desktop] Starting Axum API server...');
  apiProcess = spawn('cargo', ['run', '-p', 'runtime_server'], {
    cwd: workspaceRoot,
    stdio: 'inherit',
    shell: process.platform === 'win32',
  });

  apiProcess.on('error', (err) => {
    console.error('[dev-desktop] Failed to start API server:', err.message);
    process.exit(1);
  });

  // Wait for API server to be ready
  console.log(`[dev-desktop] Waiting for API server on port ${API_PORT}...`);
  const isReady = await waitForPort(API_PORT, MAX_WAIT_SECONDS);
  if (isReady) {
    console.log(`[dev-desktop] API server ready on port ${API_PORT}`);
  } else {
    console.warn(`[dev-desktop] API server did not become ready within ${MAX_WAIT_SECONDS}s. Continuing anyway...`);
  }

  // Start Tauri dev (blocking)
  console.log('[dev-desktop] Starting Tauri dev...');
  tauriProcess = spawn(
    process.platform === 'win32' ? 'cargo.cmd' : 'cargo',
    ['tauri', 'dev'],
    {
      cwd: tauriDir,
      stdio: 'inherit',
      shell: process.platform === 'win32',
    },
  );

  tauriProcess.on('error', (err) => {
    console.error('[dev-desktop] Failed to start Tauri dev:', err.message);
    process.exit(1);
  });

  const [code, signal] = await new Promise((resolve) => {
    tauriProcess.on('exit', (code, signal) => resolve([code, signal]));
  });

  console.log(`[dev-desktop] Tauri dev exited with code ${code} (signal: ${signal})`);
  process.exit(code ?? 0);
}

main().catch((err) => {
  console.error('[dev-desktop] Fatal error:', err);
  process.exit(1);
});
