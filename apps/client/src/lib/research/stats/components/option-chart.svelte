<script lang="ts">
	import type { TimeSeriesStat } from "../types"

	interface Props {
		data: TimeSeriesStat[]
	}

	let { data }: Props = $props()

	const labels: Record<string, string> = {
		teaching: "Docencia",
		research: "Investigación",
	}

	const colors: Record<string, string> = {
		teaching: "#0075B4",
		research: "#EDC500",
	}

	const allYears = $derived([
		...new Set(data.flatMap((s) => s.values.map((v) => v.year))),
	].sort((a, b) => a - b))

	const maxVal = $derived(
		Math.max(...data.flatMap((s) => s.values.map((v) => v.value)), 1),
	)

	const pad = 40
	const bottom = 40
	const right = 10
	const chartW = $derived(800 - pad - right)
	const chartH = 260
	const totalW = $derived(pad + chartW + right)
	const totalH = $derived(chartH + bottom)

	const barGroupW = $derived(chartW / Math.max(allYears.length, 1))
	const barW = $derived(Math.max(barGroupW / (data.length + 1) - 2, 4))

	function y(v: number) {
		return chartH - (v / maxVal) * (chartH - 20)
	}

	const yTicks = $derived.by(() => {
		const ticks = 5
		const step = Math.ceil(maxVal / ticks)
		return Array.from({ length: ticks + 1 }, (_, i) => i * step).filter((t) => t <= maxVal)
	})
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
			{@const gx = pad + yi * barGroupW + barGroupW / 2 - (data.length * (barW + 2)) / 2}

			{#each data as series, si (series.key)}
				{@const entry = series.values.find((v) => v.year === year)}
				{@const val = entry?.value ?? 0}
				{@const bx = gx + si * (barW + 2)}
				{@const by = y(val)}
				{@const bh = chartH - by}

				<rect
					x={bx}
					y={by}
					width={barW}
					height={bh}
					rx="2"
					fill={colors[series.key] ?? "#878787"}
					opacity="0.85"
				/>
				<text
					x={bx + barW / 2}
					y={by - 4}
					text-anchor="middle"
					class="fill-[#1A1A1A] text-[10px] font-semibold"
				>
					{val}
				</text>
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

	<div class="mt-3 flex items-center justify-center gap-6 text-xs font-medium text-corp-gray">
		{#each data as series (series.key)}
			<span class="inline-flex items-center gap-1.5">
				<span
					class="inline-block size-2.5 rounded-sm"
					style="background:{colors[series.key] ?? '#878787'}"
				></span>
				{labels[series.key] ?? series.key}
			</span>
		{/each}
	</div>
{/if}
