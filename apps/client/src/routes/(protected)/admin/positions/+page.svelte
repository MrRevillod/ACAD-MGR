<script lang="ts">
	import type { TableFeatures } from "@tanstack/svelte-table"
	import type { AcademicWorkPosition } from "$work-positions/entity"

	import { createQuery } from "@tanstack/svelte-query"
	import { positionService } from "$work-positions/service"
	import { Plus, Loader, Pencil, Trash2 } from "@lucide/svelte"
	import { renderSnippet, createColumnHelper } from "@tanstack/svelte-table"

	import Button from "$shared/components/ui/button.svelte"
	import DataTable from "$shared/components/ui/data-table.svelte"
	import PositionDialog from "$work-positions/components/position-dialog.svelte"

	const query = createQuery(() => ({
		queryKey: ["admin", "positions"],
		queryFn: () => positionService.list(),
	}))

	let showCreate = $state(false)

	const helper = createColumnHelper<TableFeatures, AcademicWorkPosition>()

	const columns = [
		helper.accessor("name", { header: "Nombre" }),
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
			<h1 class="text-lg font-semibold text-[#1A1A1A]">Cargos Laborales</h1>
			<p class="mt-1 text-sm text-corp-gray">
				Gestiona los cargos laborales de los académicos.
			</p>
		</div>
		<Button onclick={() => (showCreate = true)}>
			<Plus class="size-4" />
			Nuevo
		</Button>
	</div>

	{#if query.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else}
		<DataTable data={query.data ?? []} {columns} pageSize={10} />
	{/if}
</div>

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

<PositionDialog bind:open={showCreate} onClose={() => (showCreate = false)} />
