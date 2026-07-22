<script lang="ts">
	import { createQuery, useQueryClient } from "@tanstack/svelte-query"
	import { Loader, Trash2, GripVertical } from "@lucide/svelte"
	import { toast } from "svelte-sonner"

	import { classificationService } from "$research/classification/service"

	const query = createQuery(() => ({
		queryKey: ["admin", "research-lines"],
		queryFn: () => classificationService.researchLineDetails(),
	}))

	const qc = useQueryClient()

	let draggingId = $state<string | null>(null)

	function handleDragStart(e: DragEvent, subfieldOpenalexId: string) {
		if (!e.dataTransfer) return
		e.dataTransfer.setData("text/plain", subfieldOpenalexId)
		e.dataTransfer.effectAllowed = "move"
		draggingId = subfieldOpenalexId
	}

	function handleDragEnd() {
		draggingId = null
	}

	function handleDragOver(e: DragEvent) {
		if (!e.dataTransfer) return
		e.preventDefault()
		e.dataTransfer.dropEffect = "move"
	}

	async function handleDrop(e: DragEvent, targetLineId: string) {
		e.preventDefault()
		if (!e.dataTransfer) return
		const subfieldId = e.dataTransfer.getData("text/plain")
		if (!subfieldId || subfieldId === "unknown") return

		try {
			await classificationService.updateMapping(subfieldId, targetLineId)
			toast.success("Subfield movido correctamente")
			void qc.invalidateQueries({ queryKey: ["admin", "research-lines"] })
		} catch {
			toast.error("Error al mover el subfield")
		}

		draggingId = null
	}

	async function handleDelete(lineId: string, subfieldOpenalexId: string) {
		const sinAsignar = query.data?.lines.find((l) => l.slug === "sin-asignar")
		if (!sinAsignar) return
		if (lineId === sinAsignar.id) return

		try {
			await classificationService.updateMapping(subfieldOpenalexId, sinAsignar.id)
			toast.success("Subfield movido a Sin Asignar")
			void qc.invalidateQueries({ queryKey: ["admin", "research-lines"] })
		} catch {
			toast.error("Error al eliminar el subfield")
		}
	}
</script>

<div class="flex h-full flex-col">
	<div class="mb-6">
		<h1 class="text-lg font-semibold text-[#1A1A1A]">Líneas de Investigación</h1>
		<p class="mt-1 text-sm text-corp-gray">
			Arrastra subfields entre líneas para reasignarlos.
		</p>
	</div>

	{#if query.isPending}
		<div class="flex min-h-0 flex-1 items-center justify-center">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if query.isError}
		<p class="min-h-0 flex-1 text-center text-sm text-red-500">
			Error al cargar las líneas de investigación.
		</p>
	{:else}
		<div
			class="flex min-h-0 flex-1 gap-4 overflow-x-auto pb-4"
			style="scrollbar-width: thin; scrollbar-color: oklch(0.87 0.01 258.34) transparent;"
		>
			{#each query.data?.lines ?? [] as line (line.id)}
				<div
					class="group/col flex w-72 shrink-0 flex-col rounded-xl border border-corp-gray/20 bg-white max-h-full"
				>
					<div class="border-b border-corp-gray/10 px-3 py-2.5">
						<div class="flex items-center justify-between">
							<h2 class="truncate text-sm font-semibold text-[#1A1A1A]">
								{line.name}
							</h2>
							<span
								class="shrink-0 rounded-full bg-corp-blue/10 px-2 py-0.5 text-[11px] font-semibold text-corp-blue tabular-nums"
							>
								{line.subfields.length}
							</span>
						</div>
					</div>

					<div
						role="region"
						class="flex-1 space-y-1.5 overflow-y-hidden group-hover/col:overflow-y-auto p-2 {draggingId
							? 'bg-corp-blue/[0.02]'
							: ''}"
						ondragover={handleDragOver}
						ondrop={(e) => handleDrop(e, line.id)}
					>
						{#each line.subfields as sf (sf.subfieldOpenalexId)}
							{@const isDragging = draggingId === sf.subfieldOpenalexId}
							<div
								role="listitem"
								tabindex="-1"
								class="group relative cursor-grab rounded-lg border border-corp-gray/15 bg-white px-3 py-2 text-sm transition-shadow hover:shadow-sm {isDragging
									? 'opacity-40 shadow-sm'
									: ''}"
								draggable={sf.subfieldOpenalexId !== "unknown"}
								ondragstart={(e) => handleDragStart(e, sf.subfieldOpenalexId)}
								ondragend={handleDragEnd}
							>
								<div class="flex items-start gap-1.5">
									<GripVertical
										class="mt-0.5 size-3 shrink-0 text-corp-gray/30 group-hover:text-corp-gray/60"
									/>
									<div class="min-w-0 flex-1">
										<p class="truncate text-[13px] font-medium text-[#1A1A1A]">
											{sf.subfieldName}
										</p>
									</div>
									{#if sf.subfieldOpenalexId !== "unknown" && line.slug !== "sin-asignar"}
										<button
											type="button"
											title="Mover a Sin Asignar"
											class="shrink-0 rounded p-0.5 text-corp-gray/40 opacity-0 transition-colors hover:text-red-500 group-hover:opacity-100"
											onclick={() =>
												handleDelete(line.id, sf.subfieldOpenalexId)}
										>
											<Trash2 class="size-3.5" />
										</button>
									{/if}
								</div>
							</div>
						{/each}

						{#if line.subfields.length === 0}
							<p class="pt-4 text-center text-xs text-corp-gray/50">Sin subfields</p>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
