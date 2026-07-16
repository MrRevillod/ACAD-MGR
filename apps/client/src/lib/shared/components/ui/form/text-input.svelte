<script lang="ts">
	import type { Snippet } from "svelte"
	import type { FieldElementProps } from "@formisch/svelte"

	import InputLabel from "./label.svelte"
	import InputErrors from "./errors.svelte"

	interface Props extends FieldElementProps {
		class?: string
		type: "text" | "email" | "tel" | "password" | "url" | "number" | "date"
		label?: string
		placeholder?: string
		required?: boolean
		input: unknown
		errors: [string, ...string[]] | null
		rightIcon?: Snippet
	}

	let {
		class: className,
		label,
		name,
		required,
		input,
		errors,
		rightIcon,
		...fieldProps
	}: Props = $props()

	let value: string | number | undefined = $state()

	$effect(() => {
		if (typeof input === "string" || typeof input === "number") {
			value = input
		}
	})
</script>

<div class={["grid gap-1.5", className]}>
	<InputLabel {name} {label} {required} />
	<div class="relative">
		<input
			{...fieldProps}
			id={name}
			{name}
			class={[
				"h-10 w-full rounded-lg border bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50",
				errors ? "border-red-500" : "border-corp-gray/20 focus:border-corp-blue/50",
			]}
			{value}
			{required}
			aria-invalid={!!errors}
			aria-errormessage={`${name}-error`}
		/>
		{#if rightIcon}
			<div class="absolute right-2.5 top-1/2 -translate-y-1/2">
				{@render rightIcon()}
			</div>
		{/if}
	</div>
	<InputErrors {name} {errors} />
</div>
