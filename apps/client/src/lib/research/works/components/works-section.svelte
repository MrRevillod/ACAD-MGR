<script lang="ts">
	import type { Work } from "$works/entity"
	import type { Academic } from "$academics/entity"

	import { useWorksByAcademicQuery } from "$works/queries"
	import { CircleAlert, BookOpen, Loader } from "@lucide/svelte"

	import YearRange from "$shared/components/ui/year-range.svelte"
	import WorksTable from "./works-table.svelte"
	import SyncWorksButton from "./sync-works-button.svelte"
	import WorkDetailDialog from "./work-detail-dialog.svelte"

	interface Props {
		academic: Academic
		yearFrom?: string
		yearTo?: string
	}

	let { academic, yearFrom = $bindable(""), yearTo = $bindable("") }: Props = $props()

	function worksParams() {
		return {
			...(yearFrom && { yearFrom: Number(yearFrom) }),
			...(yearTo && { yearTo: Number(yearTo) }),
		}
	}

	const worksQuery = useWorksByAcademicQuery(() => academic.id, worksParams)

	let selectedWorkId = $state<string | null>(null)
	let dialogOpen = $state(false)

	function openWork(work: Work) {
		selectedWorkId = work.id
		dialogOpen = true
	}
</script>

<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
	<div class="mb-6 flex items-center justify-between gap-3">
		<div
			class="flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
		>
			<BookOpen class="size-4 text-corp-blue" />
			Publicaciones
			{#if worksQuery.data}
				<span
					class="rounded-full bg-corp-gray/10 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-corp-gray tabular-nums"
				>
					{worksQuery.data.length}
				</span>
			{/if}
		</div>
		<div class="flex items-center gap-3">
			<YearRange
				bind:yearFrom
				bind:yearTo
				showLabels={false}
				placeholderFrom="DESDE"
				placeholderTo="HASTA"
				minYear={1900}
			/>
			<SyncWorksButton academicId={academic.id} orcid={academic.orcid ?? null} />
		</div>
	</div>

	{#if worksQuery.isPending}
		<div class="flex items-center justify-center py-8">
			<Loader class="size-5 animate-spin text-corp-gray" />
		</div>
	{:else if worksQuery.isError}
		<div class="flex flex-col items-center py-8 text-center">
			<CircleAlert class="size-6 text-red-500" />
			<p class="mt-2 text-sm text-corp-gray">Error al cargar las publicaciones.</p>
		</div>
	{:else if !worksQuery.data || worksQuery.data.length === 0}
		<div class="flex flex-col items-center py-10 text-center">
			<div class="mb-3 flex size-12 items-center justify-center rounded-full bg-corp-blue/5">
				<BookOpen class="size-5 text-corp-blue/60" />
			</div>
			<p class="text-sm text-[#1A1A1A]">No hay publicaciones sincronizadas.</p>
			<p class="mt-1 max-w-sm text-xs text-corp-gray">
				{#if academic.orcid}
					Usa el botón "Sincronizar Publicaciones" para importar las publicaciones de este
					académico.
				{:else}
					Este académico no tiene ORCID asociado, por lo que no se pueden importar
					publicaciones automáticamente.
				{/if}
			</p>
		</div>
	{:else}
		<WorksTable works={worksQuery.data} onRowClick={openWork} />
	{/if}
</section>

<WorkDetailDialog bind:open={dialogOpen} bind:workId={selectedWorkId} />
