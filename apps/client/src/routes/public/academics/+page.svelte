<script lang="ts">
	import * as v from "valibot"
	import type { Academic } from "$academics/entity"
	import type { GetAcademicsParams } from "$academics/dtos"

	import { goto } from "$app/navigation"
	import { Loader, CircleAlert, Search, RotateCcw } from "@lucide/svelte"
	import { useSearchParams } from "runed/kit"
	import { createColumnHelper, type TableFeatures } from "@tanstack/svelte-table"
	import { createQuery } from "@tanstack/svelte-query"

	import { FullName } from "$shared/value-objects/full-name.value"
	import { academicService } from "$academics/service"
	import { departmentService } from "$departments/service"
	import { careerService } from "$careers/service"

	import DataTable from "$shared/components/ui/data-table.svelte"
	import Label from "$shared/components/ui/label.svelte"
	import Select from "$shared/components/ui/select.svelte"
	import Button from "$shared/components/ui/button.svelte"

	const searchParamsSchema = v.object({
		search: v.optional(v.fallback(v.string(), ""), ""),
		departmentId: v.optional(v.fallback(v.string(), ""), ""),
		careerId: v.optional(v.fallback(v.string(), ""), ""),
	})

	const params = useSearchParams(searchParamsSchema, {
		debounce: 300,
		pushHistory: false,
	})

	const departmentsQuery = createQuery(() => ({
		queryKey: ["departments"],
		queryFn: () => departmentService.list(),
	}))

	const careersQuery = createQuery(() => ({
		queryKey: ["careers", params.departmentId],
		queryFn: () =>
			careerService.list(
				params.departmentId ? { department_id: params.departmentId } : undefined,
			),
	}))

	const filters = $derived<GetAcademicsParams>({
		...(params.search && { search: params.search }),
		...(params.departmentId && { departmentId: params.departmentId }),
		...(params.careerId && { careerId: params.careerId }),
	})

	function clearFilters() {
		params.reset()
	}

	const query = createQuery(() => ({
		queryKey: ["public-academics", filters],
		queryFn: () => academicService.listPublic(filters),
	}))

	const helper = createColumnHelper<TableFeatures, Academic>()

	const columns = [
		helper.accessor(
			(row) => FullName.of(row.names, row.paternalSurname, row.maternalSurname).format(),
			{ id: "name", header: "Nombre" },
		),
		helper.accessor("email", { header: "Email" }),
		helper.accessor("orcid", {
			header: "ORCID",
			cell: ({ getValue }) => getValue() ?? "—",
		}),
		helper.accessor((row) => row.joinedAt.toDisplayDate(), {
			id: "joinedAt",
			header: "Fecha de Ingreso",
		}),
	]

	const deptItems = $derived([
		{ value: "", label: "Todos" },
		...(departmentsQuery.data?.map((d) => ({ value: d.id, label: d.name })) ?? []),
	])

	const careerItems = $derived([
		{ value: "", label: "Todas" },
		...(careersQuery.data?.map((c) => ({ value: c.id, label: c.name })) ?? []),
	])
</script>

<div class="mx-auto flex h-full max-w-[1600px] flex-col gap-6 overflow-hidden p-4 sm:p-6 lg:p-8">
	<div class="flex items-end gap-4 rounded-xl border border-corp-gray/20 bg-white p-4">
		<div class="min-w-64 flex-1">
			<Label>Buscar</Label>
			<div class="relative mt-1.5">
				<Search class="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-corp-gray/50" />
				<input
					type="text"
					bind:value={params.search}
					placeholder="Buscar académico..."
					class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white pl-10 pr-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
				/>
			</div>
		</div>
		<div class="w-56">
			<Label>Departamento</Label>
			<div class="mt-1.5">
				<Select items={deptItems} bind:value={params.departmentId} placeholder="Todos" />
			</div>
		</div>
		<div class="w-56">
			<Label>Carrera</Label>
			<div class="mt-1.5">
				<Select items={careerItems} bind:value={params.careerId} placeholder="Todas" />
			</div>
		</div>
		<Button variant="secondary" onclick={clearFilters}>
			<RotateCcw class="size-4" />
			Limpiar
		</Button>
	</div>

	<div class="min-h-0 flex-1 overflow-y-auto">
		{#if query.isPending}
			<div class="flex items-center justify-center py-16">
				<Loader class="size-6 animate-spin text-corp-gray" />
			</div>
		{:else if query.isError}
			<div class="flex flex-col items-center justify-center py-16 text-center">
				<CircleAlert class="size-8 text-red-500" />
				<p class="mt-3 text-sm text-corp-gray">Error al cargar los académicos.</p>
			</div>
		{:else}
			<DataTable
				data={query.data ?? []}
				{columns}
				onRowClick={(row: Academic) => void goto(`/public/academics/${row.id}`)}
			/>
		{/if}
	</div>
</div>
