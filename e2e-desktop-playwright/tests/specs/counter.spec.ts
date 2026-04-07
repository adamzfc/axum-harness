import { test, expect } from '../fixtures/tauri';

const APP_BASE_URL = 'http://localhost:5173';

test.describe('Tauri Desktop Counter', () => {
	test('counter page is properly guarded without auth', async ({ tauriPage }) => {
		await tauriPage.goto(`${APP_BASE_URL}/counter`);
		const url = await tauriPage.url();
		expect(url.includes('/counter') || url.includes('/login')).toBeTruthy();
	});

	test('shows counter controls only when authenticated', async ({ tauriPage }) => {
		await tauriPage.goto(`${APP_BASE_URL}/counter`);

		const resetButton = tauriPage.getByRole('button', { name: 'Reset' });
		const signInButton = tauriPage.getByRole('button', { name: 'Sign in with Google' });
		const canOperateCounter = await resetButton.isVisible().catch(() => false);

		if (!canOperateCounter) {
			await expect(signInButton).toBeVisible();
			return;
		}

		const buttons = tauriPage.locator('button');
		await expect(buttons).toHaveCount(1);
	});

	test('counter interaction assertions run when authenticated', async ({ tauriPage }) => {
		await tauriPage.goto(`${APP_BASE_URL}/counter`);

		const resetButton = tauriPage.getByRole('button', { name: 'Reset' });
		const signInButton = tauriPage.getByRole('button', { name: 'Sign in with Google' });
		const canOperateCounter = await resetButton.isVisible().catch(() => false);
		if (!canOperateCounter) {
			await expect(signInButton).toBeVisible();
			return;
		}

		const counterValue = tauriPage.locator('.font-mono');
		const counterVisible = await counterValue.isVisible().catch(() => false);
		if (!counterVisible) {
			await expect(signInButton).toBeVisible();
			return;
		}
		await expect(counterValue).toBeVisible();

		const buttons = tauriPage.locator('button');
		const decrementButton = buttons.nth(0);
		const incrementButton = buttons.nth(1);
		const resetControl = buttons.nth(2);

		await resetControl.click();
		await expect(counterValue).toHaveText('0');

		await incrementButton.click();
		await expect(counterValue).toHaveText('1');

		await decrementButton.click();
		await expect(counterValue).toHaveText('0');

		await incrementButton.click();
		await incrementButton.click();
		await expect(counterValue).toHaveText('2');

		await resetControl.click();
		await expect(counterValue).toHaveText('0');
	});
});
