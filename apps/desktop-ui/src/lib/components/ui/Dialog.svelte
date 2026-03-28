<script lang="ts">
import { Dialog as BitsDialog } from 'bits-ui';
import type { Snippet } from 'svelte';

interface Props {
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  children: Snippet;
}

let { open = $bindable(false), children, ...restProps }: Props = $props();
</script>

<BitsDialog.Root bind:open {...restProps}>
	{@render children()}
</BitsDialog.Root>
