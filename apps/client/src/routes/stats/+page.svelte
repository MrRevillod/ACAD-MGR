<script lang="ts">
	import { Loader2, AlertCircle, BarChart3, TrendingUp } from "@lucide/svelte"

	import { useWorksStatsQuery } from "$lib/research/stats/queries"
	import type { StatsQuery } from "$lib/research/stats/types"
	import DepartmentList from "$lib/research/stats/components/department-bars.svelte"
	import TrendLine from "$lib/research/stats/components/trend-line.svelte"

	const currentYear = new Date().getFullYear()
	const queryParams: StatsQuery = { year_from: currentYear - 5, year_to: currentYear }

	const statsQuery = useWorksStatsQuery(() => queryParams)

	const summaryTotal = $derived(
		(statsQuery.data?.byJournalKind ?? []).reduce(
			(a, s) => a + s.values.reduce((b, v) => b + v.value, 0),
			0,
		),
	)
	const summaryScopus = $derived(
		(statsQuery.data?.byJournalKind.find((s) => s.key === "scopus")?.values ?? []).reduce(
			(a, v) => a + v.value,
			0,
		),
	)
	const summaryWos = $derived(
		(statsQuery.data?.byJournalKind.find((s) => s.key === "wos")?.values ?? []).reduce(
			(a, v) => a + v.value,
			0,
		),
	)
	const summaryTeaching = $derived(
		(statsQuery.data?.byOption.find((s) => s.key === "teaching")?.values ?? []).reduce(
			(a, v) => a + v.value,
			0,
		),
	)
	const summaryResearch = $derived(
		(statsQuery.data?.byOption.find((s) => s.key === "research")?.values ?? []).reduce(
			(a, v) => a + v.value,
			0,
		),
	)
</script>

<div class="mx-auto flex h-full max-w-[1600px] flex-col overflow-y-auto px-4 py-8 sm:px-6 lg:px-8">
	{#if statsQuery.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader2 class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if statsQuery.isError}
		<div class="flex flex-col items-center justify-center py-16 text-center">
			<AlertCircle class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Error al cargar las estadísticas.</p>
		</div>
	{:else}
		<h1 class="mb-1 text-xl font-semibold text-[#1A1A1A]">Estadísticas de Publicaciones</h1>
		<p class="mb-6 text-sm text-corp-gray">Facultad de Ingeniería</p>

		<div class="mb-6 grid grid-cols-5 gap-3">
			<div class="rounded-lg border border-corp-gray/20 bg-white p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Total</p>
				<p class="mt-1 text-2xl font-bold text-[#1A1A1A]">{summaryTotal}</p>
			</div>
			<div class="rounded-lg border border-corp-yellow/30 bg-corp-yellow/5 p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Scopus</p>
				<p class="mt-1 text-2xl font-bold text-corp-yellow">{summaryScopus}</p>
			</div>
			<div class="rounded-lg border border-corp-blue/30 bg-corp-blue/5 p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">WoS</p>
				<p class="mt-1 text-2xl font-bold text-corp-blue">{summaryWos}</p>
			</div>
			<div class="rounded-lg border border-corp-gray/20 bg-white p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Docencia</p>
				<p class="mt-1 text-2xl font-bold text-[#1A1A1A]">{summaryTeaching}</p>
			</div>
			<div class="rounded-lg border border-corp-gray/20 bg-white p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">
					Investigación
				</p>
				<p class="mt-1 text-2xl font-bold text-[#1A1A1A]">{summaryResearch}</p>
			</div>
		</div>

		<div class="space-y-6">
			<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
				<div class="mb-4 flex items-center gap-2">
					<BarChart3 class="size-4 text-corp-blue" />
					<h2 class="text-sm font-semibold tracking-wide uppercase text-corp-blue">
						Publicaciones por Departamento
					</h2>
				</div>
				<DepartmentList data={statsQuery.data?.byDepartment ?? []} />
			</section>

			<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
				<div class="mb-4 flex items-center gap-2">
					<TrendingUp class="size-4 text-corp-blue" />
					<h2 class="text-sm font-semibold tracking-wide uppercase text-corp-blue">
						Tendencia Anual Scopus / WoS
					</h2>
				</div>
				<TrendLine journalKind={statsQuery.data?.byJournalKind ?? []} />
			</section>
		</div>
	{/if}
</div>
