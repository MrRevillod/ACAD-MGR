<script lang="ts">
	import type { Snippet } from "svelte"
	import Button from "$lib/shared/components/ui/button.svelte"

	interface Props {
		onCancel?: () => void
		submitLabel?: string
		isPending?: boolean
		children?: Snippet
	}

	let { onCancel, submitLabel = "Guardar", isPending = false, children }: Props = $props()
</script>

<div class="flex items-center justify-end gap-2 border-t border-corp-gray/20 pt-4 mt-4">
	{#if children}
		{@render children()}
	{/if}
	{#if onCancel}
		<Button variant="secondary" type="button" onclick={onCancel}>Cancelar</Button>
	{/if}
	<Button type="submit" disabled={isPending}>
		{isPending ? `${submitLabel}...` : submitLabel}
	</Button>
</div>
