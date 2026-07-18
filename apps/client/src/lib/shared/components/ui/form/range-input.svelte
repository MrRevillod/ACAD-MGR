<script lang="ts">
	import type { FieldElementProps } from "@formisch/svelte"

	import InputLabel from "./label.svelte"
	import InputErrors from "./errors.svelte"

	interface Props extends FieldElementProps {
		class?: string
		label?: string
		hint?: string
		min?: number
		max?: number
		step?: number
		required?: boolean
		input: unknown
		errors: [string, ...string[]] | null
	}

	let {
		class: className,
		label,
		hint,
		min = 0,
		max = 1,
		step = 0.1,
		name,
		required,
		input,
		errors,
		...fieldProps
	}: Props = $props()

	let rawValue: number = $state(0)

	$effect(() => {
		if (typeof input === "string") {
			const n = Number.parseFloat(input)
			rawValue = Number.isFinite(n) ? n : 0
		} else if (typeof input === "number") {
			rawValue = Number.isFinite(input) ? input : 0
		}
	})

	const displayValue = $derived(rawValue.toFixed(1))
	const fillPercent = $derived(((rawValue - min) / (max - min)) * 100)

	function handleInput(e: Event) {
		const el = e.currentTarget
		if (el instanceof HTMLInputElement) {
			rawValue = Number.parseFloat(el.value)
		}
	}
</script>

<div class={["grid gap-1.5", className]}>
	<InputLabel {name} {label} {hint} {required} />

	<div class="flex items-center gap-3">
		<div class="relative flex-1">
			<input
				{...fieldProps}
				id={name}
				{name}
				type="range"
				{min}
				{max}
				{step}
				value={rawValue}
				oninput={handleInput}
				class="range"
				style="--fill: {fillPercent}%;"
				{required}
				aria-invalid={!!errors}
				aria-errormessage={errors ? `${name}-error` : undefined}
			/>
		</div>
		<span class="min-w-10 text-right text-sm font-medium tabular-nums text-[#1A1A1A]">
			{displayValue}
		</span>
	</div>

	<InputErrors {name} {errors} />
</div>

<style>
	input[type="range"].range {
		-webkit-appearance: none;
		appearance: none;
		width: 100%;
		height: 6px;
		border-radius: 3px;
		background: linear-gradient(
			to right,
			#0075b4 0%,
			#0075b4 var(--fill, 0%),
			#e5e7eb var(--fill, 0%),
			#e5e7eb 100%
		);
		outline: none;
		cursor: pointer;
		transition: background 0.1s ease;
	}

	input[type="range"].range::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background: #0075b4;
		border: 2px solid white;
		box-shadow: 0 1px 4px rgba(0, 0, 0, 0.2);
		cursor: pointer;
		transition: transform 0.15s ease;
	}

	input[type="range"].range::-webkit-slider-thumb:hover {
		transform: scale(1.15);
	}

	input[type="range"].range::-webkit-slider-thumb:active {
		transform: scale(1.05);
	}

	input[type="range"].range::-moz-range-thumb {
		width: 20px;
		height: 20px;
		border-radius: 50%;
		background: #0075b4;
		border: 2px solid white;
		box-shadow: 0 1px 4px rgba(0, 0, 0, 0.2);
		cursor: pointer;
	}

	input[type="range"].range::-moz-range-track {
		height: 6px;
		border-radius: 3px;
		background: #e5e7eb;
	}
</style>
