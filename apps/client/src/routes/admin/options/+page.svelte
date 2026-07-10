<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { renderSnippet, createColumnHelper, type TableFeatures } from "@tanstack/svelte-table"
	import { optionService } from "$lib/academic/options/service"
	import { categoryService } from "$lib/academic/categories/service"
	import OptionDialog from "$lib/academic/options/components/option-dialog.svelte"
	import Button from "$lib/shared/components/ui/button.svelte"
	import Badge from "$lib/shared/components/ui/badge.svelte"
	import DataTable from "$lib/shared/components/ui/data-table.svelte"
	import { Plus, Loader2, Pencil, Trash2 } from "@lucide/svelte"
	import { ACADEMIC_OPTION_LABELS, type AcademicOption } from "$lib/academic/academics/enums"
	import type { AcademicCategoryOption } from "$lib/academic/options/dtos"

	const query = createQuery(() => ({
		queryKey: ["admin", "options"],
		queryFn: () => optionService.list(),
	}))

	const categoriesQuery = createQuery(() => ({
		queryKey: ["admin", "categories"],
		queryFn: () => categoryService.list(),
	}))

	const categories = $derived(categoriesQuery.data ?? [])
	const categoryMap = $derived(Object.fromEntries(categories.map((c) => [c.id, c.name])))

	let showCreate = $state(false)

	const helper = createColumnHelper<TableFeatures, AcademicCategoryOption>()

	const columns = [
		helper.accessor("categoryId", {
			header: "Categoría",
			cell: (info) => categoryMap[info.getValue()] ?? info.getValue(),
		}),
		helper.accessor("option", {
			header: "Opción",
			cell: (info) => renderSnippet(optionBadge, { value: info.getValue() }),
		}),
		helper.accessor("hours", {
			header: "Horas",
			cell: (info) => info.getValue()?.toLocaleString("es-CL") ?? "—",
		}),
		helper.display({
			id: "actions",
			header: "Acciones",
			cell: () => renderSnippet(actionsCell, {}),
		}),
	]
</script>

<div>
	<div class="mb-6 flex items-center justify-between">
		<div>
			<h1 class="text-lg font-semibold text-[#1A1A1A]">Opciones de Categoría</h1>
			<p class="mt-1 text-sm text-corp-gray">Gestiona las opciones válidas por categoría.</p>
		</div>
		<Button onclick={() => (showCreate = true)}>
			<Plus class="size-4" />
			Nueva
		</Button>
	</div>

	{#if query.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader2 class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else}
		<DataTable data={query.data ?? []} {columns} pageSize={10} />
	{/if}
</div>

{#snippet optionBadge(params: { value: AcademicOption })}
	<Badge variant={params.value === "research" ? "advanced" : "base"}>
		{ACADEMIC_OPTION_LABELS[params.value]}
	</Badge>
{/snippet}

{#snippet actionsCell(_: Record<string, never>)}
	<div class="flex items-center gap-1">
		<button
			class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-[#1A1A1A]"
		>
			<Pencil class="size-4" />
		</button>
		<button
			class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-red-50 hover:text-red-600"
		>
			<Trash2 class="size-4" />
		</button>
	</div>
{/snippet}

<OptionDialog bind:open={showCreate} onClose={() => (showCreate = false)} />
