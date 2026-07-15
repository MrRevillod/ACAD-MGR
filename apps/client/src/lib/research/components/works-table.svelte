<script lang="ts" generics="_TData extends RowData">
	import {
		createColumnHelper,
		renderSnippet,
		type RowData,
		type TableFeatures,
	} from "@tanstack/svelte-table"
	import { ExternalLink } from "@lucide/svelte"

	import DataTable from "$lib/shared/components/ui/data-table.svelte"

	import type { Work } from "../types"

	interface Props {
		works: Work[]
		onRowClick?: (work: Work) => void
	}

	let { works, onRowClick }: Props = $props()

	const helper = createColumnHelper<TableFeatures, Work>()

	const columns = [
		helper.accessor("title", {
			header: "Título",
			cell: (ctx) => renderSnippet(titleSnippet, { work: ctx.row.original }),
		}),
		helper.accessor("publicationYear", {
			header: "Año",
			cell: (ctx) => renderSnippet(yearSnippet, { year: ctx.row.original.publicationYear }),
		}),
		helper.accessor("journalKind", {
			header: "Indexación",
			cell: (ctx) =>
				renderSnippet(journalKindSnippet, { kind: ctx.row.original.journalKind }),
		}),
		helper.accessor("isAccepted", {
			header: "Estado",
			cell: (ctx) =>
				renderSnippet(statusSnippet, {
					isAccepted: ctx.row.original.isAccepted,
					isPublished: ctx.row.original.isPublished,
				}),
		}),
	]
</script>

{#snippet titleSnippet({ work }: { work: Work })}
	<div class="min-w-0">
		<p class="line-clamp-2 text-[15px] font-medium text-[#1A1A1A]">{work.title}</p>
		{#if work.doi}
			<a
				href={work.doi}
				target="_blank"
				rel="noopener"
				class="mt-0.5 inline-flex items-center gap-1 text-xs text-corp-blue hover:underline"
				onclick={(e) => e.stopPropagation()}
			>
				{work.doi}
				<ExternalLink class="size-3" />
			</a>
		{/if}
	</div>
{/snippet}

{#snippet yearSnippet({ year }: { year: number | null })}
	<span class="tabular-nums text-corp-gray">{year ?? "—"}</span>
{/snippet}

{#snippet journalKindSnippet({ kind }: { kind: string | null })}
	{#if kind === "wos"}
		<span
			class="inline-flex items-center rounded-full bg-corp-blue/10 px-2 py-0.5 text-xs font-semibold text-corp-blue uppercase"
		>
			WoS
		</span>
	{:else if kind === "scopus"}
		<span
			class="inline-flex items-center rounded-full bg-emerald-50 px-2 py-0.5 text-xs font-semibold text-emerald-700 uppercase"
		>
			Scopus
		</span>
	{:else}
		<span class="text-xs text-corp-gray/60">—</span>
	{/if}
{/snippet}

{#snippet statusSnippet({ isAccepted, isPublished }: { isAccepted: boolean; isPublished: boolean })}
	<div class="flex flex-wrap items-center gap-1">
		{#if isAccepted}
			<span
				class="inline-flex items-center rounded-full bg-emerald-50 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-emerald-700 uppercase"
			>
				Aceptado
			</span>
		{/if}
		{#if isPublished}
			<span
				class="inline-flex items-center rounded-full bg-corp-blue/10 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-corp-blue uppercase"
			>
				Publicado
			</span>
		{/if}
		{#if !isAccepted && !isPublished}
			<span class="text-xs text-corp-gray/60">—</span>
		{/if}
	</div>
{/snippet}

<DataTable data={works} {columns} {onRowClick} pageSize={10} />
