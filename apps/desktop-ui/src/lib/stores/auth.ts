import { goto } from '$app/navigation';
import { getSession, startOAuth, clearAuthStore, type AuthSession, type UserProfile } from '$lib/ipc/auth';

// Reactive state (Svelte 5 $state at module level)
export let isAuthenticated = $state(false);
export let currentUser = $state<UserProfile | null>(null);
export let authLoading = $state(false);
export let authError = $state<string | null>(null);
export let tokenExpiresAt = $state(0);

/**
 * Check stored session on app load. Returns true if valid session found.
 * Per D-11: detect valid token on page load, redirect to /counter if found.
 */
export async function checkSession(): Promise<boolean> {
	try {
		const session = await getSession();
		if (session && session.expires_at > Date.now() / 1000) {
			isAuthenticated = true;
			currentUser = session.user;
			tokenExpiresAt = session.expires_at;
			return true;
		}
		// Session expired — clear stale data
		if (session) {
			await clearAuthStore();
		}
		return false;
	} catch {
		return false;
	}
}

/**
 * Initiate Google OAuth login.
 * Per D-09: set authLoading for Lottie loading state in login page.
 */
export async function signInWithGoogle(): Promise<void> {
	authLoading = true;
	authError = null;
	try {
		await startOAuth();
		// Note: actual login completes when deep link callback fires
		// and handleOAuthCallback is called from the Tauri event handler
	} catch (e) {
		authError = String(e);
		authLoading = false;
	}
}

/**
 * Called by deep link callback handler after successful token exchange.
 */
export function setSession(session: AuthSession): void {
	isAuthenticated = true;
	currentUser = session.user;
	tokenExpiresAt = session.expires_at;
	authLoading = false;
	authError = null;
}

/**
 * Sign out: clear store, reset state, redirect to login.
 */
export async function signOut(): Promise<void> {
	await clearAuthStore();
	isAuthenticated = false;
	currentUser = null;
	tokenExpiresAt = 0;
	authError = null;
	await goto('/login');
}

/**
 * Mark auth as expired (called by refresh timer on failure).
 * Per D-07: silent expiry — clear tokens, next action triggers login redirect.
 */
export async function markExpired(): Promise<void> {
	await clearAuthStore();
	isAuthenticated = false;
	currentUser = null;
	tokenExpiresAt = 0;
}
