<script lang="ts">
	import type { Snippet } from "svelte"

	import { goto } from "$app/navigation"
	import { authStore } from "$lib/auth/store.svelte"

	let { children }: { children: Snippet } = $props()

	$effect(() => {
		if (!authStore.isAuthenticated()) {
			void goto("/login")
		}
	})
</script>

{@render children()}
