<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { Loader2, AlertCircle } from "@lucide/svelte"
	import { createColumnHelper, type TableFeatures } from "@tanstack/svelte-table"
	import { useSearchParams } from "runed/kit"
	import * as v from "valibot"
	import { academicService } from "$lib/academic/academics/service"
	import { departmentService } from "$lib/university/departments/service"
	import { careerService } from "$lib/university/careers/service"
	import { categoryService } from "$lib/academic/categories/service"
	import DataTable from "$lib/shared/components/ui/data-table.svelte"
	import AcademicsFilters from "$lib/academic/academics/components/academics-filters.svelte"
	import AcademicCreateDialog from "$lib/academic/academics/components/academic-create-dialog.svelte"
	import type { Academic, GetAcademicsParams } from "$lib/academic/academics/dtos"
	import { PLANTA_LABELS, ACADEMIC_OPTION_LABELS } from "$lib/academic/academics/enums"
	import { FullName } from "$lib/shared/value-objects/full-name.value"

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
		helper.accessor((row) => PLANTA_LABELS[row.planta], {
			id: "planta",
			header: "Planta",
		}),
		helper.accessor((row) => ACADEMIC_OPTION_LABELS[row.option], {
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
		/>

		<main class="min-w-0 flex-1 overflow-y-auto">
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
					onRowClick={(row: Academic) => void goto(resolve(`/academics/${row.id}`))}
				/>
			{/if}
		</main>
	</div>
</div>

<AcademicCreateDialog bind:open={showCreateDialog} onClose={() => (showCreateDialog = false)} />
