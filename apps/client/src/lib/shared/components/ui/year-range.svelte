<script lang="ts">
	import { Minus } from "@lucide/svelte"
	import Select from "$shared/components/ui/select.svelte"

	interface Props {
		yearFrom?: string
		yearTo?: string
		minYear?: number
		labelFrom?: string
		labelTo?: string
		showLabels?: boolean
		placeholderFrom?: string
		placeholderTo?: string
		class?: string
	}

	let {
		yearFrom = $bindable(""),
		yearTo = $bindable(""),
		minYear = 2000,
		labelFrom = "Año desde",
		labelTo = "Año hasta",
		showLabels = true,
		placeholderFrom = "Seleccionar",
		placeholderTo = "Seleccionar",
		class: className = "",
	}: Props = $props()

	const currentYear = new Date().getFullYear()

	const years = $derived(
		Array.from({ length: currentYear - minYear + 1 }, (_, i) => ({
			value: String(minYear + i),
			label: String(minYear + i),
		})).toReversed(),
	)
</script>

<div class={className} role="group" aria-label="Rango de años">
	<div class="flex items-end gap-2">
		<div class="flex-1 space-y-1">
			{#if showLabels}
				<span class="block text-xs font-medium text-corp-gray">{labelFrom}</span>
			{/if}
			<Select
				items={years}
				bind:value={yearFrom}
				placeholder={placeholderFrom}
				class="w-full"
			/>
		</div>
		<div class="flex items-center pb-3 px-0.5 text-corp-gray/40" aria-hidden="true">
			<Minus class="size-3" />
		</div>
		<div class="flex-1 space-y-1">
			{#if showLabels}
				<span class="block text-xs font-medium text-corp-gray">{labelTo}</span>
			{/if}
			<Select items={years} bind:value={yearTo} placeholder={placeholderTo} class="w-full" />
		</div>
	</div>
</div>
