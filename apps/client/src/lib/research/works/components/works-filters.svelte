<script lang="ts">
	import { useCareersQuery } from "$careers/queries"
	import { useDepartmentsQuery } from "$departments/queries"
	import { RotateCcw, Search, BookOpen, Funnel } from "@lucide/svelte"

	import Button from "$shared/components/ui/button.svelte"
	import Label from "$shared/components/ui/label.svelte"
	import Select from "$shared/components/ui/select.svelte"
	import YearRange from "$shared/components/ui/year-range.svelte"

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
	class="hidden w-80 shrink-0 self-start rounded-xl border border-corp-gray/20 bg-white p-4 lg:block"
>
	<div class="mb-5 border-b border-corp-gray/20 pb-5">
		<div class="flex items-center gap-3">
			<BookOpen class="size-4 mt-1 text-corp-blue" />
			<h1 class="text-lg font-semibold text-[#1A1A1A]">Publicaciones</h1>
		</div>
		<p class="mt-1.5 text-sm text-corp-gray">
			Catálogo de publicaciones académicas de la Facultad de Ingeniería UCT
		</p>
	</div>

	<div class="flex items-center gap-2">
		<Funnel class="size-4 text-corp-blue" />
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

		<div class="space-y-2.5">
			<Label>Rango anual de publicación</Label>
			<YearRange
				bind:yearFrom
				bind:yearTo
				minYear={1900}
				showLabels={false}
				placeholderFrom="DESDE"
				placeholderTo="HASTA"
			/>
		</div>
	</div>

	<Button variant="secondary" class="mt-6 w-full" onclick={onClear}>
		<RotateCcw class="size-4" />
		Limpiar filtros
	</Button>
</aside>
