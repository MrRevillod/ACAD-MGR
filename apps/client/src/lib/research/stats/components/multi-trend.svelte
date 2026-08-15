<script lang="ts">
	import { LineChart } from "layerchart"

	import { colorForIndex, type ColoredScopeSeries } from "./scope-colors"

	interface Props {
		items: ColoredScopeSeries[]
		kind?: "wos" | "scopus"
	}

	let { items, kind = $bindable("wos") }: Props = $props()

	const allYears = $derived(
		[...new Set(items.flatMap((s) => [...s.wos, ...s.scopus].map((v) => v.year)))].sort(
			(a, b) => a - b,
		),
	)

	const series = $derived(
		items.map((s, i) => ({
			key: s.name,
			color: s.color ?? colorForIndex(i),
			label: s.name,
		})),
	)

	const wideData = $derived(
		allYears.map((year) => {
			const row: Record<string, number> = { year }
			for (const s of items) {
				const vals = kind === "wos" ? s.wos : s.scopus
				row[s.name] = vals.find((v) => v.year === year)?.value ?? 0
			}
			return row
		}),
	)

	const minYear = $derived(allYears[0] ?? 0)
	const maxYear = $derived(allYears[allYears.length - 1] ?? 0)

	const maxVal = $derived(
		Math.max(0, ...wideData.flatMap((r) => items.map((s) => r[s.name] ?? 0))),
	)

	const yTicks = $derived.by(() => {
		const max = Math.max(maxVal, 1)
		const step = Math.max(1, Math.ceil(max / 5))
		const top = Math.ceil(max / step) * step
		const ticks: number[] = []
		for (let v = 0; v <= top; v += step) ticks.push(v)
		return ticks
	})

	const yMax = $derived(yTicks[yTicks.length - 1] ?? 1)
</script>

<div>
	{#if series.length === 0 || allYears.length === 0}
		<p class="py-8 text-center text-sm text-corp-gray">Sin datos para mostrar.</p>
	{:else}
		<LineChart
			data={wideData}
			x="year"
			{series}
			height={280}
			padding={{ left: 40, right: 16, bottom: 40, top: 10 }}
			xDomain={[minYear, maxYear]}
			xNice={false}
			yDomain={[0, yMax]}
			yNice={false}
			legend={false}
			points={true}
			props={{
				xAxis: { ticks: allYears.length, format: (d: number) => String(d) },
				yAxis: { ticks: yTicks, format: (d: number) => String(d) },
			}}
		/>
	{/if}
</div>
