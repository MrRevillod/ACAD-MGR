<script lang="ts">
	import { BookOpen, CircleAlert, Loader } from "@lucide/svelte"

	import { useWorksByAcademicQuery } from "$works/queries"

	interface Props {
		academicId: string
		selectedWorkId?: string | null
		onSelect: (workId: string) => void
	}

	let { academicId, selectedWorkId = null, onSelect }: Props = $props()

	const worksQuery = useWorksByAcademicQuery(() => academicId)
</script>

<div
	class="flex h-full flex-col overflow-hidden rounded-xl border border-corp-gray/20 bg-white lg:sticky lg:top-8 lg:max-h-[calc(100dvh-7rem)]"
>
	<div class="flex shrink-0 items-center gap-2 border-b border-corp-gray/10 px-4 py-3">
		<BookOpen class="size-4 text-corp-blue" />
		<span class="text-xs font-semibold tracking-widest uppercase text-corp-blue">
			Publicaciones
		</span>
		{#if worksQuery.data}
			<span
				class="rounded-full bg-corp-gray/10 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-corp-gray tabular-nums"
			>
				{worksQuery.data.length}
			</span>
		{/if}
	</div>

	<div class="min-h-0 flex-1 overflow-y-auto p-2">
		{#if worksQuery.isPending}
			<div class="flex justify-center py-10">
				<Loader class="size-5 animate-spin text-corp-gray" />
			</div>
		{:else if worksQuery.isError}
			<div class="flex flex-col items-center px-4 py-10 text-center">
				<CircleAlert class="size-6 text-red-500" />
				<p class="mt-2 text-sm text-corp-gray">Error al cargar las publicaciones.</p>
			</div>
		{:else if !worksQuery.data || worksQuery.data.length === 0}
			<div class="flex flex-col items-center px-4 py-10 text-center">
				<p class="text-sm text-corp-gray">No se encontraron publicaciones para editar.</p>
			</div>
		{:else}
			<div class="space-y-1.5">
				{#each worksQuery.data as work (work.id)}
					<button
						type="button"
						onclick={() => onSelect(work.id)}
						aria-pressed={selectedWorkId === work.id}
						class="w-full rounded-lg border px-3 py-2.5 text-left transition-colors active:scale-[0.96] focus-visible:ring-2 focus-visible:ring-corp-blue/30 {selectedWorkId ===
						work.id
							? 'border-corp-blue/40 bg-corp-blue/5'
							: 'border-transparent hover:bg-corp-gray/5'}"
					>
						<p class="line-clamp-2 text-[13px] leading-snug font-medium text-[#1A1A1A]">
							{work.title}
						</p>
						{#if work.publicationYear}
							<p class="mt-1 text-xs text-corp-gray tabular-nums">
								{work.publicationYear}
							</p>
						{/if}
					</button>
				{/each}
			</div>
		{/if}
	</div>
</div>
