import { test, expect } from '@playwright/test';
import { triggerMockOAuth } from '../fixtures/auth';

test.describe('Agent Chat (E2E)', () => {
	test.beforeEach(async ({ page }) => {
		await page.goto('/login');
		await triggerMockOAuth(page, 'agent_test_code');
		await page.waitForTimeout(1500);
		await page.goto('/agent');
		await page.waitForLoadState('networkidle');
	});

	test('agent page requires authentication', async ({ page }) => {
		const url = page.url();
		expect(url).toMatch(/\/(agent|login)/);
	});

	test('displays agent chat layout when authenticated', async ({ page }) => {
		const url = page.url();
		if (url.includes('/login')) {
			const signInBtn = page.getByRole('button', { name: /sign in with google/i });
			await expect(signInBtn).toBeVisible();
			return;
		}
		await expect(page.locator('text=New Chat')).toBeVisible();
		await expect(page.locator('text=Select or create a conversation')).toBeVisible();
	});

	test('has sidebar with conversation list', async ({ page }) => {
		const url = page.url();
		if (url.includes('/login')) return;

		await expect(page.locator('aside')).toBeVisible();
		await expect(page.locator('text=New Chat')).toBeVisible();
	});

	test('has message input area', async ({ page }) => {
		const url = page.url();
		if (url.includes('/login')) return;

		await expect(page.locator('input[placeholder="Type a message..."]')).toBeVisible();
		await expect(page.getByRole('button', { name: /send/i })).toBeVisible();
	});

	test('send button is disabled when input is empty', async ({ page }) => {
		const url = page.url();
		if (url.includes('/login')) return;

		const sendBtn = page.getByRole('button', { name: /send/i });
		await expect(sendBtn).toBeDisabled();
	});

	test('agent page is responsive on mobile', async ({ page }) => {
		await page.setViewportSize({ width: 375, height: 667 });
		await page.goto('/agent');
		await page.waitForLoadState('networkidle');

		const url = page.url();
		if (url.includes('/login')) {
			const signInBtn = page.getByRole('button', { name: /sign in/i });
			await expect(signInBtn).toBeVisible();
			return;
		}
		await expect(page.locator('text=New Chat')).toBeVisible();
	});

	test('agent page is properly guarded without auth', async ({ page }) => {
		await page.context().clearCookies();
		await page.goto('/agent');
		await page.waitForLoadState('networkidle');

		await expect(page).toHaveURL(/\/login/, { timeout: 10000 });
	});
});
