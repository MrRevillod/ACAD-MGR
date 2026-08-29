<script lang="ts">
	import { useProductivityQuery } from "../queries"

	import type { ProductivityJceScope, ProductivityDegree, ProductivityScope } from "../dtos"
	import type { ProductivityIndexation, ProductivityPrecision } from "../productivity-labels"
	import ProductivityChart from "./productivity-chart.svelte"
	import ProductivityFilters from "./productivity-filters.svelte"

	export interface ProductivityPanelProps {
		scope: ProductivityScope
		departmentId?: string
		researchLineId?: string
		yearFrom: number
		yearTo: number
		degree?: ProductivityDegree
		jceScope?: ProductivityJceScope
	}

	let {
		scope,
		departmentId,
		researchLineId,
		yearFrom,
		yearTo,
		degree = $bindable("all"),
		jceScope = $bindable<ProductivityJceScope>("doctor"),
	}: ProductivityPanelProps = $props()

	let month = $state("1")
	let indexation = $state<ProductivityIndexation>("all")
	let precision = $state<ProductivityPrecision>("3")

	const queryParams = $derived({
		degree,
		scope,
		jceScope,
		...(departmentId ? { departmentId } : {}),
		...(researchLineId ? { researchLineId } : {}),
		month: Number(month),
		yearFrom,
		yearTo,
	})

	const productivity = useProductivityQuery(() => queryParams)

	const countLabel = $derived(
		jceScope === "doctor" ? "N° de doctores" : "N° de académicos del alcance",
	)
	const jceLabel = $derived(
		jceScope === "doctor" ? "Horas JCE — Doctores" : "Horas JCE — Total del alcance",
	)
</script>

<div class="space-y-6">
	<ProductivityFilters bind:degree bind:jceScope bind:month bind:indexation bind:precision />

	<div class="flex flex-row items-start gap-8">
		<div class="space-y-4 w-1/5">
			<div class="rounded-xl border border-corp-gray/20 bg-white p-5">
				<p class="mb-2 text-[12px] font-medium uppercase tracking-wider text-black">
					Jornada Completa Equivalente
				</p>
				<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">
					{countLabel}
				</p>
				<p class="mt-2 text-[28px] font-semibold leading-none text-corp-ink tabular-nums">
					{productivity.data?.academicCount ?? "–"}
				</p>
				<p class="mt-5 text-[11px] font-medium uppercase tracking-wider text-corp-gray">
					{jceLabel}
				</p>
				<p class="mt-2 text-[28px] font-semibold leading-none text-corp-ink tabular-nums">
					{productivity.data ? `${productivity.data.jce} h` : "–"}
				</p>
			</div>
		</div>

		<div class="w-4/5">
			<ProductivityChart
				data={productivity.data}
				isPending={productivity.isPending}
				isError={productivity.isError}
				{indexation}
				{precision}
			/>
		</div>
	</div>
</div>
