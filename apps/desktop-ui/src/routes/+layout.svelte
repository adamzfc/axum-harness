<script lang="ts">
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import '../app.css';
	import { setSession, initAuthListeners } from '$lib/stores/auth';
	import { handleOAuthCallback } from '$lib/ipc/auth';

	const { children } = $props();

	// Initialize auth:expired listener
	let cleanupAuth: (() => void) | undefined;

	onMount(() => {
		// Listen for auth:expired events from refresh timer
		cleanupAuth = initAuthListeners();

		// Listen for deep link URLs from tauri-plugin-deep-link
		const unlistenDeepLink = listen<string>('deep-link://new-url', async (event) => {
			const url = event.payload;
			if (url.includes('oauth/callback')) {
				try {
					const session = await handleOAuthCallback(url);
					setSession(session);
					// Navigate to /counter on success (login page will redirect via $effect)
				} catch (e) {
					console.error('OAuth callback failed:', e);
				}
			}
		});

		return () => {
			cleanupAuth?.();
			unlistenDeepLink.then((fn) => fn());
		};
	});
</script>

{@render children()}
