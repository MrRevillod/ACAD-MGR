<script lang="ts">
	import type { Work } from "$works/entity"
	import type { Academic } from "$academics/entity"

	import { useWorksByAcademicQuery } from "$works/queries"
	import { useResearchLinesQuery } from "$research/classification/queries"
	import { CircleAlert, BookOpen, Loader } from "@lucide/svelte"
	import { goto } from "$app/navigation"

	import Label from "$shared/components/ui/label.svelte"
	import Select from "$shared/components/ui/select.svelte"
	import YearRange from "$shared/components/ui/year-range.svelte"
	import WorksTable from "./works-table.svelte"
	import SyncWorksButton from "./sync-works-button.svelte"
	import type { JournalKind } from "../value-objects/journal-kind.value"

	interface Props {
		academic: Academic
		yearFrom?: string
		yearTo?: string
		researchLineId?: string
		journalKind?: JournalKind
		readonly?: boolean
	}

	let {
		academic,
		yearFrom = $bindable(""),
		yearTo = $bindable(""),
		researchLineId = $bindable(""),
		journalKind = $bindable("" as JournalKind),
		readonly = false,
	}: Props = $props()

	const researchLinesQuery = useResearchLinesQuery()

	const researchLineItems = $derived([
		{ value: "", label: "Todas las líneas" },
		...(researchLinesQuery.data?.map((rl) => ({ value: rl.id, label: rl.name })) ?? []),
	])

	const journalKindItems = $derived([
		{ value: "", label: "Todas las clasificaciones" },
		{ value: "wos", label: "WoS" },
		{ value: "scopus", label: "Scopus" },
	])

	function worksParams() {
		return {
			...(yearFrom && { yearFrom: Number(yearFrom) }),
			...(yearTo && { yearTo: Number(yearTo) }),
			...(researchLineId && { researchLineId }),
			...(journalKind && { journalKind }),
		}
	}

	const worksQuery = useWorksByAcademicQuery(() => academic.id, worksParams)

	function openWork(work: Work) {
		void goto(`/works/${work.id}`)
	}
</script>

<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
	<div class="mb-4 flex flex-row gap-2 justify-between">
		<div class="space-y-2.5">
			<Label>Línea de investigación</Label>
			<Select items={researchLineItems} bind:value={researchLineId} class="min-w-40" />
		</div>
		<div class="space-y-2.5">
			<Label>Indexación</Label>
			<Select items={journalKindItems} bind:value={journalKind} class="min-w-52" />
		</div>
		<YearRange
			bind:yearFrom
			bind:yearTo
			label="Rango anual de publicación"
			showLabels={false}
			placeholderFrom="DESDE"
			placeholderTo="HASTA"
			minYear={1900}
			class="min-w-72"
		/>
		{#if !readonly}
			<div class="space-y-2.5 mt-5">
				<Label>{null}</Label>
				<SyncWorksButton academicId={academic.id} orcid={academic.orcid ?? null} />
			</div>
		{/if}
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
			{#if readonly}
				<p class="text-sm text-[#1A1A1A]">
					Este académico no tiene publicaciones disponibles para mostrar.
				</p>
			{:else}
				<p class="text-sm text-[#1A1A1A]">No hay publicaciones sincronizadas.</p>
				<p class="mt-1 max-w-sm text-xs text-corp-gray">
					{#if academic.orcid}
						Usa el botón "Sincronizar Publicaciones" para importar las publicaciones de
						este académico.
					{:else}
						Este académico no tiene ORCID asociado, por lo que no se pueden importar
						publicaciones automáticamente.
					{/if}
				</p>
			{/if}
		</div>
	{:else}
		<WorksTable works={worksQuery.data} onRowClick={openWork} pageSize={7} />
	{/if}
</section>
