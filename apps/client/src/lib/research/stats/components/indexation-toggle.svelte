<script lang="ts">
	interface Props {
		kind: "wos" | "scopus"
	}

	let { kind = $bindable("wos") }: Props = $props()
</script>

<div class="flex shrink-0 rounded-lg bg-corp-gray/10 p-1">
	<button
		type="button"
		class="rounded-md px-3 py-1 text-xs font-semibold transition-[color,scale] duration-150 ease-out active:scale-[0.96] {kind ===
		'wos'
			? 'bg-white text-corp-blue shadow-sm'
			: 'text-corp-gray hover:text-corp-ink'}"
		onclick={() => (kind = "wos")}
	>
		WoS
	</button>
	<button
		type="button"
		class="rounded-md px-3 py-1 text-xs font-semibold transition-[color,scale] duration-150 ease-out active:scale-[0.96] {kind ===
		'scopus'
			? 'bg-white text-corp-gold shadow-sm'
			: 'text-corp-gray hover:text-corp-ink'}"
		onclick={() => (kind = "scopus")}
	>
		Scopus
	</button>
</div>
