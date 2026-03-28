<script lang="ts">
import type { Snippet } from 'svelte';

interface Props {
  animate?: boolean;
  children: Snippet<[boolean]>;
}

let { animate = $bindable(false), children }: Props = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="contents"
	onmouseenter={() => (animate = true)}
	onmouseleave={() => (animate = false)}
>
	{@render children(animate)}
</div>
