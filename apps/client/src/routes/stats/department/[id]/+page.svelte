<script lang="ts">
	import { page } from "$app/state"
	import { Loader2, AlertCircle } from "@lucide/svelte"
	import { useSearchParams } from "runed/kit"
	import * as v from "valibot"

	import { useDepartmentDetailQuery } from "$lib/research/stats/queries"
	import type { DepartmentDetailQuery } from "$lib/research/stats/types"
	import DonutChart from "$lib/research/stats/components/donut-chart.svelte"
	import PublishersTable from "$lib/research/stats/components/publishers-table.svelte"

	const deptId = $derived(page.params.id ?? "")
	const currentYear = new Date().getFullYear()
	const defaultYearFrom = String(currentYear - 5)
	const defaultYearTo = String(currentYear)

	const searchParamsSchema = v.object({
		year_from: v.optional(v.fallback(v.string(), defaultYearFrom), defaultYearFrom),
		year_to: v.optional(v.fallback(v.string(), defaultYearTo), defaultYearTo),
		option: v.optional(v.fallback(v.string(), ""), ""),
		journal_kind: v.optional(v.fallback(v.string(), ""), ""),
	})

	const params = useSearchParams(searchParamsSchema, {
		debounce: 300,
		pushHistory: false,
	})

	const queryParams = $derived<DepartmentDetailQuery>({
		...(params.year_from && { year_from: Number(params.year_from) }),
		...(params.year_to && { year_to: Number(params.year_to) }),
		...(params.option && {
			option: params.option as "teaching" | "research",
		}),
		...(params.journal_kind && {
			journal_kind: params.journal_kind as "wos" | "scopus",
		}),
	})

	const detailQuery = useDepartmentDetailQuery(
		() => deptId,
		() => queryParams,
	)

	const indexSegments = $derived.by(() => {
		const d = detailQuery.data
		if (!d) return []
		const unindexed = d.totalWorks - d.scopusCount - d.wosCount
		return [
			{ label: "Scopus", value: d.scopusCount, color: "#EDC500" },
			{ label: "WoS", value: d.wosCount, color: "#0075B4" },
			...(unindexed > 0
				? [{ label: "Sin indexar", value: unindexed, color: "#E5E7EB" }]
				: []),
		]
	})
</script>

<div class="mx-auto flex h-full max-w-[1600px] flex-col overflow-y-auto px-4 py-8 sm:px-6 lg:px-8">
	{#if detailQuery.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader2 class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if detailQuery.isError || !detailQuery.data}
		<div class="flex flex-col items-center justify-center py-16 text-center">
			<AlertCircle class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Error al cargar los datos del departamento.</p>
		</div>
	{:else}
		{@const d = detailQuery.data}

		<div
			class="mb-6 flex flex-wrap items-center gap-4 rounded-xl border border-corp-gray/20 bg-white p-4"
		>
			<h1 class="text-lg font-semibold text-[#1A1A1A]">Departamento de {d.department}</h1>

			<div class="ml-auto flex flex-wrap items-center gap-3">
				<div class="flex items-center gap-2">
					<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
						>Año desde</span
					>
					<input
						type="number"
						bind:value={params.year_from}
						min="1900"
						max="2100"
						class="h-8 w-20 rounded-lg border border-corp-gray/20 bg-white px-2 text-sm outline-none focus:border-corp-blue/50"
					/>
				</div>

				<div class="flex items-center gap-2">
					<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
						>Año hasta</span
					>
					<input
						type="number"
						bind:value={params.year_to}
						min="1900"
						max="2100"
						class="h-8 w-20 rounded-lg border border-corp-gray/20 bg-white px-2 text-sm outline-none focus:border-corp-blue/50"
					/>
				</div>

				<div class="flex items-center gap-2">
					<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
						>Opción Académica</span
					>

					<select
						bind:value={params.option}
						class="h-8 rounded-lg border border-corp-gray/20 bg-white px-2 text-sm outline-none focus:border-corp-blue/50"
					>
						<option value="">Todas</option>
						<option value="teaching">Docencia</option>
						<option value="research">Investigación</option>
					</select>
				</div>

				<div class="flex items-center gap-2">
					<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
						>Indexación</span
					>

					<select
						bind:value={params.journal_kind}
						class="h-8 rounded-lg border border-corp-gray/20 bg-white px-2 text-sm outline-none focus:border-corp-blue/50"
					>
						<option value="">Todas</option>
						<option value="wos">WoS</option>
						<option value="scopus">Scopus</option>
					</select>
				</div>

				<button
					class="rounded-lg px-2 py-1 text-xs font-medium text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-corp-blue"
					onclick={() => params.reset()}
				>
					Limpiar
				</button>
			</div>
		</div>

		<div class="mb-6 grid grid-cols-5 gap-3">
			<div class="rounded-lg border border-corp-gray/20 bg-white p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Total</p>
				<p class="mt-1 text-2xl font-bold text-[#1A1A1A]">{d.totalWorks}</p>
			</div>
			<div class="rounded-lg border border-corp-yellow/30 bg-corp-yellow/5 p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Scopus</p>
				<p class="mt-1 text-2xl font-bold text-corp-yellow">{d.scopusCount}</p>
			</div>
			<div class="rounded-lg border border-corp-blue/30 bg-corp-blue/5 p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">WoS</p>
				<p class="mt-1 text-2xl font-bold text-corp-blue">{d.wosCount}</p>
			</div>
			<div class="rounded-lg border border-corp-gray/20 bg-white p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Docencia</p>
				<p class="mt-1 text-2xl font-bold text-[#1A1A1A]">{d.teachingCount}</p>
			</div>
			<div class="rounded-lg border border-corp-gray/20 bg-white p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">
					Investigación
				</p>
				<p class="mt-1 text-2xl font-bold text-[#1A1A1A]">{d.researchCount}</p>
			</div>
		</div>

		<div class="grid grid-cols-1 gap-6 lg:grid-cols-[280px_1fr]">
			<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
				<h2 class="mb-4 text-sm font-semibold tracking-wide uppercase text-corp-blue">
					Indexación
				</h2>
				<DonutChart segments={indexSegments} total={d.totalWorks} />
			</section>

			<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
				<h2 class="mb-4 text-sm font-semibold tracking-wide uppercase text-corp-blue">
					Top Publicadores
				</h2>
				<PublishersTable publishers={d.topPublishers} />
			</section>
		</div>
	{/if}
</div>
