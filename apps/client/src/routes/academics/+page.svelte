<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { Search, Loader2, AlertCircle } from "@lucide/svelte"
	import type { ColumnDef } from "@tanstack/svelte-table"
	import { academicsService } from "$lib/services/academics.service"
	import DataTable from "$lib/components/ui/data-table.svelte"
	import type { AcademicView, AcademicOption } from "$lib/types"

	const query = createQuery(() => ({
		queryKey: ["academics"],
		queryFn: () => academicsService.list(),
	}))

	let search = $state("")

	function optionLabel(o: AcademicOption): string {
		switch (o) {
			case "teaching":
				return "Docencia"
			case "research":
				return "Investigación"
		}
	}

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const columns: ColumnDef<any, AcademicView, unknown>[] = [
		{
			id: "name",
			header: "Nombre",
			accessorFn: (row) => `${row.names} ${row.paternalSurname} ${row.maternalSurname}`,
		},
		{ id: "email", header: "Email", accessorKey: "email" },
		{ id: "department", header: "Departamento", accessorKey: "department" },
		{ id: "category", header: "Categoría", accessorKey: "category" },
		{
			id: "option",
			header: "Opción",
			accessorFn: (row) => optionLabel(row.option),
		},
	]
</script>

<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
	<div class="mb-6">
		<h1 class="text-lg font-semibold text-[#1A1A1A]">Académicos</h1>
		<p class="mt-1 text-sm text-corp-gray">Listado de académicos de la Facultad de Ingeniería.</p>
	</div>

	<div class="mb-6 flex items-center gap-3">
		<div class="relative flex-1 max-w-sm">
			<Search class="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-corp-gray/50" />
			<input
				type="text"
				bind:value={search}
				placeholder="Buscar académico..."
				class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white pl-10 pr-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
			/>
		</div>
	</div>

	{#if query.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader2 class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if query.isError}
		<div class="flex flex-col items-center justify-center py-16 text-center">
			<AlertCircle class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Error al cargar los académicos.</p>
		</div>
	{:else}
		<DataTable
			data={query.data ?? []}
			{columns}
			bind:search
			searchFields={[
				"names",
				"paternalSurname",
				"maternalSurname",
				"email",
				"department",
				"category",
			]}
			onRowClick={(row: AcademicView) => void goto(resolve(`/academics/${row.id}`))}
		/>
	{/if}
</div>
