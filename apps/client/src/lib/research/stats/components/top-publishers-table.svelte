<script lang="ts">
	import type { TableFeatures } from "@tanstack/svelte-table"
	import type { TopPublisher } from "$stats/dtos"

	import { goto } from "$app/navigation"
	import { FullName } from "$shared/value-objects/full-name.value"
	import { authStore } from "$lib/auth/store.svelte"
	import { createColumnHelper, renderSnippet } from "@tanstack/svelte-table"

	import Badge from "$shared/components/ui/badge.svelte"
	import DataTable from "$shared/components/ui/data-table.svelte"

	interface Props {
		publishers: TopPublisher[]
	}

	let { publishers }: Props = $props()

	const optionLabel: Record<string, string> = {
		teaching: "Docencia",
		research: "Investigación",
	}

	const helper = createColumnHelper<TableFeatures, TopPublisher>()

	const columns = [
		helper.accessor((row) => FullName.fromFullString(row.name).toString(), {
			id: "name",
			header: "Nombre",
			cell: (info) => renderSnippet(nameSnippet, { name: info.getValue() }),
		}),
		helper.accessor("total", {
			id: "total",
			header: "Total",
			cell: (info) =>
				renderSnippet(numberSnippet, {
					value: info.getValue(),
					cls: "font-semibold text-corp-ink",
				}),
		}),
		helper.accessor("wos", {
			id: "wos",
			header: "WoS",
			cell: (info) =>
				renderSnippet(numberSnippet, {
					value: info.getValue(),
					cls: "font-medium text-corp-blue",
				}),
		}),
		helper.accessor("scopus", {
			id: "scopus",
			header: "Scopus",
			cell: (info) =>
				renderSnippet(numberSnippet, {
					value: info.getValue(),
					cls: "font-medium text-corp-gold",
				}),
		}),
		helper.accessor("unindexed", {
			id: "unindexed",
			header: "Sin indexar",
			cell: (info) =>
				renderSnippet(numberSnippet, {
					value: info.getValue(),
					cls: "font-medium text-corp-gray",
				}),
		}),
		helper.accessor("option", {
			id: "option",
			header: "Opción",
			cell: (info) => renderSnippet(optionSnippet, { option: info.getValue() }),
		}),
	]

	function rowClick(p: TopPublisher) {
		const dest = authStore.isAuthenticated
			? `/academics/${p.academicId}`
			: `/public/academics/${p.academicId}`
		void goto(dest)
	}
</script>

<DataTable data={publishers} {columns} pageSize={10} onRowClick={rowClick} />

{#snippet nameSnippet({ name }: { name: string })}
	<span>{name}</span>
{/snippet}

{#snippet numberSnippet({ value, cls }: { value: number; cls: string })}
	<span class="text-right tabular-nums {cls}">{value}</span>
{/snippet}

{#snippet optionSnippet({ option }: { option: string })}
	<Badge variant={option === "research" ? "advanced" : "base"}>
		{optionLabel[option] ?? option}
	</Badge>
{/snippet}
