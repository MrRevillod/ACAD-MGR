<script lang="ts">
	import type { FieldElementProps } from "@formisch/svelte"

	import { countryItems } from "$shared/countries"
	import Select from "./select.svelte"

	interface Props extends Omit<FieldElementProps, "onchange"> {
		class?: string
		label?: string
		placeholder?: string
		required?: boolean
		input: string | string[] | null | undefined
		errors: [string, ...string[]] | null
		disabled?: boolean
		onValueChange?: (value: string) => void
	}

	let {
		class: className,
		label = "País de nacionalidad",
		placeholder = "Seleccionar país...",
		required,
		input,
		errors,
		disabled,
		onValueChange,
		...fieldProps
	}: Props = $props()

	const countryOptions = $derived(countryItems.map((c) => ({ label: c.label, value: c.value })))
</script>

<Select
	{...fieldProps}
	class={className}
	{label}
	{placeholder}
	{required}
	{input}
	{errors}
	{disabled}
	{onValueChange}
	options={countryOptions}
/>
