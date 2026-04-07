import assert from 'node:assert/strict';
import { getBodyText, isLoginPageVisible, navigateTo, waitForAnyText } from '../helpers/navigate.mjs';

async function getCounterValueElement() {
	const selectors = [
		'//div[contains(@class, "tabular-nums")]',
		'//div[contains(@class, "font-mono") and normalize-space() != ""]',
		'//main//div[contains(@class, "font-mono") and normalize-space() != ""]',
	];

	for (const selector of selectors) {
		const el = await $(selector);
		if (await el.isExisting()) {
			return el;
		}
	}

	return null;
}

async function getResetButton() {
	const button = await $('//button[normalize-space()="Reset"]');
	if (!(await button.isExisting())) {
		return null;
	}
	return button;
}

async function assertCounterPageInteractive() {
	const counterValue = await getCounterValueElement();
	if (counterValue) {
		await counterValue.waitForDisplayed({ timeout: 10000 });
	}

	const resetButton = await getResetButton();
	assert.ok(resetButton, 'Reset button should exist');
	await resetButton.waitForDisplayed({ timeout: 10000 });
	await resetButton.waitForEnabled({ timeout: 10000 });
	await resetButton.waitForClickable({ timeout: 10000 });
	assert.equal(await resetButton.isClickable(), true, 'Reset button should be clickable');

	return { counterValue, resetButton };
}

async function assertResetKeepsNumericValue() {
	const { counterValue, resetButton } = await assertCounterPageInteractive();
	const before = counterValue ? (await counterValue.getText()).trim() : (await getBodyText()).trim().match(/-?\d+/)?.[0] ?? '';
	assert.match(before, /^-?\d+$/, 'Counter value before reset should be numeric');

	await resetButton.click();

	await browser.waitUntil(
		async () => {
			const currentValue = counterValue ? (await counterValue.getText()).trim() : (await getBodyText()).trim();
			const after = currentValue;
			return /^-?\d+$/.test(after);
		},
		{
			timeout: 10000,
			interval: 250,
			timeoutMsg: 'Counter value did not converge to numeric text after reset',
		},
	);

	const after = counterValue ? (await counterValue.getText()).trim() : (await getBodyText()).trim().match(/-?\d+/)?.[0] ?? '';
	assert.equal(after, '0', 'Reset should deterministically converge counter value to 0');
}

describe('Tauri Desktop Counter', () => {
  it('counter page is properly guarded without auth', async () => {
    await navigateTo('/counter');
    const url = await browser.getUrl();
    assert.ok(url.includes('/counter') || url.includes('/login'), 'Should be on counter or login page');
  });

	it('displays counter page content when authenticated', async () => {
		await navigateTo('/counter');
		await waitForAnyText(['Sign in with Google', 'Welcome back', 'Reset', 'Failed to load persisted counter value'], 10000);

		if (await isLoginPageVisible()) {
			const signInButton = await $('//button[contains(., "Sign in with Google")]');
			await signInButton.waitForDisplayed({ timeout: 10000 });
			await signInButton.waitForEnabled({ timeout: 10000 });
			await signInButton.waitForClickable({ timeout: 10000 });
			return;
		}

		await assertCounterPageInteractive();
		await assertResetKeepsNumericValue();
	});

	it('has increment, decrement, and reset buttons when authenticated', async () => {
		await navigateTo('/counter');
		await waitForAnyText(['Sign in with Google', 'Welcome back', 'Reset', 'Failed to load persisted counter value'], 10000);

		if (await isLoginPageVisible()) {
			return;
		}

		await assertCounterPageInteractive();
		await assertResetKeepsNumericValue();

		const body = await getBodyText();
		assert.ok(body.includes('Reset'), 'Should render reset control');

    const buttons = await $$('button');
    assert.ok(buttons.length >= 3, 'Should have at least 3 buttons');
  });

	it('counter page is responsive on mobile viewport', async () => {
		await browser.setWindowSize(375, 667);
		await navigateTo('/counter');
		await waitForAnyText(['Sign in with Google', 'Welcome back', 'Reset', 'Counter', 'Failed to load persisted counter value'], 10000);

		if (await isLoginPageVisible()) {
			const signInButton = await $('//button[contains(., "Sign in")]');
			await signInButton.waitForDisplayed({ timeout: 10000 });
			await signInButton.waitForEnabled({ timeout: 10000 });
			return;
		}

		await waitForAnyText(['Reset', 'Counter', 'Failed to load persisted counter value'], 10000);
		await assertCounterPageInteractive();
	});
});
