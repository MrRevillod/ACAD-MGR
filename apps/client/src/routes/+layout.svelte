<script lang="ts">
	import type { Snippet } from "svelte"

	import { Toaster } from "svelte-sonner"
	import { queryClient } from "$lib/shared/http/query-client"
	import { QueryClientProvider } from "@tanstack/svelte-query"
	import Header from "$lib/components/Header.svelte"

	import "./layout.css"

	let { children }: { children: Snippet } = $props()
</script>

<svelte:head>
	<title>ACAD MGR</title>
</svelte:head>

<QueryClientProvider client={queryClient}>
	<Header />
	<main class="min-h-[calc(100dvh-3.5rem)]">
		{@render children()}
	</main>
	<Toaster richColors position="top-right" />
</QueryClientProvider>
