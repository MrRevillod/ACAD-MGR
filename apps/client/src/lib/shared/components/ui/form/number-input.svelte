<script lang="ts">
	import type { FieldElementProps } from "@formisch/svelte"
	import type { HTMLInputAttributes } from "svelte/elements"

	import { Minus, Plus } from "@lucide/svelte"
	import InputLabel from "./label.svelte"
	import InputErrors from "./errors.svelte"

	interface Props extends FieldElementProps {
		class?: string
		label?: string
		hint?: string
		placeholder?: string
		required?: boolean
		min?: number
		max?: number
		step?: number
		autocomplete?: HTMLInputAttributes["autocomplete"]
		input: unknown
		errors: [string, ...string[]] | null
	}

	let {
		class: className,
		label,
		hint,
		name,
		required,
		min,
		max,
		step = 1,
		input,
		errors,
		oninput: formOninput,
		...fieldProps
	}: Props = $props()

	const parse = (raw: string | number): number => {
		if (typeof raw === "number") return Number.isFinite(raw) ? raw : 0
		const n = Number.parseFloat(raw.replace(",", "."))
		return Number.isFinite(n) ? n : 0
	}

	const toInput = (value: number): string => {
		if (Number.isInteger(value)) return String(value)
		return String(value).replace(".", ",")
	}

	let lastGood: string = $state("")
	let value: string = $state("")
	let inputEl: HTMLInputElement | undefined = $state()

	$effect(() => {
		if (typeof input === "string" || typeof input === "number") {
			const current = parse(input)
			lastGood = toInput(current)
			value = toInput(current)
		}
	})

	function handleInput(e: Event) {
		const el = e.currentTarget
		if (!(el instanceof HTMLInputElement)) return

		const raw = el.value
		const sanitized = raw.replace(/[^\d.,-]/g, "")

		if (raw !== sanitized) {
			el.value = sanitized
		}

		value = sanitized
		lastGood = sanitized
		formOninput?.(e as Parameters<typeof formOninput>[0])
	}

	function adjust(delta: number) {
		const next = parse(lastGood) + delta
		const clamped = Math.max(min ?? -Infinity, Math.min(max ?? Infinity, next))
		const rounded = Math.round(clamped / step) * step
		const out = toInput(rounded)
		lastGood = out
		value = out

		if (inputEl) {
			inputEl.value = out
			inputEl.dispatchEvent(new Event("input", { bubbles: true }))
		}
	}
</script>

<div class={["grid gap-1.5", className]}>
	<InputLabel {name} {label} {hint} {required} />

	<div class="flex items-center gap-2">
		<button
			type="button"
			class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg border border-corp-gray/20 text-corp-gray transition-colors hover:border-corp-blue/50 hover:text-corp-blue disabled:cursor-not-allowed disabled:opacity-40"
			onclick={() => adjust(-step)}
			disabled={min !== undefined && parse(lastGood) <= min}
			aria-label="Disminuir"
		>
			<Minus class="size-4" />
		</button>

		<input
			{...fieldProps}
			id={name}
			{name}
			type="text"
			inputmode="decimal"
			class={[
				"h-10 w-full min-w-0 flex-1 rounded-lg border bg-white px-3 text-center text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50",
				errors ? "border-red-500" : "border-corp-gray/20 focus:border-corp-blue/50",
			]}
			{value}
			{required}
			bind:this={inputEl}
			aria-invalid={!!errors}
			aria-errormessage={`${name}-error`}
			oninput={handleInput}
		/>

		<button
			type="button"
			class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg border border-corp-gray/20 text-corp-gray transition-colors hover:border-corp-blue/50 hover:text-corp-blue disabled:cursor-not-allowed disabled:opacity-40"
			onclick={() => adjust(step)}
			disabled={max !== undefined && parse(lastGood) >= max}
			aria-label="Aumentar"
		>
			<Plus class="size-4" />
		</button>
	</div>

	<InputErrors {name} {errors} />
</div>
