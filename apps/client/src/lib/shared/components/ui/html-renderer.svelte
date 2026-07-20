<script lang="ts">
	import DOMPurify from "dompurify"

	let {
		html = "",
		tag = "div",
		class: className = "",
	}: { html: string; tag?: string; class?: string } = $props()

	const sanitized = $derived(
		DOMPurify.sanitize(html, {
			ALLOWED_TAGS: ["i", "b", "em", "strong", "sub", "sup", "br"],
			ALLOWED_ATTR: [],
		}),
	)
</script>

{#if tag === "p"}
	<p class={className}>{@html sanitized}</p>
{:else if tag === "span"}
	<span class={className}>{@html sanitized}</span>
{:else if tag === "h1"}
	<h1 class={className}>{@html sanitized}</h1>
{:else if tag === "h2"}
	<h2 class={className}>{@html sanitized}</h2>
{:else if tag === "h3"}
	<h3 class={className}>{@html sanitized}</h3>
{:else if tag === "h4"}
	<h4 class={className}>{@html sanitized}</h4>
{:else}
	<div class={className}>{@html sanitized}</div>
{/if}
