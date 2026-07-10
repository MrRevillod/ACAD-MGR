<script lang="ts">
	import Label from "$lib/shared/components/ui/label.svelte"
	import Select from "$lib/shared/components/ui/select.svelte"
	import Button from "$lib/shared/components/ui/button.svelte"
	import { Search, RotateCcw, Plus } from "@lucide/svelte"
	import { authStore } from "$lib/auth/auth.store.svelte"
	import type { Department } from "$lib/university/departments/dtos"
	import type { Career } from "$lib/university/careers/dtos"
	import type { AcademicCategory } from "$lib/academic/categories/dtos"

	interface Props {
		search: string
		deptFilter: string
		careerFilter: string
		catFilter: string
		plantaFilter: string
		optionFilter: string
		departments: Department[] | undefined
		careers: Career[] | undefined
		categories: AcademicCategory[] | undefined
		onClear: () => void
		onCreate: () => void
	}

	let {
		search = $bindable(),
		deptFilter = $bindable(),
		careerFilter = $bindable(),
		catFilter = $bindable(),
		plantaFilter = $bindable(),
		optionFilter = $bindable(),
		departments,
		careers,
		categories,
		onClear,
		onCreate,
	}: Props = $props()

	const isAdmin = $derived(authStore.isAuthenticated())

	const deptItems = $derived([
		{ value: "", label: "Todos" },
		...(departments?.map((d) => ({ value: d.id, label: d.name })) ?? []),
	])

	const careerItems = $derived([
		{ value: "", label: "Todas" },
		...(careers?.map((c) => ({ value: c.id, label: c.name })) ?? []),
	])

	const catItems = $derived([
		{ value: "", label: "Todas" },
		...(categories?.map((c) => ({ value: c.id, label: c.name })) ?? []),
	])

	const plantaItems = [
		{ value: "", label: "Todas" },
		{ value: "adjunta", label: "Adjunta" },
		{ value: "permanente", label: "Permanente" },
	]

	const optionItems = [
		{ value: "", label: "Todas" },
		{ value: "teaching", label: "Docencia" },
		{ value: "research", label: "Investigación" },
	]
</script>

<aside
	class="hidden w-72 shrink-0 self-start overflow-y-auto rounded-xl border border-corp-gray/20 bg-white p-4 lg:block"
>
	<h1 class="text-lg font-semibold text-[#1A1A1A]">Académicos</h1>
	<p class="mt-1 text-sm text-corp-gray">Facultad de Ingeniería</p>

	{#if isAdmin}
		<button
			class="mt-4 inline-flex w-full items-center justify-center gap-1.5 rounded-lg bg-corp-blue px-4 py-2 text-sm font-medium text-white shadow-sm transition-colors hover:bg-corp-blue/90 active:scale-95"
			onclick={onCreate}
		>
			<Plus class="size-4" />
			Crear académico
		</button>
	{/if}

	<div class="relative mt-6">
		<Search class="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-corp-gray/50" />
		<input
			type="text"
			bind:value={search}
			placeholder="Buscar académico..."
			class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white pl-10 pr-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
		/>
	</div>

	<div class="mt-6 space-y-4">
		<div class="space-y-2.5">
			<Label>Departamento</Label>
			<Select items={deptItems} bind:value={deptFilter} placeholder="Todos" />
		</div>
		<div class="space-y-2.5">
			<Label>Carrera</Label>
			<Select items={careerItems} bind:value={careerFilter} placeholder="Todas" />
		</div>
		<div class="space-y-2.5">
			<Label>Categoría</Label>
			<Select items={catItems} bind:value={catFilter} placeholder="Todas" />
		</div>
		<div class="space-y-2.5">
			<Label>Planta</Label>
			<Select items={plantaItems} bind:value={plantaFilter} placeholder="Todas" />
		</div>
		<div class="space-y-2.5">
			<Label>Opción</Label>
			<Select items={optionItems} bind:value={optionFilter} placeholder="Todas" />
		</div>
	</div>

	<Button variant="secondary" class="mt-6 w-full" onclick={onClear}>
		<RotateCcw class="size-4" />
		Limpiar filtros
	</Button>
</aside>
