<script lang="ts">
	import type { Snippet } from "svelte"

	import { Toaster } from "svelte-sonner"
	import { queryClient } from "$shared/http/tanstack"
	import { QueryClientProvider } from "@tanstack/svelte-query"

	import Header from "$shared/components/Header.svelte"

	import "./layout.css"

	let { children }: { children: Snippet } = $props()
</script>

<svelte:head>
	<title>ACAD MGR</title>
</svelte:head>

<QueryClientProvider client={queryClient}>
	<Header />
	<main class="h-[calc(100dvh-3.5rem)] overflow-hidden bg-neutral-50">
		{@render children()}
	</main>
	<Toaster richColors position="top-right" />
</QueryClientProvider>
