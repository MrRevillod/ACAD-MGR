<script lang="ts">
	import type { TimeSeriesStat } from "$stats/dtos"
	import { LineChart } from "layerchart"

	interface Props {
		journalKind: TimeSeriesStat[]
	}

	let { journalKind }: Props = $props()

	const scopusVals = $derived(journalKind.find((s) => s.key === "scopus")?.values ?? [])
	const wosVals = $derived(journalKind.find((s) => s.key === "wos")?.values ?? [])

	const allYears = $derived(
		[...new Set([...scopusVals, ...wosVals].map((v) => v.year))].sort((a, b) => a - b),
	)

	const wideData = $derived(
		allYears.map((year) => ({
			year,
			scopus: scopusVals.find((v) => v.year === year)?.value ?? 0,
			wos: wosVals.find((v) => v.year === year)?.value ?? 0,
		})),
	)

	const minYear = $derived(allYears[0] ?? 0)
	const maxYear = $derived(allYears[allYears.length - 1] ?? 0)
</script>

{#if allYears.length === 0}
	<p class="py-8 text-center text-sm text-corp-gray">Sin datos para mostrar.</p>
{:else}
	<LineChart
		data={wideData}
		x="year"
		series={[
			{ key: "scopus", color: "#EDC500", label: "Scopus" },
			{ key: "wos", color: "#0075B4", label: "WoS" },
		]}
		height={260}
		padding={{ left: 50, right: 20, bottom: 50, top: 10 }}
		xDomain={[minYear, maxYear]}
		xNice={false}
		yDomain={[0, null]}
		yNice={true}
		legend={true}
		points={true}
		props={{
			xAxis: { ticks: allYears.length, format: (d: number) => String(d) },
		}}
	/>
{/if}
