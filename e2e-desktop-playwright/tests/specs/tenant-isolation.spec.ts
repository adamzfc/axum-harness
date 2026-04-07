import { test, expect, type APIRequestContext } from '@playwright/test';
import { TENANT_A, TENANT_B } from '../fixtures/tenant';
import { spawn, type ChildProcess } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const API_BASE_URL = 'http://127.0.0.1:3001';
const TENANT_INIT_URL = `${API_BASE_URL}/api/tenant/init`;
const COUNTER_RESET_URL = `${API_BASE_URL}/api/counter/reset`;
const COUNTER_INCREMENT_URL = `${API_BASE_URL}/api/counter/increment`;
const COUNTER_VALUE_URL = `${API_BASE_URL}/api/counter/value`;
const RETRY_LIMIT = 3;
const RETRY_DELAY_MS = 1200;
const API_READY_URL = `${API_BASE_URL}/readyz`;

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '..', '..', '..');

let ownedApiProcess: ChildProcess | null = null;

test.describe('Tauri Desktop Tenant Isolation', () => {
	test.describe.configure({ mode: 'serial' });
	const BASELINE = 0;
	const TENANT_A_WRITES = 2;

	test.beforeAll(async () => {
		await ensureApiReady();
	});

	test.afterAll(async () => {
		stopOwnedApiProcess();
	});

	test.beforeEach(async ({ request }) => {
		await withRetry('resetTenantPair', () => resetTenantPair(request, BASELINE));
	});

	test('uses the same fixed tenant pair as web harness', async () => {
		expect(TENANT_A.userSub).toBe('tenant_a_user');
		expect(TENANT_B.userSub).toBe('tenant_b_user');
	});

	test('tenant-1 write does not alter tenant-2 value (run-1)', async ({ request }) => {
		await assertIsolationFlow(request, 'run-1', BASELINE, TENANT_A_WRITES);
	});

	test('tenant-1 write does not alter tenant-2 value (run-2, same seed)', async ({ request }) => {
		await assertIsolationFlow(request, 'run-2', BASELINE, TENANT_A_WRITES);
	});
});

