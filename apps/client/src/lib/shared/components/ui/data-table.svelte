<script lang="ts" generics="TData extends RowData">
	import {
		createTable,
		FlexRender,
		tableFeatures,
		rowPaginationFeature,
		rowSortingFeature,
		createPaginatedRowModel,
		createSortedRowModel,
		type ColumnDef,
		type RowData,
	} from "@tanstack/svelte-table"
	import {
		ChevronUp,
		ChevronDown,
		ChevronsLeft,
		ChevronLeft,
		ChevronRight,
		ChevronsRight,
	} from "@lucide/svelte"

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	type FlexColumnDef<T extends RowData> = ColumnDef<any, T, any>

	interface DataTableProps<T extends RowData> {
		columns: FlexColumnDef<T>[]
		data: T[]
		onRowClick?: (row: T) => void
		search?: string
		searchFields?: string[]
		pageSize?: number
		class?: string
	}

	let {
		columns,
		data,
		onRowClick,
		search = $bindable(""),
		searchFields = [] as string[],
		pageSize = 15,
		class: className = "",
	}: DataTableProps<TData> = $props()

	const filtered = $derived.by(() => {
		if (!search || searchFields.length === 0) return data

		return data.filter((row) =>
			searchFields.some((field) => {
				const val = row[field as keyof TData]
				return String(val ?? "")
					.toLowerCase()
					.includes(search.toLowerCase())
			}),
		)
	})

	const features = tableFeatures({
		rowPaginationFeature,
		rowSortingFeature,
		paginatedRowModel: createPaginatedRowModel(),
		sortedRowModel: createSortedRowModel(),
	})

	const table = createTable(
		{
			features,
			// eslint-disable-next-line svelte/no-unused-svelte-ignore
			// svelte-ignore state_referenced_locally
			columns,
			get data() {
				return filtered
			},
		},
		(state) => ({ pagination: state.pagination, sorting: state.sorting }),
	)

	$effect(() => {
		if (pageSize !== table.state.pagination.pageSize) {
			table.setPageSize(pageSize)
		}
	})

	const pagination = $derived(table.state.pagination)
</script>

<div class="w-full rounded-xl border border-corp-gray/20 bg-white {className}">
	<table class="w-full caption-bottom text-sm">
		<thead>
			{#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
				<tr class="border-b border-corp-gray/20 bg-gray-100">
					{#each headerGroup.headers as header (header.id)}
						<th
							class="px-4 py-3 text-left text-xs font-medium tracking-wide uppercase text-corp-gray {header.column.getCanSort()
								? 'cursor-pointer select-none'
								: ''}"
							onclick={header.column.getToggleSortingHandler()}
							scope="col"
						>
							<span class="inline-flex items-center gap-1">
								<FlexRender {header} />
								{#if header.column.getIsSorted() === "asc"}
									<ChevronUp class="size-3.5 shrink-0" />
								{:else if header.column.getIsSorted() === "desc"}
									<ChevronDown class="size-3.5 shrink-0" />
								{/if}
							</span>
						</th>
					{/each}
				</tr>
			{/each}
		</thead>
		<tbody>
			{#each table.getRowModel().rows as row (row.id)}
				<tr
					class="border-b border-corp-gray/10 transition-colors last:border-0 even:bg-gray-50 {onRowClick
						? 'cursor-pointer hover:bg-corp-blue/4'
						: ''}"
					role={onRowClick ? "button" : undefined}
					tabindex={onRowClick ? 0 : undefined}
					onclick={() => onRowClick?.(row.original)}
				>
					{#each row.getAllCells() as cell (cell.id)}
						<td class="px-4 py-3 text-[#1A1A1A]">
							<FlexRender {cell} />
						</td>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>

	{#if filtered.length === 0}
		<p class="px-4 py-8 text-center text-sm text-corp-gray">Sin resultados.</p>
	{/if}

	{#if table.getPageCount() > 1}
		<div class="flex items-center justify-between border-t border-corp-gray/10 px-4 py-3">
			<p class="text-sm text-corp-gray">
				Página {pagination.pageIndex + 1} de {table.getPageCount()} ({filtered.length} registros)
			</p>
			<div class="flex items-center gap-1">
				<button
					class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-[#1A1A1A] disabled:opacity-30"
					onclick={() => table.setPageIndex(0)}
					disabled={!table.getCanPreviousPage()}
				>
					<ChevronsLeft class="size-4" />
				</button>
				<button
					class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-[#1A1A1A] disabled:opacity-30"
					onclick={() => table.previousPage()}
					disabled={!table.getCanPreviousPage()}
				>
					<ChevronLeft class="size-4" />
				</button>
				{#each { length: Math.min(table.getPageCount(), 7) } as _, i (i)}
					{@const pageNum =
						Math.max(0, Math.min(pagination.pageIndex - 3, table.getPageCount() - 7)) + i}
					{#if pageNum < table.getPageCount()}
						<button
							class="flex size-8 items-center justify-center rounded-lg text-sm transition-colors {pageNum ===
							pagination.pageIndex
								? 'bg-corp-blue text-white'
								: 'text-corp-gray hover:bg-corp-gray/5 hover:text-[#1A1A1A]'}"
							onclick={() => table.setPageIndex(pageNum)}
						>
							{pageNum + 1}
						</button>
					{/if}
				{/each}
				<button
					class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-[#1A1A1A] disabled:opacity-30"
					onclick={() => table.nextPage()}
					disabled={!table.getCanNextPage()}
				>
					<ChevronRight class="size-4" />
				</button>
				<button
					class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-[#1A1A1A] disabled:opacity-30"
					onclick={() => table.setPageIndex(table.getPageCount() - 1)}
					disabled={!table.getCanNextPage()}
				>
					<ChevronsRight class="size-4" />
				</button>
			</div>
		</div>
	{/if}
</div>
