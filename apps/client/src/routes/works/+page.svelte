<script lang="ts">
	import { AlertCircle, BookOpen, Loader2 } from "@lucide/svelte"
	import * as v from "valibot"
	import { useSearchParams } from "runed/kit"

	import WorkDetailDialog from "$lib/research/components/work-detail-dialog.svelte"
	import WorksFilters from "$lib/research/components/works-filters.svelte"
	import WorksTable from "$lib/research/components/works-table.svelte"
	import { useWorksQuery } from "$lib/research/queries"
	import type { GetWorksParams, Work, WorkType } from "$lib/research/types"

	const schema = v.object({
		search: v.optional(v.fallback(v.string(), ""), ""),
		type: v.optional(v.fallback(v.string(), ""), ""),
		domain_id: v.optional(v.fallback(v.string(), ""), ""),
		field_id: v.optional(v.fallback(v.string(), ""), ""),
		subfield_id: v.optional(v.fallback(v.string(), ""), ""),
		topic_id: v.optional(v.fallback(v.string(), ""), ""),
		topic_min_score: v.optional(v.fallback(v.string(), ""), ""),
		keyword_id: v.optional(v.fallback(v.string(), ""), ""),
		department_id: v.optional(v.fallback(v.string(), ""), ""),
		career_id: v.optional(v.fallback(v.string(), ""), ""),
		year_from: v.optional(v.fallback(v.string(), ""), ""),
		year_to: v.optional(v.fallback(v.string(), ""), ""),
	})

	const params = useSearchParams(schema, { debounce: 300, pushHistory: false })

	// Cascade: clear children when parent changes
	$effect(() => {
		if (!params.domain_id) {
			params.field_id = ""
			params.subfield_id = ""
			params.topic_id = ""
		}
	})
	$effect(() => {
		if (!params.field_id) {
			params.subfield_id = ""
			params.topic_id = ""
		}
	})
	$effect(() => {
		if (!params.subfield_id) params.topic_id = ""
	})
	$effect(() => {
		if (!params.department_id) params.career_id = ""
	})

	let filters = $derived<GetWorksParams>({
		size: 100,
		yearFrom: params.year_from ? Number(params.year_from) : new Date().getFullYear() - 5,
		...(params.search && { search: params.search }),
		...(params.type && { type: [params.type as WorkType] }),
		...(params.domain_id && { domainId: params.domain_id }),
		...(params.field_id && { fieldId: params.field_id }),
		...(params.subfield_id && { subfieldId: params.subfield_id }),
		...(params.topic_id && { topicId: params.topic_id }),
		...(params.topic_min_score && { topicMinScore: Number(params.topic_min_score) }),
		...(params.keyword_id && { keywordId: params.keyword_id }),
		...(params.department_id && { departmentId: params.department_id }),
		...(params.career_id && { careerId: params.career_id }),
		...(params.year_to && { yearTo: Number(params.year_to) }),
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
</script>

<div class="mx-auto flex h-full max-w-[1600px] flex-col px-4 py-8 sm:px-6 lg:px-8">
	<header class="mb-6 shrink-0">
		<div class="flex items-center gap-3">
			<div class="flex size-10 items-center justify-center rounded-xl bg-corp-blue/10">
				<BookOpen class="size-5 text-corp-blue" />
			</div>
			<div>
				<h1 class="text-lg font-semibold text-[#1A1A1A]">Publicaciones</h1>
				<p class="text-sm text-corp-gray">
					Catálogo global de obras académicas importadas desde OpenAlex.
				</p>
			</div>
		</div>
	</header>

	<div class="flex min-h-0 flex-1 gap-8">
		<WorksFilters
			bind:search={params.search}
			bind:type={params.type}
			bind:domainId={params.domain_id}
			bind:fieldId={params.field_id}
			bind:subfieldId={params.subfield_id}
			bind:topicId={params.topic_id}
			bind:topicMinScore={params.topic_min_score}
			bind:keywordId={params.keyword_id}
			bind:departmentId={params.department_id}
			bind:careerId={params.career_id}
			bind:yearFrom={params.year_from}
			bind:yearTo={params.year_to}
			onClear={clearFilters}
		/>

		<main class="min-w-0 flex-1 overflow-y-auto">
			{#if worksQuery.isPending}
				<div class="flex items-center justify-center py-16">
					<Loader2 class="size-6 animate-spin text-corp-gray" />
				</div>
			{:else if worksQuery.isError}
				<div class="flex flex-col items-center justify-center py-16 text-center">
					<AlertCircle class="size-8 text-red-500" />
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
