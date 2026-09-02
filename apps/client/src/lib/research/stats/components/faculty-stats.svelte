<script lang="ts">
	import type { WorksStatsResponse } from "$stats/dtos"

	import { ChartBar, Gauge, Info, Trophy } from "@lucide/svelte"

	import { buildProductivityDescription } from "../productivity-labels"
	import ProductivityHelpDialog from "./productivity-help-dialog.svelte"
	import ProductivityPanel from "./productivity-panel.svelte"
	import StatsSection from "./stats-section.svelte"
	import TopLimitSelect from "./top-limit-select.svelte"
	import TopPublishersTable from "./top-publishers-table.svelte"
	import TrendLine from "./trend-line.svelte"

	import type { ProductivityDegree, ProductivityJceScope } from "../dtos"
	import type { ProductivitySectionProps } from "../productivity-labels"

	interface Props {
		data: WorksStatsResponse
		productivity: ProductivitySectionProps
		limit?: string
	}

	let { data, productivity, limit = $bindable("10") }: Props = $props()

	let openSection = $state<"trend" | "ranking" | "productivity">("trend")
	function initialDegree() {
		return productivity.degree
	}

	let selectedDegree = $state<ProductivityDegree>(initialDegree())

	function initialJceScope() {
		return productivity.jceScope ?? "doctor"
	}

	let selectedJceScope = $state<ProductivityJceScope>(initialJceScope())

	const productivityDescription = $derived(
		buildProductivityDescription(selectedDegree, selectedJceScope, productivity.denominator),
	)

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
		class="flex size-10 shrink-0 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-corp-ink"
		title="Cómo se calcula este indicador"
		aria-label="Cómo se calcula este indicador"
		onclick={() => (showProductivityInfo = true)}
	>
		<Info class="size-5" />
	</button>
{/snippet}

{#snippet rankingAction()}
	<TopLimitSelect bind:value={limit} />
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
			action={rankingAction}
			description="Top {limit} publicadores del periodo, ordenados por total de publicaciones."
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
			<ProductivityPanel
				bind:degree={selectedDegree}
				bind:jceScope={selectedJceScope}
				scope={productivity.scope}
				departmentId={productivity.departmentId}
				researchLineId={productivity.researchLineId}
				yearFrom={productivity.yearFrom}
				yearTo={productivity.yearTo}
			/>
		</StatsSection>
	</div>
</div>

<ProductivityHelpDialog bind:open={showProductivityInfo} />
