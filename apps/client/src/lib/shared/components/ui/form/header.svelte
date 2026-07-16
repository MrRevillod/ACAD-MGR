<script lang="ts">
	import { type FormStore, reset } from "@formisch/svelte"
	import Button from "$shared/components/ui/button.svelte"

	interface Props {
		of: FormStore
		heading: string
		description?: string
	}

	let { of: formStore, heading, description }: Props = $props()
</script>

<div class="flex items-start justify-between gap-4 border-b border-corp-gray/20 pb-4 mb-4">
	<div>
		<h2 class="text-lg font-semibold text-[#1A1A1A]">{heading}</h2>
		{#if description}
			<p class="mt-1 text-sm text-corp-gray">{description}</p>
		{/if}
	</div>
	<Button variant="ghost" size="sm" type="button" onclick={() => reset(formStore)}>
		Limpiar
	</Button>
</div>
