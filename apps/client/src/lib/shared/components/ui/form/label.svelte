<script lang="ts">
	interface Props {
		component?: "label" | "legend" | "div"
		name?: string
		label?: string
		required?: boolean
	}

	let { component, name, label, required }: Props = $props()

	const element = $derived(component ?? (name ? "label" : "div"))
</script>

{#if label}
	<svelte:element
		this={element}
		class="inline-block text-xs font-medium tracking-wide uppercase text-corp-gray"
		for={element === "label" ? name : undefined}
	>
		{label}
		{#if required}
			<span class="ml-0.5 text-red-500">*</span>
		{/if}
	</svelte:element>
{/if}
