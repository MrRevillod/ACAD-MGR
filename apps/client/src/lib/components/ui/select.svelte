<script lang="ts">
	import { Select as SelectPrimitive } from "bits-ui"
	import { ChevronDown, Check } from "@lucide/svelte"

	type Item = {
		value: string
		label: string
	}

	let {
		items = [],
		placeholder = "Seleccionar...",
		value = $bindable(""),
		class: className = "",
	}: {
		items: Item[]
		placeholder?: string
		value?: string
		class?: string
	} = $props()
</script>

<SelectPrimitive.Root type="single" bind:value {items}>
	<SelectPrimitive.Trigger
		class="inline-flex h-10 w-full touch-none select-none items-center rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10 data-placeholder:text-corp-gray/50 {className}"
	>
		<SelectPrimitive.Value {placeholder} />
		<ChevronDown class="ml-auto size-4 shrink-0 text-corp-gray/60" />
	</SelectPrimitive.Trigger>
	<SelectPrimitive.Portal>
		<SelectPrimitive.Content
			class="z-50 max-h-64 w-[var(--bits-select-anchor-width)] min-w-[8rem] overflow-hidden rounded-xl border border-corp-gray/20 bg-white p-1 shadow-lg"
			sideOffset={4}
		>
			<SelectPrimitive.Viewport>
				{#each items as item (item.value)}
					<SelectPrimitive.Item
						class="flex h-9 w-full cursor-pointer select-none items-center rounded-lg px-3 py-2 text-sm text-[#1A1A1A] outline-none transition-colors data-highlighted:bg-corp-blue/5 data-state-checked:bg-corp-blue/10"
						value={item.value}
						label={item.label}
					>
						{#snippet children({ selected })}
							<span class="flex-1">{item.label}</span>
							{#if selected}
								<Check class="size-4 shrink-0 text-corp-blue" />
							{/if}
						{/snippet}
					</SelectPrimitive.Item>
				{/each}
			</SelectPrimitive.Viewport>
		</SelectPrimitive.Content>
	</SelectPrimitive.Portal>
</SelectPrimitive.Root>
