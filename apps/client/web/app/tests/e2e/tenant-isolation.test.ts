import { test, expect, type APIRequestContext } from '@playwright/test';
import { TENANT_A, TENANT_B, resetTenantPairCounter } from '../fixtures/tenant';
import { buildTenantAuthHeaders } from '../fixtures/auth';
import { ensureApiReady } from '../fixtures/runtime';

const COUNTER_VALUE_URL = 'http://127.0.0.1:3001/api/counter/value';
const COUNTER_INCREMENT_URL = 'http://127.0.0.1:3001/api/counter/increment';

test.describe('Tenant Isolation (E2E)', () => {
	test.describe.configure({ mode: 'serial' });
	const COUNTER_START = 0;
	const TENANT_A_MUTATIONS = 2;

	test.beforeEach(async ({ page }) => {
		await ensureApiReady();
		await resetTenantPairCounter(page);
	});

	test('tenant-1 write does not alter tenant-2 value (run-1)', async ({ request }) => {
		await assertTenantIsolationFlow({ request, runLabel: 'run-1', seed: COUNTER_START, mutations: TENANT_A_MUTATIONS });
	});

	test('tenant-1 write does not alter tenant-2 value (run-2, same seed)', async ({ request }) => {
		await assertTenantIsolationFlow({ request, runLabel: 'run-2', seed: COUNTER_START, mutations: TENANT_A_MUTATIONS });
	});
});

type IsolationFlowArgs = {
	request: APIRequestContext;
	runLabel: string;
	seed: number;
	mutations: number;
};


async function readTenantCounter(request: APIRequestContext, userSub: string): Promise<number> {
	const response = await request.get(COUNTER_VALUE_URL, {
		headers: buildTenantAuthHeaders(userSub)
	});
	const body = (await response.json().catch(() => ({}))) as { value?: number };
	if (response.status() !== 200 || typeof body.value !== 'number') {
		throw new Error(`read counter failed for ${userSub}: status=${response.status()}, body=${JSON.stringify(body)}`);
	}
	return body.value;
}

async function incrementTenantCounter(request: APIRequestContext, userSub: string, times: number): Promise<void> {
	for (let i = 0; i < times; i += 1) {
		const response = await request.post(COUNTER_INCREMENT_URL, {
			headers: buildTenantAuthHeaders(userSub)
		});
		const body = (await response.json().catch(() => ({}))) as { value?: number };
		if (response.status() !== 200 || typeof body.value !== 'number') {
			throw new Error(
				`increment failed for ${userSub} at step ${i + 1}: status=${response.status()}, body=${JSON.stringify(body)}`
			);
		}
	}
}

async function assertTenantIsolationFlow({ request, runLabel, seed, mutations }: IsolationFlowArgs): Promise<void> {
	const tenantAStart = await readTenantCounter(request, TENANT_A.userSub);
	expect(
		tenantAStart,
		`[${runLabel}] expected tenant-1 baseline ${seed}, got ${tenantAStart}`
	).toBe(seed);

	await incrementTenantCounter(request, TENANT_A.userSub, mutations);
	const tenantAAfter = await readTenantCounter(request, TENANT_A.userSub);
	const expectedTenantA = seed + mutations;
	expect(
		tenantAAfter,
		`[${runLabel}] tenant-1 expected ${expectedTenantA} after ${mutations} writes, got ${tenantAAfter}`
	).toBe(expectedTenantA);

	const tenantBAfter = await readTenantCounter(request, TENANT_B.userSub);
	expect(
		tenantBAfter,
		`[${runLabel}] tenant-2 leaked value after tenant-1 writes: expected ${seed}, got ${tenantBAfter}`
	).toBe(seed);
}
