<script lang="ts">
	import { CircleAlert, Loader } from "@lucide/svelte"
	import { LineChart } from "layerchart"

	import { useProductivityQuery } from "../queries"

	import type { ProductivityIndexation } from "../productivity-labels"
	import type { ProductivityDegree, ProductivityScope } from "../dtos"

	export interface ProductivitySectionProps {
		denominator: string
		degree: ProductivityDegree
		scope: ProductivityScope
		departmentId?: string
		researchLineId?: string
		yearFrom: number
		yearTo: number
	}

	export interface ProductivityChartProps {
		degree: ProductivityDegree
		scope: ProductivityScope
		departmentId?: string
		researchLineId?: string
		month: number
		yearFrom: number
		yearTo: number
		indexation: ProductivityIndexation
	}

	let {
		degree,
		scope,
		departmentId,
		researchLineId,
		month,
		yearFrom,
		yearTo,
		indexation,
	}: ProductivityChartProps = $props()

	const queryParams = $derived({
		degree,
		scope,
		...(departmentId ? { departmentId } : {}),
		...(researchLineId ? { researchLineId } : {}),
		month,
		yearFrom,
		yearTo,
	})

	const productivity = useProductivityQuery(() => queryParams)

	const series = $derived.by(() => {
		const key = { all: "total", wos: "wos", scopus: "scopus" }[indexation]
		return [
			{
				key: "total",
				color: "#1F2937",
				label: "Total",
			},
			{
				key: "wos",
				color: "#0075B4",
				label: "WoS",
			},
			{
				key: "scopus",
				color: "#C9A500",
				label: "Scopus",
			},
		].filter((s) => s.key === key)
	})

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
		const max = Math.max(maxVal, 1e-9)
		const raw = max / 6
		const mag = 10 ** Math.floor(Math.log10(raw))
		const norm = raw / mag
		const nice = norm < 1.5 ? 1 : norm < 3 ? 2 : norm < 7 ? 5 : 10
		const step = nice * mag
		const top = Math.ceil(max / step) * step
		const ticks: number[] = []
		for (let v = 0; v <= top + 1e-9; v += step) ticks.push(v)
		return ticks
	})

	const yMax = $derived(yTicks[yTicks.length - 1] ?? 1)

	const yTickFmt = $derived.by(() => {
		const step = yTicks[1] - yTicks[0]
		const decimals = step >= 1 ? 0 : Math.max(0, Math.ceil(-Math.log10(step) - 1e-9))
		return (d: number) => d.toFixed(decimals)
	})
	const minYear = $derived(allYears[0] ?? 0)
	const maxYear = $derived(allYears[allYears.length - 1] ?? 0)
</script>

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
		legend={false}
		points={true}
		props={{
			xAxis: { ticks: allYears.length, format: (d: number) => String(d) },
			yAxis: { ticks: yTicks, format: yTickFmt },
		}}
	/>
	<p class="mt-1 text-xs text-corp-gray">
		{indexation === "all"
			? "Publicaciones de todo el alcance ÷ Σ JCE (Doctor)."
			: indexation === "wos"
				? "Publicaciones indexadas en WoS ÷ Σ JCE (Doctor)."
				: "Publicaciones indexadas en Scopus ÷ Σ JCE (Doctor)."}
	</p>
	<p class="mt-3 text-sm text-corp-gray">
		Σ JCE (Doctor) del alcance: {productivity.data.jce} h.
	</p>
{/if}
