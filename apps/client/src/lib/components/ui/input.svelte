<script lang="ts">
	import type { HTMLInputAttributes } from "svelte/elements"

	type $$Props = HTMLInputAttributes & {
		value?: string | number | null
		class?: string
	}

	let { value = $bindable(""), class: className = "", ...restProps }: $$Props = $props()
</script>

<input
	bind:value
	class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10 disabled:cursor-not-allowed disabled:opacity-50 {className}"
	{...restProps}
/>
