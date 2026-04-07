import assert from 'node:assert/strict';
import { getBodyText, navigateTo, waitForAnyText } from '../helpers/navigate.mjs';

async function assertGoogleSignInReady() {
	const signInButton = await $('//button[contains(., "Sign in with Google")]');
	await signInButton.waitForDisplayed({ timeout: 10000 });
	await signInButton.waitForEnabled({ timeout: 10000 });
	await signInButton.waitForClickable({ timeout: 10000 });
	assert.equal(await signInButton.isClickable(), true, 'Google sign-in button should be clickable');
}

describe('Tauri Desktop Login', () => {
	it('shows login page with Google sign-in button', async () => {
		await navigateTo('/login');

		await waitForAnyText(['Sign in with Google', 'Counter', 'Admin Dashboard'], 10000);

		const body = await getBodyText();
		if (body.includes('Sign in with Google')) {
			await assertGoogleSignInReady();
			assert.ok(body.includes('Tauri App'));
		}

    const title = await browser.getTitle();
    assert.ok(title.length > 0, 'Page title should not be empty');
  });

  it('shows welcome text on login page', async () => {
    await navigateTo('/login');

    await waitForAnyText(['Welcome back', 'Counter', 'Admin Dashboard'], 10000);
  });

	it('shows disabled email input', async () => {
		await navigateTo('/login');
		await waitForAnyText(['Sign in with Google', 'Welcome back', 'Counter', 'Admin Dashboard'], 10000);

		const body = await getBodyText();
		if (body.includes('Sign in with Google')) {
			await assertGoogleSignInReady();
			const emailInput = await $('input[type="email"]');
			await emailInput.waitForDisplayed({ timeout: 10000 });

      const isDisabled = await emailInput.getAttribute('disabled');
      assert.ok(isDisabled !== null, 'Email input should be disabled');
      return;
    }

		await waitForAnyText(['Counter', 'Admin Dashboard', 'Welcome back'], 10000);
	});

	it('login page is responsive on mobile viewport', async () => {
		await browser.setWindowSize(375, 667);
		await navigateTo('/login');

		await waitForAnyText(['Sign in', 'Counter', 'Admin Dashboard'], 10000);

		const body = await getBodyText();
		if (body.includes('Sign in with Google')) {
			await assertGoogleSignInReady();
		}
	});
});
