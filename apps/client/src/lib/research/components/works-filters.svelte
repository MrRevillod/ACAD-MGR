<script lang="ts">
	import { Filter, RotateCcw, Search } from "@lucide/svelte"

	import Button from "$lib/shared/components/ui/button.svelte"
	import Label from "$lib/shared/components/ui/label.svelte"
	import Select from "$lib/shared/components/ui/select.svelte"

	import { useCareersQuery, useDepartmentsQuery } from "../queries"

	interface Props {
		search: string
		departmentId: string
		careerId: string
		yearFrom: string
		yearTo: string
		journalKind: string
		onClear: () => void
	}

	let {
		search = $bindable(),
		departmentId = $bindable(),
		careerId = $bindable(),
		yearFrom = $bindable(),
		yearTo = $bindable(),
		journalKind = $bindable(),
		onClear,
	}: Props = $props()

	const departmentsQuery = useDepartmentsQuery()
	const careersQuery = useCareersQuery(() => departmentId)

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

	const journalKindItems = $derived([
		{ value: "", label: "Todas las clasificaciones" },
		{ value: "scopus", label: "Scopus" },
		{ value: "wos", label: "WoS" },
	])
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
			placeholder="Buscar por título..."
			class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white pl-10 pr-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
		/>
	</div>

	<div class="mt-6 space-y-4">
		<div class="space-y-2.5">
			<Label>Departamento</Label>
			<Select items={departmentItems} bind:value={departmentId} />
		</div>

		<div class="space-y-2.5">
			<Label>Carrera</Label>
			<Select items={careerItems} bind:value={careerId} disabled={!departmentId} />
		</div>

		<div class="space-y-2.5">
			<Label>Indexación</Label>
			<Select items={journalKindItems} bind:value={journalKind} />
		</div>

		<div class="grid grid-cols-2 gap-2">
			<div class="space-y-2.5">
				<Label>Año desde</Label>
				<input
					type="number"
					min="1900"
					max="2100"
					bind:value={yearFrom}
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
