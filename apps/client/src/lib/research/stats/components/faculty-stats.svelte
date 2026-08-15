<script lang="ts">
	import type { WorksStatsResponse } from "$stats/dtos"

	import { PieChart } from "@lucide/svelte"

	import DonutChart from "./donut-chart.svelte"
	import TopPublishersTable from "./top-publishers-table.svelte"
	import TrendLine from "./trend-line.svelte"

	interface Props {
		data: WorksStatsResponse
	}

	let { data }: Props = $props()

	const unindexed = $derived(
		data.facultySummary.totalWorks -
			data.facultySummary.wosCount -
			data.facultySummary.scopusCount,
	)

	const indexSegments = $derived([
		{ label: "WoS", value: data.facultySummary.wosCount, color: "#0075B4" },
		{ label: "Scopus", value: data.facultySummary.scopusCount, color: "#C9A500" },
		...(unindexed > 0 ? [{ label: "Sin indexar", value: unindexed, color: "#E5E7EB" }] : []),
	])
</script>

<div class="space-y-4">
	<div class="grid grid-cols-2 gap-3 sm:grid-cols-4">
		<div class="rounded-xl border border-corp-gray/20 bg-white p-5">
			<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">
				Total publicaciones
			</p>
			<p class="mt-2 text-[28px] font-semibold leading-none text-corp-ink tabular-nums">
				{data.facultySummary.totalWorks}
			</p>
		</div>
		<div class="rounded-xl border border-corp-blue/30 bg-corp-blue/5 p-5">
			<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">WoS</p>
			<p class="mt-2 text-[28px] font-semibold leading-none text-corp-blue tabular-nums">
				{data.facultySummary.wosCount}
			</p>
		</div>
		<div class="rounded-xl border border-corp-yellow/30 bg-corp-yellow/5 p-5">
			<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">Scopus</p>
			<p class="mt-2 text-[28px] font-semibold leading-none text-corp-gold tabular-nums">
				{data.facultySummary.scopusCount}
			</p>
		</div>
		<div class="rounded-xl border border-corp-gray/20 bg-white p-5">
			<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">
				Sin indexar
			</p>
			<p class="mt-2 text-[28px] font-semibold leading-none text-corp-gray tabular-nums">
				{unindexed}
			</p>
		</div>
	</div>

	<div class="rounded-xl border border-corp-gray/20 bg-white p-6">
		<h2 class="text-sm font-semibold tracking-wide uppercase text-corp-blue">
			Tendencia anual de publicaciones
		</h2>
		<p class="mt-1 text-sm text-corp-gray">
			Evolución anual de publicaciones de la Facultad de Ingeniería, según tipo de indexación.
		</p>
		<div class="mt-4">
			<TrendLine journalKind={data.byJournalKind} />
		</div>
	</div>

	<div class="rounded-xl border border-corp-gray/20 bg-white">
		<div class="flex items-center gap-2 border-b border-corp-gray/10 px-5 py-4">
			<PieChart class="size-4 shrink-0 text-corp-blue" />
			<h2 class="text-sm font-semibold tracking-wide uppercase text-corp-blue">
				Ranking y distribución de publicaciones
			</h2>
		</div>
		<div class="p-6">
			<p class="mb-5 text-sm text-corp-gray">
				Top 20 publicadores del periodo, ordenados por total de publicaciones. A la
				izquierda, la distribución por tipo de indexación.
			</p>
			<div class="grid grid-cols-1 gap-8 lg:grid-cols-[320px_1fr]">
				<div class="flex justify-center">
					<DonutChart
						segments={indexSegments}
						total={data.facultySummary.totalWorks}
						class="mt-4"
					/>
				</div>
				<div>
					<TopPublishersTable publishers={data.topPublishers} />
				</div>
			</div>
		</div>
	</div>
</div>
