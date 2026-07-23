<script lang="ts">
	import * as v from "valibot"

	import type { TableFeatures } from "@tanstack/svelte-table"
	import type { DepartmentDetailQuery, TopPublisher } from "$stats/dtos"

	import { page } from "$app/state"
	import { goto } from "$app/navigation"
	import { FullName } from "$shared/value-objects/full-name.value"
	import { authStore } from "$lib/auth/store.svelte"
	import { useSearchParams } from "runed/kit"
	import { useDepartmentDetailQuery } from "$stats/queries"
	import { Loader, CircleAlert, RotateCcw } from "@lucide/svelte"
	import { createColumnHelper, renderSnippet } from "@tanstack/svelte-table"

	import YearRange from "$shared/components/ui/year-range.svelte"
	import Select from "$shared/components/ui/select.svelte"
	import Badge from "$shared/components/ui/badge.svelte"
	import Button from "$shared/components/ui/button.svelte"
	import DataTable from "$shared/components/ui/data-table.svelte"
	import DonutChart from "$stats/components/donut-chart.svelte"

	const deptId = $derived(page.params.id ?? "")
	const currentYear = new Date().getFullYear()
	const defaultYearFrom = String(currentYear - 5)
	const defaultYearTo = String(currentYear)

	const searchParamsSchema = v.object({
		yearFrom: v.optional(v.fallback(v.string(), defaultYearFrom), defaultYearFrom),
		yearTo: v.optional(v.fallback(v.string(), defaultYearTo), defaultYearTo),
		option: v.optional(v.fallback(v.string(), ""), ""),
		journalKind: v.optional(v.fallback(v.string(), ""), ""),
	})

	const params = useSearchParams(searchParamsSchema, {
		debounce: 300,
		pushHistory: false,
	})

	const queryParams = $derived<DepartmentDetailQuery>({
		yearFrom: Number(params.yearFrom),
		yearTo: Number(params.yearTo),
		...(params.option && {
			option: params.option as "teaching" | "research",
		}),
		...(params.journalKind && {
			journalKind: params.journalKind as "wos" | "scopus",
		}),
	})

	const optionItems = [
		{ value: "", label: "Todas" },
		{ value: "teaching", label: "Docencia" },
		{ value: "research", label: "Investigación" },
	]

	const detailQuery = useDepartmentDetailQuery(
		() => deptId,
		() => queryParams,
	)

	const unindexedCount = $derived.by(() => {
		const d = detailQuery.data
		if (!d) return 0
		return d.totalWorks - d.scopusCount - d.wosCount
	})

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

	const optionLabel: Record<string, string> = {
		teaching: "Docencia",
		research: "Investigación",
	}

	const helper = createColumnHelper<TableFeatures, TopPublisher>()

	const columns = [
		helper.accessor((row) => FullName.fromFullString(row.name).toString(), {
			id: "name",
			header: "Nombre",
			cell: (info) => renderSnippet(nameSnippet, { name: info.getValue() }),
		}),
		helper.accessor("total", {
			id: "total",
			header: "Total",
			cell: (info) =>
				renderSnippet(numberSnippet, {
					value: info.getValue(),
					cls: "font-semibold text-[#1A1A1A]",
				}),
		}),
		helper.accessor("scopus", {
			id: "scopus",
			header: "Scopus",
			cell: (info) =>
				renderSnippet(numberSnippet, {
					value: info.getValue(),
					cls: "font-medium text-corp-yellow",
				}),
		}),
		helper.accessor("wos", {
			id: "wos",
			header: "WoS",
			cell: (info) =>
				renderSnippet(numberSnippet, {
					value: info.getValue(),
					cls: "font-medium text-corp-blue",
				}),
		}),
		helper.accessor("unindexed", {
			id: "unindexed",
			header: "Sin indexar",
			cell: (info) =>
				renderSnippet(numberSnippet, {
					value: info.getValue(),
					cls: "font-medium text-corp-gray",
				}),
		}),
		helper.accessor("option", {
			id: "option",
			header: "Opción",
			cell: (info) => renderSnippet(optionSnippet, { option: info.getValue() }),
		}),
	]
</script>

