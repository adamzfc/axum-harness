import { test, expect, type Page } from '@playwright/test';
import { triggerMockOAuth } from '../fixtures/auth';
import {
	TENANT_A,
	TENANT_B,
	resetTenantPairCounter
} from '../fixtures/tenant';

test.describe('Tenant Isolation (E2E)', () => {
	test.describe.configure({ mode: 'serial' });
	const COUNTER_START = 0;
	const TENANT_A_MUTATIONS = 2;

	test.beforeEach(async ({ page }) => {
		await resetTenantPairCounter(page);
	});

	test('tenant-1 write does not alter tenant-2 value (run-1)', async ({ page }) => {
		await assertTenantIsolationFlow({ page, runLabel: 'run-1', seed: COUNTER_START, mutations: TENANT_A_MUTATIONS });
	});

	test('tenant-1 write does not alter tenant-2 value (run-2, same seed)', async ({ page }) => {
		await assertTenantIsolationFlow({ page, runLabel: 'run-2', seed: COUNTER_START, mutations: TENANT_A_MUTATIONS });
	});
});

type IsolationFlowArgs = {
	page: Page;
	runLabel: string;
	seed: number;
	mutations: number;
};

async function readCounterValue(page: Page): Promise<number> {
	const counterDisplay = page.locator('.font-mono');
	await counterDisplay.waitFor({ state: 'visible', timeout: 10000 });
	const text = (await counterDisplay.textContent())?.trim() ?? '';
	const parsed = Number(text);
	if (Number.isNaN(parsed)) {
		throw new Error(`counter value is not numeric: "${text}"`);
	}
	return parsed;
}

async function openCounterAsTenant(page: Page, mockCode: string): Promise<void> {
	await page.goto('/login');
	await triggerMockOAuth(page, mockCode);
	await page.waitForTimeout(1000);
	await page.goto('/counter');
	await page.waitForLoadState('networkidle');
}

async function incrementCounter(page: Page, times: number): Promise<void> {
	for (let i = 0; i < times; i += 1) {
		await page.locator('button').nth(1).click();
	}
}

async function assertTenantIsolationFlow({ page, runLabel, seed, mutations }: IsolationFlowArgs): Promise<void> {
	await openCounterAsTenant(page, TENANT_A.mockCode);
	const tenantAStart = await readCounterValue(page);
	expect(
		tenantAStart,
		`[${runLabel}] expected tenant-1 baseline ${seed}, got ${tenantAStart}`
	).toBe(seed);

	await incrementCounter(page, mutations);
	const tenantAAfter = await readCounterValue(page);
	const expectedTenantA = seed + mutations;
	expect(
		tenantAAfter,
		`[${runLabel}] tenant-1 expected ${expectedTenantA} after ${mutations} writes, got ${tenantAAfter}`
	).toBe(expectedTenantA);

	await openCounterAsTenant(page, TENANT_B.mockCode);
	const tenantBAfter = await readCounterValue(page);
	expect(
		tenantBAfter,
		`[${runLabel}] tenant-2 leaked value after tenant-1 writes: expected ${seed}, got ${tenantBAfter}`
	).toBe(seed);
}
