<script lang="ts">
	import { CircleAlert, Loader } from "@lucide/svelte"
	import { LineChart } from "layerchart"

	import Select from "$shared/components/ui/select.svelte"
	import { useConfig } from "$shared/config/queries"

	import { useProductivityQuery } from "../queries"

	import type { ProductivityDegree, ProductivityScope } from "../dtos"

	interface Props {
		title: string
		denominator: string
		degree: ProductivityDegree
		scope: ProductivityScope
		departmentId?: string
		researchLineId?: string
		yearFrom: number
		yearTo: number
	}

	let {
		title,
		denominator,
		degree,
		scope,
		departmentId,
		researchLineId,
		yearFrom,
		yearTo,
	}: Props = $props()

	const monthNames = [
		"Enero",
		"Febrero",
		"Marzo",
		"Abril",
		"Mayo",
		"Junio",
		"Julio",
		"Agosto",
		"Septiembre",
		"Octubre",
		"Noviembre",
		"Diciembre",
	]

	const monthItems = $derived(monthNames.map((label, i) => ({ value: String(i + 1), label })))

	const degreeItems = [
		{ value: "all", label: "Total" },
		{ value: "doctor", label: "Doctores" },
		{ value: "magister", label: "Magísteres" },
	]

	const degreePhrases: Record<ProductivityDegree, string> = {
		all: "Publicaciones",
		doctor: "Publicaciones de autores con grado de doctor",
		magister: "Publicaciones de autores con grado de magíster",
	}

	let month = $state("1")
	let selectedDegree = $state<string>(degree)

	const description = $derived(
		`${degreePhrases[selectedDegree as ProductivityDegree]} ÷ Σ JCE (Doctor) ${denominator}, por año.`,
	)

	const queryParams = $derived({
		degree: selectedDegree as ProductivityDegree,
		scope,
		...(departmentId ? { departmentId } : {}),
		...(researchLineId ? { researchLineId } : {}),
		month: Number(month),
		yearFrom,
		yearTo,
	})

	const productivity = useProductivityQuery(() => queryParams)
	const config = useConfig()

	const series = $derived([
		{ key: "total", color: "#1F2937", label: "Total" },
		{ key: "wos", color: "#0075B4", label: "WoS" },
		{ key: "scopus", color: "#C9A500", label: "Scopus" },
	])

	const allYears = $derived(
		[
			...new Set(
				(productivity.data?.trend ?? []).flatMap((s) => s.values.map((v) => v.year)),
			),
		].sort((a, b) => a - b),
	)

	const wideData = $derived(
		allYears.map((year) => {
			const row: Record<string, number> = { year }
			for (const s of productivity.data?.trend ?? []) {
				row[s.key] = s.values.find((v) => v.year === year)?.value ?? 0
			}
			return row
		}),
	)

	const maxVal = $derived(
		Math.max(0, ...wideData.flatMap((r) => series.map((s) => r[s.key] ?? 0))),
	)

	const yTicks = $derived.by(() => {
		const max = Math.max(maxVal, 1)
		const step = Math.max(0.5, Math.ceil((max / 5) * 2) / 2)
		const top = Math.ceil(max / step) * step
		const ticks: number[] = []
		for (let v = 0; v <= top; v += step) ticks.push(v)
		return ticks
	})

	const yMax = $derived(yTicks[yTicks.length - 1] ?? 1)
	const minYear = $derived(allYears[0] ?? 0)
	const maxYear = $derived(allYears[allYears.length - 1] ?? 0)

	const jornadas = $derived.by(() => {
		if (!productivity.data || !config.data) return null
		return productivity.data.jce / config.data.jceMax
	})
</script>

<div class="overflow-hidden rounded-xl border border-corp-gray/20 bg-white">
	<div class="flex flex-wrap items-start justify-between gap-4 px-5 py-4">
		<div class="min-w-0">
			<h2 class="truncate text-sm font-semibold tracking-wide uppercase text-corp-blue">
				{title}
			</h2>
			{#if description}
				<p class="mt-0.5 block truncate text-xs font-normal normal-case text-corp-gray">
					{description}
				</p>
			{/if}
		</div>
		<div class="flex items-end gap-3">
			<Select
				items={degreeItems}
				bind:value={selectedDegree}
				placeholder="Grado"
				class="min-w-36"
			/>
			<Select items={monthItems} bind:value={month} placeholder="Mes" class="min-w-36" />
		</div>
	</div>

	<div class="border-t border-corp-gray/10 p-6">
		{#if productivity.isPending}
			<div class="flex items-center justify-center py-16">
				<Loader class="size-6 animate-spin text-corp-gray" />
			</div>
		{:else if productivity.isError || !productivity.data}
			<div class="flex flex-col items-center justify-center py-16 text-center">
				<CircleAlert class="size-8 text-red-500" />
				<p class="mt-3 text-sm text-corp-gray">Error al cargar la productividad.</p>
			</div>
		{:else if allYears.length === 0}
			<p class="py-8 text-center text-sm text-corp-gray">Sin datos para mostrar.</p>
		{:else}
			<LineChart
				data={wideData}
				x="year"
				{series}
				height={280}
				padding={{ left: 50, right: 20, bottom: 40, top: 10 }}
				xDomain={[minYear, maxYear]}
				xNice={false}
				yDomain={[0, yMax]}
				yNice={false}
				legend={true}
				points={true}
				props={{
					xAxis: { ticks: allYears.length, format: (d: number) => String(d) },
					yAxis: { ticks: yTicks, format: (d: number) => String(d) },
				}}
			/>
			{#if jornadas}
				<p class="mt-3 text-xs text-corp-gray">
					Σ JCE (Doctor) del alcance: {productivity.data.jce} h · {jornadas.toFixed(1)} jornadas
					completas.
				</p>
			{/if}
		{/if}
	</div>
</div>