<div class="mx-auto flex h-full max-w-[1600px] flex-col overflow-y-auto px-4 py-8 sm:px-6 lg:px-8">
	{#if detailQuery.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if detailQuery.isError || !detailQuery.data}
		<div class="flex flex-col items-center justify-center py-16 text-center">
			<CircleAlert class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Error al cargar los datos del departamento.</p>
		</div>
	{:else}
		{@const d = detailQuery.data}

		<div class="mb-4 flex items-center justify-between gap-4">
			<div class="min-w-0">
				<h1 class="text-2xl font-semibold tracking-tight text-[#1A1A1A]">
					Departamento de {d.department}
				</h1>
				<p class="mt-1 text-sm text-corp-gray">Ranking de publicadores</p>
			</div>

			<div class="flex items-end gap-3">
				<div class="space-y-2.5">
					<span
						class="block text-xs font-medium tracking-wide uppercase text-corp-gray whitespace-nowrap"
						>Rango anual de publicación</span
					>
					<YearRange
						bind:yearFrom={params.yearFrom}
						bind:yearTo={params.yearTo}
						labelFrom="Desde"
						labelTo="Hasta"
						showLabels={false}
					/>
				</div>
				<Select
					items={optionItems}
					bind:value={params.option}
					placeholder="Opción Académica"
					class="min-w-48"
				/>

				<Button variant="primary" onclick={() => params.reset()}>
					<RotateCcw class="size-3.5" />
					Limpiar
				</Button>
			</div>
		</div>

		<div class="mb-4 grid grid-cols-6 gap-3">
			<div class="rounded-lg border border-corp-gray/20 bg-white p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Total</p>
				<p class="mt-1 text-xl font-bold text-[#1A1A1A]">{d.totalWorks}</p>
			</div>
			<div class="rounded-lg border border-corp-yellow/30 bg-corp-yellow/5 p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Scopus</p>
				<p class="mt-1 text-xl font-bold text-corp-yellow">{d.scopusCount}</p>
			</div>
			<div class="rounded-lg border border-corp-blue/30 bg-corp-blue/5 p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">WoS</p>
				<p class="mt-1 text-xl font-bold text-corp-blue">{d.wosCount}</p>
			</div>
			<div class="rounded-lg border border-corp-gray/20 bg-white p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">
					Sin indexar
				</p>
				<p class="mt-1 text-xl font-bold text-corp-gray">{unindexedCount}</p>
			</div>
			<div class="rounded-lg border border-corp-gray/20 bg-white p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Docencia</p>
				<p class="mt-1 text-xl font-bold text-[#1A1A1A]">{d.teachingCount}</p>
			</div>
			<div class="rounded-lg border border-corp-gray/20 bg-white p-4">
				<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">
					Investigación
				</p>
				<p class="mt-1 text-xl font-bold text-[#1A1A1A]">{d.researchCount}</p>
			</div>
		</div>

		<div class="grid grid-cols-1 gap-6 lg:grid-cols-[280px_1fr]">
			<section
				class="flex flex-col justify-start gap-8 rounded-xl border border-corp-gray/20 bg-white p-6"
			>
				<h2
					class="text-sm font-semibold tracking-wide uppercase text-corp-blue text-center"
				>
					Distribución Porcentual de tipos de indexación
				</h2>
				<DonutChart segments={indexSegments} total={d.totalWorks} class="mt-4" />
			</section>

			<section class="rounded-xl border border-corp-gray/20 bg-white p-4">
				<DataTable
					data={d.topPublishers}
					{columns}
					pageSize={20}
					onRowClick={(p: TopPublisher) => {
						const dest = authStore.isAuthenticated
							? `/academics/${p.academicId}`
							: `/public/academics/${p.academicId}`
						void goto(dest)
					}}
				/>
			</section>
		</div>
	{/if}
</div>

{#snippet nameSnippet({ name }: { name: string })}
	<span>{name}</span>
{/snippet}

{#snippet numberSnippet({ value, cls }: { value: number; cls: string })}
	<span class="text-right tabular-nums {cls}">{value}</span>
{/snippet}

{#snippet optionSnippet({ option }: { option: string })}
	<Badge variant={option === "research" ? "advanced" : "base"}>
		{optionLabel[option] ?? option}
	</Badge>
{/snippet}
