<script lang="ts">
	import type { Snippet } from "svelte"

	import { goto } from "$app/navigation"
	import { authStore } from "$auth/store.svelte"

	let { children }: { children: Snippet } = $props()

	$effect.pre(() => {
		if (!authStore.isAuthenticated) {
			void goto("/login")
		}
	})
</script>

{#if authStore.isAuthenticated}
	{@render children()}
{/if}
