<script lang="ts">
	import { Filter, RotateCcw, Search } from "@lucide/svelte"

	import Button from "$lib/shared/components/ui/button.svelte"
	import Label from "$lib/shared/components/ui/label.svelte"
	import Select from "$lib/shared/components/ui/select.svelte"

	import {
		useCareersQuery,
		useDepartmentsQuery,
		useDomainsQuery,
		useFieldsQuery,
		useKeywordsQuery,
		useSubfieldsQuery,
		useTopicsQuery,
	} from "../queries"
	import { WORK_TYPE_LABELS } from "../types"

	interface Props {
		search: string
		type: string
		domainId: string
		fieldId: string
		subfieldId: string
		topicId: string
		topicMinScore: string
		keywordId: string
		departmentId: string
		careerId: string
		yearFrom: string
		yearTo: string
		onClear: () => void
	}

	let {
		search = $bindable(),
		type = $bindable(),
		domainId = $bindable(),
		fieldId = $bindable(),
		subfieldId = $bindable(),
		topicId = $bindable(),
		topicMinScore = $bindable(),
		keywordId = $bindable(),
		departmentId = $bindable(),
		careerId = $bindable(),
		yearFrom = $bindable(),
		yearTo = $bindable(),
		onClear,
	}: Props = $props()

	const departmentsQuery = useDepartmentsQuery()
	const careersQuery = useCareersQuery(() => departmentId)
	const domainsQuery = useDomainsQuery()
	const keywordsQuery = useKeywordsQuery()

	const fieldsQuery = useFieldsQuery(() => domainId)
	const subfieldsQuery = useSubfieldsQuery(() => fieldId)
	const topicsQuery = useTopicsQuery(() => subfieldId)

	const departmentItems = $derived([
		{ value: "", label: "Todos los departamentos" },
		...(departmentsQuery.data?.map((d) => ({ value: d.id, label: d.name })) ?? []),
	])

	const careerItems = $derived([
		{
			value: "",
			label: departmentId ? "Todas las carreras" : "Selecciona un departamento primero",
		},
		...(careersQuery.data?.map((c) => ({ value: c.id, label: c.name })) ?? []),
	])

	const domainItems = $derived([
		{ value: "", label: "Todos los dominios" },
		...(domainsQuery.data?.map((d) => ({ value: d.id, label: d.name })) ?? []),
	])

	const fieldItems = $derived([
		{
			value: "",
			label: domainId ? "Todos los campos" : "Selecciona un dominio primero",
		},
		...(fieldsQuery.data?.map((f) => ({ value: f.id, label: f.name })) ?? []),
	])

	const subfieldItems = $derived([
		{
			value: "",
			label: fieldId ? "Todos los subcampos" : "Selecciona un campo primero",
		},
		...(subfieldsQuery.data?.map((s) => ({ value: s.id, label: s.name })) ?? []),
	])

	const topicItems = $derived([
		{
			value: "",
			label: subfieldId ? "Todos los tópicos" : "Selecciona un subcampo primero",
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

	function onTypeChange(v: string) {
		type = v || ""
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
			bind:value={search}
			oninput={() => (search = search || "")}
			placeholder="Buscar por título..."
			class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white pl-10 pr-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
		/>
	</div>

	<div class="mt-6 space-y-4">
		<div class="space-y-2.5">
			<Label>Tipo</Label>
			<Select items={workTypeItems} bind:value={type} onValueChange={onTypeChange} />
		</div>

		<div class="space-y-2.5">
			<Label>Departamento</Label>
			<Select items={departmentItems} bind:value={departmentId} />
		</div>

		<div class="space-y-2.5">
			<Label>Carrera</Label>
			<Select items={careerItems} bind:value={careerId} disabled={!departmentId} />
		</div>

		<div class="space-y-2.5">
			<Label>Dominio</Label>
			<Select items={domainItems} bind:value={domainId} />
		</div>

		<div class="space-y-2.5">
			<Label>Campo</Label>
			<Select items={fieldItems} bind:value={fieldId} disabled={!domainId} />
		</div>

		<div class="space-y-2.5">
			<Label>Subcampo</Label>
			<Select items={subfieldItems} bind:value={subfieldId} disabled={!fieldId} />
		</div>

		<div class="space-y-2.5">
			<Label>Tópico</Label>
			<Select items={topicItems} bind:value={topicId} disabled={!subfieldId} />
		</div>

		<div class="space-y-2.5">
			<Label>Score mínimo (tópico)</Label>
			<input
				type="number"
				step="0.05"
				min="0"
				max="1"
				bind:value={topicMinScore}
				oninput={() => (topicMinScore = topicMinScore ? String(Number(topicMinScore)) : "")}
				placeholder="0.0 – 1.0"
				class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm tabular-nums text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
			/>
		</div>

		<div class="space-y-2.5">
			<Label>Keyword</Label>
			<Select items={keywordItems} bind:value={keywordId} />
		</div>

		<div class="grid grid-cols-2 gap-2">
			<div class="space-y-2.5">
				<Label>Año desde</Label>
				<input
					type="number"
					min="1900"
					max="2100"
					bind:value={yearFrom}
					oninput={() => (yearFrom = yearFrom ? String(Number(yearFrom)) : "")}
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
					bind:value={yearTo}
					oninput={() => (yearTo = yearTo ? String(Number(yearTo)) : "")}
					placeholder="2100"
					class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm tabular-nums text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
				/>
			</div>
		</div>
	</div>

	<Button variant="secondary" class="mt-6 w-full" onclick={onClear}>
		<RotateCcw class="size-4" />
		Limpiar filtros
	</Button>
</aside>
