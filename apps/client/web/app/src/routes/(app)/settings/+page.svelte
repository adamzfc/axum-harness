<script lang="ts">
	import { onMount } from 'svelte';
	import { Button, Card, Input } from '$lib/components';

	let apiKey = $state('');
	let baseUrl = $state('https://api.openai.com/v1');
	let model = $state('gpt-4o-mini');
	let saving = $state(false);
	let saved = $state(false);

	async function loadSettings() {
		try {
			if (typeof window !== 'undefined' && (window as { __TAURI__?: unknown }).__TAURI__) {
				const { Store } = await import('@tauri-apps/plugin-store');
				const store = await Store.load('settings.json');

				apiKey = ((await store.get('api_key')) as string | null) ?? '';
				baseUrl = ((await store.get('base_url')) as string | null) ?? 'https://api.openai.com/v1';
				model = ((await store.get('model')) as string | null) ?? 'gpt-4o-mini';
			}
		} catch {
			// ignore load failures
		}
	}

	async function saveSettings() {
		saving = true;
		saved = false;

		try {
			if (typeof window !== 'undefined' && (window as { __TAURI__?: unknown }).__TAURI__) {
				const { Store } = await import('@tauri-apps/plugin-store');
				const store = await Store.load('settings.json');

				await store.set('api_key', apiKey);
				await store.set('base_url', baseUrl);
				await store.set('model', model);
				await store.save();
			}

			saved = true;
			setTimeout(() => {
				saved = false;
			}, 2000);
		} catch {
			// ignore save failures
		} finally {
			saving = false;
		}
	}

	onMount(() => {
		loadSettings();
	});
</script>

<div class="p-4 md:p-6 max-w-lg mx-auto space-y-6">
	<div>
		<h1 class="text-2xl font-semibold text-[var(--color-text)]">Settings</h1>
		<p class="text-sm text-[var(--color-text-muted)] mt-1">Configure your Agent Chat API connection</p>
	</div>

	<Card class="p-5 space-y-4">
		<div>
			<label class="text-sm font-medium text-[var(--color-text)]" for="api_key">API Key</label>
			<Input id="api_key" type="password" bind:value={apiKey} placeholder="sk-..." class="mt-1" />
		</div>

		<div>
			<label class="text-sm font-medium text-[var(--color-text)]" for="base_url">Base URL</label>
			<Input
				id="base_url"
				bind:value={baseUrl}
				placeholder="https://api.openai.com/v1"
				class="mt-1"
			/>
		</div>

		<div>
			<label class="text-sm font-medium text-[var(--color-text)]" for="model">Model</label>
			<Input id="model" bind:value={model} placeholder="gpt-4o-mini" class="mt-1" />
		</div>

		<Button variant="primary" onclick={saveSettings} disabled={saving}>
			{saving ? 'Saving...' : saved ? 'Saved!' : 'Save Settings'}
		</Button>
	</Card>
</div>
