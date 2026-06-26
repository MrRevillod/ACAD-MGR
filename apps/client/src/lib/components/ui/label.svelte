<script lang="ts">
	import type { Snippet } from "svelte"

	let {
		class: className = "",
		children,
		...restProps
	}: {
		class?: string
		children: Snippet
	} = $props()
</script>

<label
	class="text-xs font-medium tracking-wide uppercase text-corp-gray {className}"
	{...restProps}
>
	{@render children?.()}
</label>