function sleep(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

async function withRetry<T>(operationName: string, operation: () => Promise<T>): Promise<T> {
	let lastError: unknown;
	for (let attempt = 1; attempt <= RETRY_LIMIT; attempt += 1) {
		try {
			return await operation();
		} catch (error) {
			lastError = error;
			if (attempt === RETRY_LIMIT) break;
			await sleep(RETRY_DELAY_MS);
		}
	}

	throw new Error(
		`[tenant-isolation] ${operationName} failed after ${RETRY_LIMIT} attempts: ${
			lastError instanceof Error ? lastError.message : String(lastError)
		}`
	);
}

async function waitForApiReady(timeoutMs = 120_000): Promise<boolean> {
	const start = Date.now();
	while (Date.now() - start < timeoutMs) {
		try {
			const response = await fetch(API_READY_URL);
			if (response.ok) {
				return true;
			}
		} catch {
			// keep polling
		}

		await sleep(500);
	}

	return false;
}

function runtimeServerBinaryPath(): string {
	const binary = process.platform === 'win32' ? 'runtime_server.exe' : 'runtime_server';
	return path.join(workspaceRoot, 'target', 'debug', binary);
}

async function ensureApiReady(): Promise<void> {
	if (await waitForApiReady(1_000)) {
		return;
	}

	const runtimeBinary = runtimeServerBinaryPath();
	if (existsSync(runtimeBinary)) {
		ownedApiProcess = spawn(runtimeBinary, [], {
			cwd: workspaceRoot,
			stdio: 'ignore',
			shell: false
		});
	} else {
		ownedApiProcess = spawn('cargo', ['run', '-p', 'runtime_server'], {
			cwd: workspaceRoot,
			stdio: 'ignore',
			shell: process.platform === 'win32'
		});
	}

	const ready = await waitForApiReady(120_000);
	if (!ready) {
		stopOwnedApiProcess();
		throw new Error('runtime_server did not become ready at /readyz within timeout');
	}
}

function stopOwnedApiProcess(): void {
	if (!ownedApiProcess) {
		return;
	}

	if (!ownedApiProcess.killed) {
		if (process.platform === 'win32') {
			spawn('taskkill', ['/PID', String(ownedApiProcess.pid), '/F', '/T'], {
				stdio: 'ignore',
				shell: false
			});
		} else {
			ownedApiProcess.kill('SIGTERM');
		}
	}

	ownedApiProcess = null;
}

function toBase64Url(input: string): string {
	return Buffer.from(input).toString('base64').replace(/=/g, '').replace(/\+/g, '-').replace(/\//g, '_');
}

function makeTenantToken(userSub: string): string {
	const header = toBase64Url(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
	const payload = toBase64Url(JSON.stringify({ sub: userSub, exp: 4_102_444_800 }));
	return `${header}.${payload}.desktop-e2e`;
}

function authHeaders(userSub: string): Record<string, string> {
	return {
		Authorization: `Bearer ${makeTenantToken(userSub)}`,
		'content-type': 'application/json'
	};
}

async function initTenantPair(request: APIRequestContext): Promise<void> {
	for (const tenant of [TENANT_A, TENANT_B]) {
		const response = await request.post(TENANT_INIT_URL, {
			headers: authHeaders(tenant.userSub),
			data: { user_sub: tenant.userSub, user_name: tenant.userName }
		});
		const body = (await response.json().catch(() => ({}))) as { tenant_id?: string };
		if (response.status() !== 200 || typeof body.tenant_id !== 'string' || body.tenant_id.length === 0) {
			throw new Error(
				`[${tenant.label}] tenant init failed: status=${response.status()}, body=${JSON.stringify(body)}`
			);
		}
	}
}

async function readTenantCounter(request: APIRequestContext, userSub: string): Promise<number> {
	const response = await request.get(COUNTER_VALUE_URL, {
		headers: authHeaders(userSub)
	});
	const body = (await response.json().catch(() => ({}))) as { value?: number };
	if (response.status() !== 200 || typeof body.value !== 'number') {
		throw new Error(`read counter failed for ${userSub}: status=${response.status()}, body=${JSON.stringify(body)}`);
	}
	return body.value;
}

async function incrementTenantCounter(request: APIRequestContext, userSub: string, times = 1): Promise<void> {
	for (let i = 0; i < times; i += 1) {
		const response = await request.post(COUNTER_INCREMENT_URL, {
			headers: authHeaders(userSub)
		});
		const body = (await response.json().catch(() => ({}))) as { value?: number };
		if (response.status() !== 200 || typeof body.value !== 'number') {
			throw new Error(
				`increment failed for ${userSub} at step ${i + 1}: status=${response.status()}, body=${JSON.stringify(body)}`
			);
		}
	}
}

async function resetTenantPair(request: APIRequestContext, seedValue = 0): Promise<void> {
	await initTenantPair(request);

	for (const tenant of [TENANT_A, TENANT_B]) {
		const resetResponse = await request.post(COUNTER_RESET_URL, {
			headers: authHeaders(tenant.userSub)
		});
		const resetBody = (await resetResponse.json().catch(() => ({}))) as { value?: number };
		if (resetResponse.status() !== 200 || typeof resetBody.value !== 'number') {
			throw new Error(
				`[${tenant.label}] reset failed: status=${resetResponse.status()}, body=${JSON.stringify(resetBody)}`
			);
		}

		if (seedValue > 0) {
			await incrementTenantCounter(request, tenant.userSub, seedValue);
		}

		const current = await readTenantCounter(request, tenant.userSub);
		if (current !== seedValue) {
			throw new Error(`[${tenant.label}] reset baseline mismatch: expected ${seedValue}, got ${current}`);
		}
	}
}

async function assertIsolationFlow(request: APIRequestContext, runLabel: string, seed: number, writes: number): Promise<void> {
	const tenantAStart = await withRetry(`${runLabel}:readTenantCounter(tenant-1:start)`, () =>
		readTenantCounter(request, TENANT_A.userSub)
	);
	expect(tenantAStart, `[${runLabel}] tenant-1 baseline mismatch: expected ${seed}, got ${tenantAStart}`).toBe(seed);

	await withRetry(`${runLabel}:incrementTenantCounter(tenant-1)`, () =>
		incrementTenantCounter(request, TENANT_A.userSub, writes)
	);

	const tenantAAfter = await withRetry(`${runLabel}:readTenantCounter(tenant-1:after)`, () =>
		readTenantCounter(request, TENANT_A.userSub)
	);
	const expectedTenantA = seed + writes;
	expect(
		tenantAAfter,
		`[${runLabel}] tenant-1 write result mismatch: expected ${expectedTenantA}, got ${tenantAAfter}`
	).toBe(expectedTenantA);

	const tenantBAfter = await withRetry(`${runLabel}:readTenantCounter(tenant-2:after)`, () =>
		readTenantCounter(request, TENANT_B.userSub)
	);
	expect(
		tenantBAfter,
		`[${runLabel}] tenant-2 leaked after tenant-1 writes: expected ${seed}, got ${tenantBAfter}`
	).toBe(seed);
}
