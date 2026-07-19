<script lang="ts">
	import DOMPurify from "dompurify"

	let {
		html = "" as string,
		as = "div" as "div" | "p" | "span" | "h1" | "h2" | "h3" | "h4",
		class: className = "" as string,
	}: { html: string; as?: string; class?: string } = $props()

	const sanitized = $derived(
		DOMPurify.sanitize(html, {
			ALLOWED_TAGS: ["i", "b", "em", "strong", "sub", "sup", "br"],
			ALLOWED_ATTR: [],
		}),
	)
</script>

{#if as === "p"}
	<p class={className}>{@html sanitized}</p>
{:else if as === "span"}
	<span class={className}>{@html sanitized}</span>
{:else if as === "h1"}
	<h1 class={className}>{@html sanitized}</h1>
{:else if as === "h2"}
	<h2 class={className}>{@html sanitized}</h2>
{:else if as === "h3"}
	<h3 class={className}>{@html sanitized}</h3>
{:else if as === "h4"}
	<h4 class={className}>{@html sanitized}</h4>
{:else}
	<div class={className}>{@html sanitized}</div>
{/if}
