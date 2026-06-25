<script lang="ts">
	import type { Snippet } from "svelte"

	import { Toaster } from "svelte-sonner"
	import { queryClient } from "$lib/query/client"
	import { QueryClientProvider } from "@tanstack/svelte-query"

	import "./layout.css"

	let { children }: { children: Snippet } = $props()
</script>

<svelte:head>
	<title>ACAD MGR</title>
</svelte:head>

<QueryClientProvider client={queryClient}>
	<Toaster richColors position="top-right" />
	{@render children()}
</QueryClientProvider>
