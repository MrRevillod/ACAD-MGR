<script lang="ts">
	interface Props {
		component?: "label" | "legend" | "div"
		name?: string
		label?: string
		hint?: string
		required?: boolean
	}

	let { component, name, label, hint, required }: Props = $props()

	const element = $derived(component ?? (name ? "label" : "div"))
</script>

{#if label}
	<svelte:element
		this={element}
		class="inline-block text-xs font-medium tracking-wide uppercase text-corp-gray"
		for={element === "label" ? name : undefined}
	>
		{label}
		{#if hint}
			<span class="ml-1 text-[10px] font-normal normal-case text-corp-gray/50">{hint}</span>
		{/if}
		{#if required}
			<span class="ml-0.5 text-red-500">*</span>
		{/if}
	</svelte:element>
{/if}
