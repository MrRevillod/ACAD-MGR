<script lang="ts">
	import type { Snippet } from "svelte"

	let { children }: { children: Snippet } = $props()
</script>

<div
	class="mx-auto flex h-full max-w-[1600px] flex-col overflow-y-auto overscroll-contain px-4 py-8 sm:px-6 lg:px-8"
>
	{@render children()}
</div>
