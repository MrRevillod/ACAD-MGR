<script lang="ts">
	import type { WorksStatsResponse } from "$stats/dtos"

	import { ArrowRight, Building2, ChartBar, Gauge, Info, Layers } from "@lucide/svelte"

	import { degreePhrases } from "../productivity-labels"
	import { withScopeColors } from "./scope-colors"
	import BarsList from "./bars-list.svelte"
	import IndexationToggle from "./indexation-toggle.svelte"
	import MultiTrend from "./multi-trend.svelte"
	import ProductivityChart from "./productivity-chart.svelte"
	import ProductivityFilters from "./productivity-filters.svelte"
	import ProductivityHelpDialog from "./productivity-help-dialog.svelte"
	import StatsSection from "./stats-section.svelte"
	import TrendLine from "./trend-line.svelte"

	import type { ProductivityDegree } from "../dtos"
	import type { ProductivitySectionProps } from "./productivity-chart.svelte"

	interface Props {
		data: WorksStatsResponse
		productivity: ProductivitySectionProps
	}

	let { data, productivity }: Props = $props()

	let openSection = $state<"faculty" | "departments" | "lines" | "productivity">("faculty")
	let deptKind = $state<"wos" | "scopus">("wos")
	let lineKind = $state<"wos" | "scopus">("wos")
	function initialDegree() {
		return productivity.degree
	}

	let selectedDegree = $state<string>(initialDegree())
	let month = $state("1")
	let indexation = $state<"all" | "wos" | "scopus">("all")

	const productivityDescription = $derived(
		`${degreePhrases[selectedDegree as ProductivityDegree]} ÷ Σ JCE (Doctor) ${productivity.denominator}, por año.`,
	)

	let showProductivityInfo = $state(false)

	const departments = $derived(withScopeColors(data.byDepartment))
	const lines = $derived(withScopeColors(data.byResearchLine))

	function deptHref(id: string) {
		return `/stats/department/${id}`
	}

	function lineHref(id: string) {
		return `/stats/research-line/${id}`
	}
</script>

{#snippet facultyAction()}
	<a
		href="/stats/faculty"
		class="inline-flex items-center gap-1 text-xs font-medium tracking-wide uppercase text-corp-blue transition-colors hover:text-corp-blue/80 hover:underline"
	>
		Ver detalle completo <ArrowRight class="size-3" />
	</a>
{/snippet}

{#snippet deptToggleAction()}
	<IndexationToggle bind:kind={deptKind} />
{/snippet}

{#snippet lineToggleAction()}
	<IndexationToggle bind:kind={lineKind} />
{/snippet}

{#snippet productivityInfoAction()}
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

<div class="overflow-hidden rounded-xl border border-corp-gray/20 bg-white">
	<StatsSection
		title="Métricas y Estadísticas de la Facultad"
		icon={ChartBar}
		open={openSection === "faculty"}
		ontoggle={() => (openSection = "faculty")}
		first
		action={facultyAction}
		description="Publicaciones de la Facultad de Ingeniería indexadas en WoS y Scopus, por año."
	>
		<TrendLine journalKind={data.byJournalKind} />
	</StatsSection>

	<StatsSection
		title="Departamentos"
		icon={Building2}
		open={openSection === "departments"}
		ontoggle={() => (openSection = "departments")}
		action={deptToggleAction}
		description="Total de publicaciones por departamento en el rango seleccionado. Usa el selector para comparar la tendencia entre WoS y Scopus."
	>
		<div class="grid grid-cols-1 items-start gap-8 lg:grid-cols-[320px_1fr]">
			<BarsList data={departments} hrefFor={deptHref} />
			<MultiTrend items={departments} bind:kind={deptKind} />
		</div>
	</StatsSection>

	<StatsSection
		title="Líneas de investigación"
		icon={Layers}
		open={openSection === "lines"}
		ontoggle={() => (openSection = "lines")}
		action={lineToggleAction}
		description="Total de publicaciones por línea de investigación en el rango seleccionado. Usa el selector para comparar la tendencia entre WoS y Scopus."
	>
		<div class="grid grid-cols-1 items-start gap-8 lg:grid-cols-[320px_1fr]">
			<BarsList data={lines} hrefFor={lineHref} />
			<MultiTrend items={lines} bind:kind={lineKind} />
		</div>
	</StatsSection>

	<StatsSection
		title="Productividad por jornada completa"
		icon={Gauge}
		open={openSection === "productivity"}
		ontoggle={() => (openSection = "productivity")}
		barAction={productivityInfoAction}
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

<ProductivityHelpDialog bind:open={showProductivityInfo} />
