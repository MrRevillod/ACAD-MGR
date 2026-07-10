<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { renderSnippet, createColumnHelper, type TableFeatures } from "@tanstack/svelte-table"
	import { categoryService } from "$lib/academic/categories/service"
	import CategoryDialog from "$lib/academic/categories/components/category-dialog.svelte"
	import Button from "$lib/shared/components/ui/button.svelte"
	import Badge from "$lib/shared/components/ui/badge.svelte"
	import DataTable from "$lib/shared/components/ui/data-table.svelte"
	import { Plus, Loader2, Pencil, Trash2 } from "@lucide/svelte"
	import type { AcademicCategory } from "$lib/academic/categories/dtos"

	const query = createQuery(() => ({
		queryKey: ["admin", "categories"],
		queryFn: () => categoryService.list(),
	}))

	let showCreate = $state(false)

	const helper = createColumnHelper<TableFeatures, AcademicCategory>()

	const columns = [
		helper.accessor("name", { header: "Nombre" }),
		helper.accessor("planta", {
			header: "Planta",
			cell: (info) => renderSnippet(plantaBadge, { value: info.getValue() }),
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
			<h1 class="text-lg font-semibold text-[#1A1A1A]">Categorías Académicas</h1>
			<p class="mt-1 text-sm text-corp-gray">Gestiona las categorías académicas por planta.</p>
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

{#snippet plantaBadge(params: { value: string })}
	<Badge variant={params.value === "permanente" ? "advanced" : "base"}>
		{params.value === "permanente" ? "Permanente" : "Adjunta"}
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

<CategoryDialog bind:open={showCreate} onClose={() => (showCreate = false)} />
