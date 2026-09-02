<script lang="ts">
	import Select from "$shared/components/ui/select.svelte"

	const items = [
		{ value: "5", label: "Top 5" },
		{ value: "10", label: "Top 10" },
		{ value: "15", label: "Top 15" },
		{ value: "20", label: "Top 20" },
		{ value: "50", label: "Top 50" },
	]

	let {
		value = $bindable("10"),
		class: className = "",
	}: {
		value?: string
		class?: string
	} = $props()
</script>

<Select {items} bind:value placeholder="Top" class={className} />
