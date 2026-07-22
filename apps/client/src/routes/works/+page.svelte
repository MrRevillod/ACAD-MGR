<script lang="ts">
	import * as v from "valibot"
	import type { Work } from "$works/entity"
	import type { JournalKind } from "$works/value-objects/journal-kind.value"
	import type { GetWorksParams } from "$works/dtos"

	import { useWorksQuery } from "$works/queries"
	import { useSearchParams } from "runed/kit"
	import { CircleAlert, BookOpen, Loader } from "@lucide/svelte"

	import WorksTable from "$works/components/works-table.svelte"
	import WorksFilters from "$works/components/works-filters.svelte"
	import WorkDetailDialog from "$works/components/work-detail-dialog.svelte"

	const yearFromDefault = String(new Date().getFullYear() - 5)

	const schema = v.object({
		search: v.optional(v.fallback(v.string(), ""), ""),
		departmentId: v.optional(v.fallback(v.string(), ""), ""),
		careerId: v.optional(v.fallback(v.string(), ""), ""),
		yearFrom: v.optional(v.fallback(v.string(), yearFromDefault), yearFromDefault),
		yearTo: v.optional(v.fallback(v.string(), ""), ""),
		journalKind: v.optional(v.fallback(v.string(), ""), ""),
		researchLineId: v.optional(v.fallback(v.string(), ""), ""),
	})

	const params = useSearchParams(schema, { debounce: 300, pushHistory: false })

	let filters = $derived<GetWorksParams>({
		size: 100,
		yearFrom: Number(params.yearFrom),
		...(params.search && { search: params.search }),
		...(params.departmentId && { departmentId: params.departmentId }),
		...(params.departmentId && params.careerId && { careerId: params.careerId }),
		...(params.yearTo && { yearTo: Number(params.yearTo) }),
		...(params.journalKind && { journalKind: params.journalKind as JournalKind }),
		...(params.researchLineId && { researchLineId: params.researchLineId }),
	})

	const worksQuery = useWorksQuery(() => filters)

	let selectedWorkId = $state<string | null>(null)
	let dialogOpen = $state(false)

	function openWork(work: Work) {
		selectedWorkId = work.id
		dialogOpen = true
	}

	function clearFilters() {
		params.reset()
	}

	$effect(() => {
		if (!dialogOpen) selectedWorkId = null
	})
</script>

<div class="mx-auto flex h-full max-w-[1600px] flex-col px-4 py-8 sm:px-6 lg:px-8">
	<div class="flex min-h-0 flex-1 gap-8">
		<WorksFilters
			bind:search={params.search}
			bind:departmentId={params.departmentId}
			bind:careerId={params.careerId}
			bind:yearFrom={params.yearFrom}
			bind:yearTo={params.yearTo}
			bind:journalKind={params.journalKind}
			bind:researchLineId={params.researchLineId}
			onClear={clearFilters}
		/>

		<main class="min-w-0 flex-1 overflow-y-auto">
			{#if worksQuery.isPending}
				<div class="flex items-center justify-center py-16">
					<Loader class="size-6 animate-spin text-corp-gray" />
				</div>
			{:else if worksQuery.isError}
				<div class="flex flex-col items-center justify-center py-16 text-center">
					<CircleAlert class="size-8 text-red-500" />
					<p class="mt-3 text-sm text-corp-gray">Error al cargar las publicaciones.</p>
				</div>
			{:else if !worksQuery.data || worksQuery.data.length === 0}
				<div class="flex flex-col items-center justify-center py-16 text-center">
					<div
						class="mb-3 flex size-12 items-center justify-center rounded-full bg-corp-blue/5"
					>
						<BookOpen class="size-5 text-corp-blue/60" />
					</div>
					<p class="text-sm text-[#1A1A1A]">No se encontraron publicaciones.</p>
					<p class="mt-1 max-w-sm text-xs text-corp-gray">
						Ajusta los filtros o sincroniza publicaciones desde la página de un
						académico con ORCID.
					</p>
				</div>
			{:else}
				<WorksTable works={worksQuery.data} onRowClick={openWork} />
			{/if}
		</main>
	</div>
</div>

<WorkDetailDialog bind:open={dialogOpen} bind:workId={selectedWorkId} />
