<script lang="ts">
	import type { TimeSeriesStat } from "$stats/dtos"

	import { goto } from "$app/navigation"
	import { ArrowRight } from "@lucide/svelte"

	interface Props {
		data: TimeSeriesStat[]
	}

	let { data }: Props = $props()

	const items = $derived(
		data
			.map((s) => ({
				id: s.id,
				name: s.key,
				total: s.values.reduce((sum, v) => sum + v.value, 0),
			}))
			.sort((a, b) => b.total - a.total),
	)

	const max = $derived(items[0]?.total ?? 1)
</script>

{#if items.length === 0}
	<p class="py-8 text-center text-sm text-corp-gray">Sin datos para mostrar.</p>
{:else}
	<div class="space-y-1.5">
		{#each items as item (item.name)}
			{@const w = Math.max((item.total / max) * 100, 2)}
			<button
				class="group flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left transition-colors hover:bg-corp-blue/4 disabled:opacity-50"
				onclick={() => item.id && void goto(`/stats/department/${item.id}`)}
				disabled={!item.id}
				aria-label={item.id ? `Ver estadísticas de ${item.name}` : undefined}
			>
				<span
					class="w-44 shrink-0 text-sm font-medium text-[#1A1A1A] group-hover:text-corp-blue truncate"
				>
					{item.name}
				</span>

				<span class="flex min-w-0 flex-1 items-center gap-2">
					<span
						class="h-5 rounded-sm bg-corp-blue/80 transition-colors group-hover:bg-corp-blue"
						style="width:{w}%"
					></span>
					<span class="text-sm font-semibold text-[#1A1A1A] tabular-nums"
						>{item.total}</span
					>
				</span>

				{#if item.id}
					<span
						class="flex shrink-0 items-center gap-1 text-xs font-medium text-corp-gray opacity-0 transition-opacity group-hover:opacity-100"
					>
						Ver <ArrowRight class="size-3" />
					</span>
				{/if}
			</button>
		{/each}
	</div>
{/if}
