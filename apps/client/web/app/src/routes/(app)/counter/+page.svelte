<script lang="ts">
import { Button } from '$lib/components';
import { Plus, Minus, RotateCcw } from '@jis3r/icons';
import { onMount } from 'svelte';

let count = $state(0);
let loading = $state(false);

const isTauri = typeof window !== 'undefined' && (window as any).__TAURI__;

async function invokeCommand(cmd: string) {
	if (isTauri) {
		return (window as any).__TAURI__.core.invoke(cmd);
	} else {
		const method = cmd.startsWith('counter_get') ? 'GET' : 'POST';
		const endpoint = cmd.replace('counter_', '').replace('get_value', 'value');
		const url = `http://localhost:3001/api/counter/${endpoint}`;
		const resp = await fetch(url, { method });
		const data = await resp.json();
		return data.value;
	}
}

async function loadValue() {
	loading = true;
	try {
		count = await invokeCommand('counter_get_value');
	} catch {}
	loading = false;
}

async function increment() {
	count = await invokeCommand('counter_increment');
}

async function decrement() {
	count = await invokeCommand('counter_decrement');
}

async function reset() {
	count = await invokeCommand('counter_reset');
}

onMount(() => {
	loadValue();
});
</script>

<div class="flex min-h-screen flex-col items-center justify-center gap-8 bg-[var(--color-bg)] px-4">
	<!-- Counter Display -->
	<div class="font-mono text-8xl sm:text-9xl text-[var(--color-text)] tabular-nums py-8 select-none">
		{count}
	</div>

	<!-- Controls -->
	<div class="flex flex-row items-center gap-4">
		<Button
			variant="secondary"
			size="lg"
			class="h-12 w-12"
			onclick={decrement}
		>
			{#snippet icon()}
				<Minus class="h-5 w-5" />
			{/snippet}
			{''}
		</Button>

		<Button
			variant="primary"
			size="lg"
			class="h-12 w-12"
			onclick={increment}
		>
			{#snippet icon()}
				<Plus class="h-5 w-5" />
			{/snippet}
			{''}
		</Button>
	</div>

	<!-- Reset -->
	<Button
		variant="ghost"
		size="md"
		onclick={reset}
	>
		{#snippet icon()}
			<RotateCcw class="h-4 w-4" />
		{/snippet}
		Reset
	</Button>
</div>
