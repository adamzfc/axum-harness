<script lang="ts">
import { Card } from '$lib/components';
import { onMount } from 'svelte';
import type { AdminDashboardStats } from '$lib/generated/api/AdminDashboardStats';

let stats = $state<AdminDashboardStats>({ tenant_count: 0, counter_value: 0, last_login: null, app_version: '0.0.0' });
let loading = $state(true);

const isTauri = typeof window !== 'undefined' && (window as any).__TAURI__;

async function fetchStats() {
	loading = true;
	try {
		if (isTauri) {
			stats = await (window as any).__TAURI__.core.invoke('admin_get_dashboard_stats');
		} else {
			const resp = await fetch('http://localhost:3001/api/admin/stats');
			stats = await resp.json();
		}
	} catch (e) {
		console.error('Failed to load stats:', e);
	}
	loading = false;
}

onMount(() => {
	fetchStats();
});

const statCards = $derived([
	{ label: 'Tenants', value: String(stats.tenant_count), icon: '🏢' },
	{ label: 'Counter', value: String(stats.counter_value), icon: '🔢' },
	{ label: 'Last Login', value: stats.last_login ? new Date(stats.last_login).toLocaleDateString() : 'N/A', icon: '👤' },
	{ label: 'Version', value: stats.app_version, icon: '📦' },
]);
</script>

<div class="p-4 md:p-6 space-y-6">
	<div>
		<h1 class="text-2xl font-semibold text-[var(--color-text)]">Admin Dashboard</h1>
		<p class="text-sm text-[var(--color-text-muted)] mt-1">Real-time application metrics</p>
	</div>
	<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
		{#each statCards as stat}
			<Card class="p-5">
				<div class="flex items-center justify-between">
					<p class="text-sm text-[var(--color-text-muted)]">{stat.label}</p>
					<span class="text-lg">{stat.icon}</span>
				</div>
				<p class="text-2xl font-semibold text-[var(--color-text)] mt-2">{loading ? '...' : stat.value}</p>
			</Card>
		{/each}
	</div>
</div>
