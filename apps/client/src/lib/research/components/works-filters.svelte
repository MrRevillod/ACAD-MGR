<script lang="ts">
	import { Filter, RotateCcw, Search } from "@lucide/svelte"

	import Button from "$lib/shared/components/ui/button.svelte"
	import Label from "$lib/shared/components/ui/label.svelte"
	import Select from "$lib/shared/components/ui/select.svelte"

	import {
		useDomainsQuery,
		useFieldsQuery,
		useKeywordsQuery,
		useSubfieldsQuery,
		useTopicsQuery,
	} from "../queries"
	import { WORK_TYPE_LABELS, type GetWorksParams, type WorkType } from "../types"

	interface Props {
		filters: GetWorksParams
	}

	let { filters = $bindable() }: Props = $props()

	const domainsQuery = useDomainsQuery()
	const keywordsQuery = useKeywordsQuery()

	const fieldsQuery = useFieldsQuery(() => filters.domainId)
	const subfieldsQuery = useSubfieldsQuery(() => filters.fieldId)
	const topicsQuery = useTopicsQuery(() => filters.subfieldId)

	$effect(() => {
		if (filters.domainId === undefined) filters.fieldId = undefined
	})
	$effect(() => {
		if (filters.fieldId === undefined) filters.subfieldId = undefined
	})
	$effect(() => {
		if (filters.subfieldId === undefined) filters.topicId = undefined
	})

	const domainItems = $derived([
		{ value: "", label: "Todos los dominios" },
		...(domainsQuery.data?.map((d) => ({ value: d.id, label: d.name })) ?? []),
	])

	const fieldItems = $derived([
		{ value: "", label: filters.domainId ? "Todos los campos" : "Selecciona un dominio primero" },
		...(fieldsQuery.data?.map((f) => ({ value: f.id, label: f.name })) ?? []),
	])

	const subfieldItems = $derived([
		{ value: "", label: filters.fieldId ? "Todos los subcampos" : "Selecciona un campo primero" },
		...(subfieldsQuery.data?.map((s) => ({ value: s.id, label: s.name })) ?? []),
	])

	const topicItems = $derived([
		{
			value: "",
			label: filters.subfieldId ? "Todos los tópicos" : "Selecciona un subcampo primero",
		},
		...(topicsQuery.data?.map((t) => ({ value: t.id, label: t.name })) ?? []),
	])

	const keywordItems = $derived([
		{ value: "", label: "Todas las keywords" },
		...(keywordsQuery.data?.map((k) => ({ value: k.id, label: k.name })) ?? []),
	])

	const workTypeItems = $derived([
		{ value: "", label: "Todos los tipos" },
		...Object.entries(WORK_TYPE_LABELS).map(([value, label]) => ({ value, label })),
	])

	let typeValue = $derived(filters.type?.[0] ?? "")
	let searchValue = $derived(filters.search ?? "")
	let topicMinScoreValue = $derived(filters.topicMinScore?.toString() ?? "")
	let yearFromValue = $derived(filters.yearFrom?.toString() ?? "")
	let yearToValue = $derived(filters.yearTo?.toString() ?? "")

	function clearFilters() {
		filters = { size: 100 }
	}

	function onTypeChange(v: string) {
		filters.type = v ? [v as WorkType] : undefined
	}
</script>

<aside
	class="hidden w-80 shrink-0 overflow-y-auto rounded-xl border border-corp-gray/20 bg-white p-4 lg:block"
>
	<div class="flex items-center gap-2">
		<Filter class="size-4 text-corp-blue" />
		<h2 class="text-xs font-semibold tracking-widest text-[#1A1A1A] uppercase">Filtros</h2>
	</div>

	<div class="relative mt-4">
		<Search class="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-corp-gray/50" />
		<input
			type="text"
			bind:value={searchValue}
			oninput={() => (filters.search = searchValue || undefined)}
			placeholder="Buscar por título..."
			class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white pl-10 pr-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
		/>
	</div>

	<div class="mt-6 space-y-4">
		<div class="space-y-2.5">
			<Label>Tipo</Label>
			<Select items={workTypeItems} bind:value={typeValue} onValueChange={onTypeChange} />
		</div>

		<div class="space-y-2.5">
			<Label>Dominio</Label>
			<Select
				items={domainItems}
				value={filters.domainId ?? ""}
				onValueChange={(v) => (filters.domainId = v || undefined)}
			/>
		</div>

		<div class="space-y-2.5">
			<Label>Campo</Label>
			<Select
				items={fieldItems}
				value={filters.fieldId ?? ""}
				onValueChange={(v) => (filters.fieldId = v || undefined)}
				disabled={!filters.domainId}
			/>
		</div>

		<div class="space-y-2.5">
			<Label>Subcampo</Label>
			<Select
				items={subfieldItems}
				value={filters.subfieldId ?? ""}
				onValueChange={(v) => (filters.subfieldId = v || undefined)}
				disabled={!filters.fieldId}
			/>
		</div>

		<div class="space-y-2.5">
			<Label>Tópico</Label>
			<Select
				items={topicItems}
				value={filters.topicId ?? ""}
				onValueChange={(v) => (filters.topicId = v || undefined)}
				disabled={!filters.subfieldId}
			/>
		</div>

		<div class="space-y-2.5">
			<Label>Score mínimo (tópico)</Label>
			<input
				type="number"
				step="0.05"
				min="0"
				max="1"
				bind:value={topicMinScoreValue}
				oninput={() =>
					(filters.topicMinScore = topicMinScoreValue ? Number(topicMinScoreValue) : undefined)}
				placeholder="0.0 – 1.0"
				class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm tabular-nums text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
			/>
		</div>

		<div class="space-y-2.5">
			<Label>Keyword</Label>
			<Select
				items={keywordItems}
				value={filters.keywordId ?? ""}
				onValueChange={(v) => (filters.keywordId = v || undefined)}
			/>
		</div>

		<div class="grid grid-cols-2 gap-2">
			<div class="space-y-2.5">
				<Label>Año desde</Label>
				<input
					type="number"
					min="1900"
					max="2100"
					bind:value={yearFromValue}
					oninput={() => (filters.yearFrom = yearFromValue ? Number(yearFromValue) : undefined)}
					placeholder="1900"
					class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm tabular-nums text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
				/>
			</div>
			<div class="space-y-2.5">
				<Label>Año hasta</Label>
				<input
					type="number"
					min="1900"
					max="2100"
					bind:value={yearToValue}
					oninput={() => (filters.yearTo = yearToValue ? Number(yearToValue) : undefined)}
					placeholder="2100"
					class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm tabular-nums text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
				/>
			</div>
		</div>
	</div>

	<Button variant="secondary" class="mt-6 w-full" onclick={clearFilters}>
		<RotateCcw class="size-4" />
		Limpiar filtros
	</Button>
</aside>
