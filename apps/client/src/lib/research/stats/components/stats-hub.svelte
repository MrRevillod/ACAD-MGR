<script lang="ts">
	import type { WorksStatsResponse } from "$stats/dtos"

	import { ArrowRight, Building2, ChartBar, Layers } from "@lucide/svelte"

	import { withScopeColors } from "./scope-colors"
	import BarsList from "./bars-list.svelte"
	import IndexationToggle from "./indexation-toggle.svelte"
	import MultiTrend from "./multi-trend.svelte"
	import StatsSection from "./stats-section.svelte"
	import TrendLine from "./trend-line.svelte"

	interface Props {
		data: WorksStatsResponse
	}

	let { data }: Props = $props()

	let openSection = $state<"faculty" | "departments" | "lines">("faculty")
	let deptKind = $state<"wos" | "scopus">("wos")
	let lineKind = $state<"wos" | "scopus">("wos")

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
</div>
