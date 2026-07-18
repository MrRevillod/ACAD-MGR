<script lang="ts">
	import * as v from "valibot"
	import type { Academic } from "$academics/entity"
	import type { TableFeatures } from "@tanstack/svelte-table"
	import type { GetAcademicsParams } from "$academics/dtos"

	import { toast } from "svelte-sonner"
	import { goto } from "$app/navigation"
	import { Loader, CircleAlert } from "@lucide/svelte"
	import { useSearchParams } from "runed/kit"
	import { createColumnHelper } from "@tanstack/svelte-table"
	import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query"

	import { FullName } from "$shared/value-objects/full-name.value"
	import { careerService } from "$careers/service"
	import { academicService } from "$academics/service"
	import { departmentService } from "$departments/service"
	import { categoryService } from "$categories/service"

	import DataTable from "$shared/components/ui/data-table.svelte"
	import AcademicsFilters from "$academics/components/academics-filters.svelte"
	import AcademicCreateDialog from "$academics/components/academic-create-dialog.svelte"

	const searchParamsSchema = v.object({
		search: v.optional(v.fallback(v.string(), ""), ""),
		department_id: v.optional(v.fallback(v.string(), ""), ""),
		career_id: v.optional(v.fallback(v.string(), ""), ""),
		category_id: v.optional(v.fallback(v.string(), ""), ""),
		planta: v.optional(v.fallback(v.string(), ""), ""),
		option: v.optional(v.fallback(v.string(), ""), ""),
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
		queryKey: ["careers", params.department_id],
		queryFn: () =>
			careerService.list(
				params.department_id ? { department_id: params.department_id } : undefined,
			),
	}))

	const categoriesQuery = createQuery(() => ({
		queryKey: ["categories"],
		queryFn: () => categoryService.list(),
	}))

	let filters = $derived<GetAcademicsParams>({
		...(params.search && { search: params.search }),
		...(params.department_id && { department_id: params.department_id }),
		...(params.career_id && { career_id: params.career_id }),
		...(params.category_id && { category_id: params.category_id }),
		...(params.planta && { planta: params.planta as GetAcademicsParams["planta"] }),
		...(params.option && { option: params.option as GetAcademicsParams["option"] }),
	})

	let showCreateDialog = $state(false)

	function clearFilters() {
		params.reset()
	}

	const query = createQuery(() => ({
		queryKey: ["academics", filters],
		queryFn: () => academicService.list(filters),
	}))

	const queryClient = useQueryClient()

	const importMutation = createMutation(() => ({
		mutationFn: (file: File) => academicService.import(file),
		onSuccess: (result) => {
			void queryClient.invalidateQueries({ queryKey: ["academics"] })
			toast.success(`${result.imported} académicos importados`)
			if (result.errors.length > 0) {
				toast.error(`${result.errors.length} filas con errores`)
			}
		},
		onError: () => toast.error("Error al importar el archivo"),
	}))

	const helper = createColumnHelper<TableFeatures, Academic>()

	const columns = [
		helper.accessor(
			(row) => FullName.of(row.names, row.paternalSurname, row.maternalSurname).format(),
			{
				id: "name",
				header: "Nombre",
			},
		),
		helper.accessor("email", { header: "Email" }),
		helper.accessor("department", { header: "Departamento" }),
		helper.accessor("category", { header: "Categoría" }),
		helper.accessor((row) => row.planta.toDisplay(), {
			id: "planta",
			header: "Planta",
		}),
		helper.accessor((row) => row.option.toDisplay(), {
			id: "option",
			header: "Opción",
		}),
	]
</script>

<div class="mx-auto flex h-full max-w-[1600px] flex-col px-4 py-8 sm:px-6 lg:px-8">
	<div class="flex min-h-0 flex-1 gap-8">
		<AcademicsFilters
			bind:search={params.search}
			bind:deptFilter={params.department_id}
			bind:careerFilter={params.career_id}
			bind:catFilter={params.category_id}
			bind:plantaFilter={params.planta}
			bind:optionFilter={params.option}
			departments={departmentsQuery.data}
			careers={careersQuery.data}
			categories={categoriesQuery.data}
			onClear={clearFilters}
			onCreate={() => (showCreateDialog = true)}
			onImport={(file) => importMutation.mutate(file)}
		/>

		<main class="min-w-0 flex-1 overflow-y-auto">
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
					onRowClick={(row: Academic) => void goto(`/academics/${row.id}`)}
				/>
			{/if}
		</main>
	</div>
</div>

<AcademicCreateDialog bind:open={showCreateDialog} onClose={() => (showCreateDialog = false)} />
