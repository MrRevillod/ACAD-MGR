<script lang="ts">
	import type { WorksStatsResponse } from "$stats/dtos"

	import { ChartBar, Gauge, Info, Trophy } from "@lucide/svelte"

	import { degreePhrases } from "../productivity-labels"
	import ProductivityChart from "./productivity-chart.svelte"
	import ProductivityFilters from "./productivity-filters.svelte"
	import ProductivityHelpDialog from "./productivity-help-dialog.svelte"
	import StatsSection from "./stats-section.svelte"
	import TopPublishersTable from "./top-publishers-table.svelte"
	import TrendLine from "./trend-line.svelte"

	import type { ProductivityDegree } from "../dtos"
	import type { ProductivitySectionProps } from "./productivity-chart.svelte"

	interface Props {
		data: WorksStatsResponse
		productivity: ProductivitySectionProps
	}

	let { data, productivity }: Props = $props()

	let openSection = $state<"trend" | "ranking" | "productivity">("trend")
	function initialDegree() {
		return productivity.degree
	}

	let selectedDegree = $state<string>(initialDegree())
	let month = $state("1")

	const productivityDescription = $derived(
		`${degreePhrases[selectedDegree as ProductivityDegree]} ÷ Σ JCE (Doctor) ${productivity.denominator}, por año.`,
	)

	let indexation = $state<"all" | "wos" | "scopus">("all")

	let showProductivityInfo = $state(false)

	const unindexed = $derived(
		data.facultySummary.totalWorks -
			data.facultySummary.wosCount -
			data.facultySummary.scopusCount,
	)
</script>

{#snippet facultyInfoAction()}
	<button
		type="button"
		class="flex size-9 shrink-0 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-corp-ink"
		title="Cómo se calcula este indicador"
		aria-label="Cómo se calcula este indicador"
		onclick={() => (showProductivityInfo = true)}
	>
		<Info class="size-5" />
	</button>
{/snippet}

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

	<div class="overflow-hidden rounded-xl border border-corp-gray/20 bg-white">
		<StatsSection
			title="Tendencia anual de publicaciones"
			icon={ChartBar}
			open={openSection === "trend"}
			ontoggle={() => (openSection = "trend")}
			first
			description="Evolución anual de publicaciones de la Facultad de Ingeniería, según tipo de indexación."
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

		<StatsSection
			title="Productividad por jornada completa"
			icon={Gauge}
			open={openSection === "productivity"}
			ontoggle={() => (openSection = "productivity")}
			barAction={facultyInfoAction}
			description={productivityDescription}
		>
			<div class="grid grid-cols-1 items-start gap-8 lg:grid-cols-[20%_1fr]">
				<ProductivityFilters bind:degree={selectedDegree} bind:month bind:indexation />
				<ProductivityChart
					degree={selectedDegree as ProductivityDegree}
					scope={productivity.scope}
					departmentId={productivity.departmentId}
					researchLineId={productivity.researchLineId}
					month={Number(month)}
					yearFrom={productivity.yearFrom}
					yearTo={productivity.yearTo}
					{indexation}
				/>
			</div>
		</StatsSection>
	</div>
</div>

<ProductivityHelpDialog bind:open={showProductivityInfo} />
