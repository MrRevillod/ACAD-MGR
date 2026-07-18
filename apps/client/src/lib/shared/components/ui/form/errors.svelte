<script lang="ts">
	import Expandable from "./expandable.svelte"

	interface Props {
		name?: string
		errors: [string, ...string[]] | null
	}

	let { name, errors }: Props = $props()

	// Use frozen error signal
	let frozenErrors: [string, ...string[]] | null = $state(null)

	// Freeze error while element collapses to prevent UI from jumping
	$effect(() => {
		if (errors) {
			frozenErrors = errors
		} else {
			const timeout = setTimeout(() => {
				frozenErrors = null
			}, 200)

			return () => clearTimeout(timeout)
		}
	})
</script>

<Expandable expanded={!!errors}>
	<div id={`${name}-error`} class="mt-1 text-xs text-red-600">
		{frozenErrors?.[0]}
	</div>
</Expandable>
