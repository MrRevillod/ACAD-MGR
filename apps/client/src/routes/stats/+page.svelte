<script lang="ts">
	import * as v from "valibot"
	import type { StatsQuery } from "$stats/dtos"

	import { useSearchParams } from "runed/kit"
	import { useWorksStatsQuery } from "$stats/queries"
	import { Loader, CircleAlert, BarChart, TrendingUp, RotateCcw } from "@lucide/svelte"

	import YearRange from "$lib/shared/components/ui/year-range.svelte"
	import Button from "$lib/shared/components/ui/button.svelte"

	import TrendLine from "$stats/components/trend-line.svelte"
	import DepartmentList from "$stats/components/department-bars.svelte"

	const currentYear = new Date().getFullYear()
	const defaultYearFrom = String(currentYear - 5)
	const defaultYearTo = String(currentYear)

	const searchParamsSchema = v.object({
		year_from: v.optional(v.fallback(v.string(), defaultYearFrom), defaultYearFrom),
		year_to: v.optional(v.fallback(v.string(), defaultYearTo), defaultYearTo),
	})

	const params = useSearchParams(searchParamsSchema, {
		debounce: 300,
		pushHistory: false,
	})

	const queryParams = $derived<StatsQuery>({
		yearFrom: Number(params.year_from),
		yearTo: Number(params.year_to),
	})

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
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if statsQuery.isError}
		<div class="flex flex-col items-center justify-center py-16 text-center">
			<CircleAlert class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Error al cargar las estadísticas.</p>
		</div>
	{:else}
		<div class="mb-6 flex flex-wrap items-start justify-between gap-4">
			<div>
				<h1 class="text-xl font-semibold text-[#1A1A1A]">Estadísticas de Publicaciones</h1>
				<p class="mt-1 text-sm text-corp-gray">Facultad de Ingeniería</p>
			</div>
			<div class="flex items-end gap-3">
				<div class="space-y-2.5">
					<span class="block text-xs font-medium tracking-wide uppercase text-corp-gray"
						>Rango anual de publicación</span
					>
					<YearRange
						bind:yearFrom={params.year_from}
						bind:yearTo={params.year_to}
						labelFrom="Desde"
						labelTo="Hasta"
						showLabels={false}
					/>
				</div>
				<Button variant="secondary" onclick={() => params.reset()}>
					<RotateCcw class="size-3.5" />
					Limpiar
				</Button>
			</div>
		</div>

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

		<div class="grid grid-cols-1 gap-6 lg:grid-cols-[3fr_7fr]">
			<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
				<div class="mb-4 flex items-center gap-2">
					<BarChart class="size-4 text-corp-blue" />
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
