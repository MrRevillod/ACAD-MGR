<script lang="ts">
	import type { DepartmentDetail } from "$stats/dtos"

	import { ChartBar, Trophy } from "@lucide/svelte"

	import StatsSection from "./stats-section.svelte"
	import TopPublishersTable from "./top-publishers-table.svelte"
	import TrendLine from "./trend-line.svelte"

	interface Props {
		data: DepartmentDetail
	}

	let { data }: Props = $props()

	let openSection = $state<"trend" | "ranking">("trend")

	const unindexed = $derived(data.totalWorks - data.scopusCount - data.wosCount)
</script>

<div class="space-y-4">
	<div class="grid grid-cols-2 gap-3 sm:grid-cols-6">
		<div class="rounded-xl border border-corp-gray/20 bg-white p-5">
			<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">Total</p>
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
		<div class="rounded-xl border border-corp-gray/20 bg-white p-5">
			<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">Docencia</p>
			<p class="mt-2 text-[28px] font-semibold leading-none text-corp-ink tabular-nums">
				{data.teachingCount}
			</p>
		</div>
		<div class="rounded-xl border border-corp-gray/20 bg-white p-5">
			<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">
				Investigación
			</p>
			<p class="mt-2 text-[28px] font-semibold leading-none text-corp-ink tabular-nums">
				{data.researchCount}
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
			description="Evolución anual de las publicaciones del departamento, según tipo de indexación."
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
			<TopPublishersTable publishers={data.topPublishers} />
		</StatsSection>
	</div>
</div>
