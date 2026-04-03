import { invoke } from '@tauri-apps/api/core';
import type { UserProfile } from '$lib/generated/auth/UserProfile';
import type { TokenPair } from '$lib/generated/auth/TokenPair';

/**
 * Full auth session combining generated TokenPair + UserProfile.
 * This is a frontend composition type — not defined in contracts
 * because it aggregates multiple contract types for client-side use.
 */
export interface AuthSession {
	tokens: TokenPair;
	id_token: string;
	user: UserProfile;
}

export async function startOAuth(): Promise<void> {
	return invoke('start_oauth');
}

export async function handleOAuthCallback(url: string): Promise<AuthSession> {
	return invoke('handle_oauth_callback', { url });
}

export async function getSession(): Promise<AuthSession | null> {
	return invoke('get_session');
}

export async function clearAuthStore(): Promise<void> {
	const { Store } = await import('@tauri-apps/plugin-store');
	const store = await Store.load('auth.json');
	await store.delete('tokens');
	await store.delete('id_token');
	await store.delete('user');
}
