<script lang="ts">
	import type { TimeSeriesStat } from "../types"

	interface Props {
		data: TimeSeriesStat[]
	}

	let { data }: Props = $props()

	const allYears = $derived([
		...new Set(data.flatMap((s) => s.values.map((v) => v.year))),
	].sort((a, b) => a - b))

	const maxVal = $derived(
		Math.max(...data.flatMap((s) => s.values.map((v) => v.value)), 1),
	)

	const deptColors = [
		"#0075B4", "#EDC500", "#2E86AB", "#A4243B", "#D8973C",
		"#3C896D", "#8A4F7D", "#D16014", "#1B998B", "#6A4C93",
	]

	function color(i: number): string {
		return deptColors[i % deptColors.length]!
	}

	const pad = 50
	const bottom = 80
	const right = 10
	const chartW = $derived(800 - pad - right)
	const chartH = 300
	const totalW = $derived(pad + chartW + right)
	const totalH = $derived(chartH + bottom)

	const barGroupW = $derived(chartW / Math.max(allYears.length, 1))
	const barW = $derived(Math.max(barGroupW / (data.length + 1) - 1, 3))

	function y(v: number) {
		return chartH - (v / maxVal) * (chartH - 20)
	}

	const yTicks = $derived.by(() => {
		const ticks = 5
		const step = Math.ceil(maxVal / ticks)
		return Array.from({ length: ticks + 1 }, (_, i) => i * step).filter((t) => t <= maxVal)
	})

	const legendRows = $derived(data.length > 5 ? 2 : 1)
	const perRow = $derived(Math.ceil(data.length / legendRows))
</script>

{#if allYears.length === 0}
	<p class="py-8 text-center text-sm text-corp-gray">Sin datos para mostrar.</p>
{:else}
	<svg viewBox="0 0 {totalW} {totalH}" class="w-full" role="img">
		{#each yTicks as tick (tick)}
			<line
				x1={pad}
				x2={pad + chartW}
				y1={y(tick)}
				y2={y(tick)}
				class="stroke-corp-gray/15"
			/>
			<text x={pad - 6} y={y(tick) + 4} text-anchor="end" class="fill-corp-gray text-[10px]">{tick}</text>
		{/each}

		{#each allYears as year, yi (year)}
			{@const gx = pad + yi * barGroupW + barGroupW / 2 - (data.length * (barW + 1)) / 2}

			{#each data as series, si (series.key)}
				{@const entry = series.values.find((v) => v.year === year)}
				{@const val = entry?.value ?? 0}
				{@const bx = gx + si * (barW + 1)}
				{@const by = y(val)}
				{@const bh = chartH - by}

				<rect
					x={bx}
					y={by}
					width={barW}
					height={bh}
					rx="1"
					fill={color(si)}
					opacity="0.85"
				/>
				{#if val > 0}
					<text
						x={bx + barW / 2}
						y={by - 3}
						text-anchor="middle"
						class="fill-[#1A1A1A] text-[9px] font-semibold"
					>
						{val}
					</text>
				{/if}
			{/each}

			<text
				x={pad + yi * barGroupW + barGroupW / 2}
				y={chartH + 16}
				text-anchor="middle"
				class="fill-corp-gray text-[10px] font-medium"
			>
				{year}
			</text>
		{/each}
	</svg>

	<div class="mt-3 flex flex-wrap items-center justify-center gap-x-4 gap-y-1 text-xs font-medium text-corp-gray">
		{#each data as series, si (series.key)}
			<span class="inline-flex items-center gap-1.5">
				<span class="inline-block size-2.5 rounded-sm" style="background:{color(si)}"></span>
				{series.key}
			</span>
		{/each}
	</div>
{/if}
