<script lang="ts">
	import type { ResearchLineStatsResponse } from "$stats/dtos"

	import { ChartBar, Trophy } from "@lucide/svelte"

	import BarsList from "./bars-list.svelte"
	import StatsSection from "./stats-section.svelte"
	import TopPublishersTable from "./top-publishers-table.svelte"
	import TrendLine from "./trend-line.svelte"

	interface Props {
		data: ResearchLineStatsResponse
	}

	let { data }: Props = $props()

	let openSection = $state<"trend" | "ranking">("trend")

	const unindexed = $derived(data.totalWorks - data.wosCount - data.scopusCount)
</script>

<div class="space-y-4">
	<div class="grid grid-cols-2 gap-3 sm:grid-cols-4">
		<div class="rounded-xl border border-corp-gray/20 bg-white p-5">
			<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">
				Total publicaciones
			</p>
			<p class="mt-2 text-[28px] font-semibold leading-none text-corp-ink tabular-nums">
				{data.totalWorks}
			</p>
		</div>
		<div class="rounded-xl border border-corp-blue/30 bg-corp-blue/5 p-5">
			<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">WoS</p>
			<p class="mt-2 text-[28px] font-semibold leading-none text-corp-blue tabular-nums">
				{data.wosCount}
			</p>
		</div>
		<div class="rounded-xl border border-corp-yellow/30 bg-corp-yellow/5 p-5">
			<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">Scopus</p>
			<p class="mt-2 text-[28px] font-semibold leading-none text-corp-gold tabular-nums">
				{data.scopusCount}
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

	<div class="overflow-hidden rounded-xl border border-corp-gray/20 bg-white">
		<StatsSection
			title="Tendencia anual de publicaciones"
			icon={ChartBar}
			open={openSection === "trend"}
			ontoggle={() => (openSection = "trend")}
			first
			description="Evolución anual de las publicaciones de la línea, según tipo de indexación."
		>
			<TrendLine journalKind={data.byJournalKind} />
		</StatsSection>

		<StatsSection
			title="Ranking de publicadores"
			icon={Trophy}
			open={openSection === "ranking"}
			ontoggle={() => (openSection = "ranking")}
			description="Top 20 publicadores del periodo, ordenados por total de publicaciones."
		>
			<div class="grid grid-cols-1 items-start gap-8 lg:grid-cols-[320px_1fr]">
				<div class="w-full">
					<h2 class="mb-4 text-sm font-semibold tracking-wide uppercase text-corp-blue">
						Por departamento
					</h2>
					<BarsList data={data.byDepartment} />
				</div>
				<div>
					<TopPublishersTable publishers={data.topPublishers} />
				</div>
			</div>
		</StatsSection>
	</div>
</div>
