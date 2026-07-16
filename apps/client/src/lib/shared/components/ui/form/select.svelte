<script lang="ts">
	import type { FieldElementProps } from "@formisch/svelte"

	import { ChevronDown, Check } from "@lucide/svelte"
	import { Select as SelectPrimitive } from "bits-ui"

	import InputLabel from "./label.svelte"
	import InputErrors from "./errors.svelte"

	interface Item {
		label: string
		value: string
	}

	interface Props extends Omit<FieldElementProps, "onchange"> {
		class?: string
		label?: string
		options: Item[]
		placeholder?: string
		required?: boolean
		input: string | string[] | null | undefined
		errors: [string, ...string[]] | null
		disabled?: boolean
		onValueChange?: (value: string) => void
	}

	let {
		class: className,
		label,
		name,
		options,
		placeholder = "",
		required,
		input,
		errors,
		disabled,
		onValueChange,
		...fieldProps
	}: Props = $props()

	let selectedValue = $state("")
	let hiddenRef: HTMLSelectElement | undefined = $state()

	$effect(() => {
		const val = typeof input === "string" ? input : ""
		if (val !== selectedValue) {
			selectedValue = val
		}
	})

	function handleValueChange(v: string | undefined) {
		const val = v ?? ""
		if (val === selectedValue) return
		selectedValue = val
		onValueChange?.(val)
		if (hiddenRef) {
			hiddenRef.value = val
			hiddenRef.dispatchEvent(new Event("change", { bubbles: true }))
			hiddenRef.dispatchEvent(new Event("input", { bubbles: true }))
		}
	}
</script>

<div class={["grid gap-1.5", className]}>
	<InputLabel {name} {label} {required} />

	<select
		bind:this={hiddenRef}
		class="sr-only"
		aria-hidden="true"
		tabindex={-1}
		{...fieldProps}
		{name}
		{required}
	>
		<option value="">{placeholder}</option>
		{#each options as opt (opt.value)}
			<option value={opt.value}>{opt.label}</option>
		{/each}
	</select>

	<SelectPrimitive.Root
		type="single"
		value={selectedValue}
		onValueChange={handleValueChange}
		{disabled}
	>
		<SelectPrimitive.Trigger
			class="inline-flex h-10 w-full items-center rounded-lg border bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors data-placeholder:text-corp-gray/50 {errors
				? 'border-red-500'
				: 'border-corp-gray/20 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10'}"
		>
			<SelectPrimitive.Value {placeholder} />
			<ChevronDown class="ml-auto size-4 shrink-0 text-corp-gray/60" />
		</SelectPrimitive.Trigger>
		<SelectPrimitive.Portal>
			<SelectPrimitive.Content
				class="z-50 max-h-64 w-(--bits-select-anchor-width) min-w-32 overflow-hidden rounded-xl border border-corp-gray/20 bg-white p-1 shadow-lg"
				sideOffset={4}
			>
				<SelectPrimitive.Viewport>
					{#each options as item (item.value)}
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

	<InputErrors {name} {errors} />
</div>
