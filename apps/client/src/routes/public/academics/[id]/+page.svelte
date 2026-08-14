<script lang="ts">
	import * as v from "valibot"

	import { page } from "$app/state"
	import { useQuery } from "$shared/http/tanstack"
	import { useSearchParams } from "runed/kit"
	import { academicService } from "$academics/service"
	import { Loader, CircleAlert, Construction, Network, Info } from "@lucide/svelte"

	import Dialog from "$shared/components/ui/dialog.svelte"
	import Button from "$shared/components/ui/button.svelte"
	import WorksSection from "$works/components/works-section.svelte"
	import AcademicStats from "$stats/components/academic-stats.svelte"
	import AcademicSidebar from "$academics/components/academic-sidebar.svelte"
	import CollaborationGraph from "$collaborations/components/collaboration-graph.svelte"
	import CollaborationGraphHelpDialog from "$collaborations/components/collaboration-graph-help-dialog.svelte"

	const id = $derived(page.params.id ?? "")

	const yearFromDefault = String(new Date().getFullYear() - 5)

	const yearParamsSchema = v.object({
		yearFrom: v.optional(v.fallback(v.string(), yearFromDefault), yearFromDefault),
		yearTo: v.optional(v.fallback(v.string(), ""), ""),
	})

	const yearParams = useSearchParams(yearParamsSchema, { debounce: 300, pushHistory: false })

	const tabParamsSchema = v.object({
		tab: v.optional(
			v.fallback(v.picklist(["publications", "stats", "collaborations"]), "publications"),
			"publications",
		),
	})

	const tabParams = useSearchParams(tabParamsSchema, { pushHistory: true })
	const activeTab = $derived(tabParams.tab)

	const academicQuery = useQuery(() => ({
		queryKey: ["public-academic", id],
		queryFn: () => academicService.getPublic(id),
		enabled: Boolean(id),
	}))

	const academic = $derived(academicQuery.data)

	let requestEditDialogOpen = $state(false)
	let showGraphHelp = $state(false)
	/* -- disabled temporarily --
	let isRequesting = $state(false)
	let requestSent = $state(false)

	async function handleRequestEdit() {
		isRequesting = true
		try {
			await academicService.requestProfileUpdate(id)
			requestSent = true
			toast.success("Enlace enviado a tu correo electrónico")
		} catch {
			toast.error("Error al solicitar la edición del perfil")
		} finally {
			isRequesting = false
		}
	}
	*/
</script>

<div class="h-full overflow-y-auto">
	{#if academicQuery.isPending}
		<div class="flex h-full items-center justify-center">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if academicQuery.isError || !academic}
		<div class="flex h-full flex-col items-center justify-center text-center">
			<CircleAlert class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Académico no encontrado.</p>
		</div>
	{:else}
		<div class="mx-auto max-w-[1600px] px-4 py-8 sm:px-6 lg:px-8">
			<div class="grid grid-cols-1 gap-6 lg:grid-cols-[320px_1fr]">
				<AcademicSidebar
					{academic}
					readonly
					onRequestEdit={() => (requestEditDialogOpen = true)}
				/>
				<div class="flex h-[calc(100dvh-10rem)] flex-col">
					<div class="mb-4 flex shrink-0 rounded-lg bg-corp-gray/10 p-1">
						<button
							type="button"
							class="flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors {activeTab ===
							'publications'
								? 'bg-white text-corp-blue shadow-sm'
								: 'text-corp-gray hover:text-[#1a1a1a]'}"
							onclick={() => (tabParams.tab = "publications")}
						>
							Publicaciones
						</button>
						<button
							type="button"
							class="flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors {activeTab ===
							'stats'
								? 'bg-white text-corp-blue shadow-sm'
								: 'text-corp-gray hover:text-[#1a1a1a]'}"
							onclick={() => (tabParams.tab = "stats")}
						>
							Estadísticas
						</button>
						<button
							type="button"
							class="flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors {activeTab ===
							'collaborations'
								? 'bg-white text-corp-blue shadow-sm'
								: 'text-corp-gray hover:text-[#1a1a1a]'}"
							onclick={() => (tabParams.tab = "collaborations")}
						>
							Colaboraciones
						</button>
					</div>
					<div class="min-h-0 flex-1 space-y-6 overflow-y-auto">
						{#if activeTab === "collaborations"}
							<section
								class="flex h-full min-h-0 flex-col overflow-hidden rounded-xl border border-corp-gray/20 bg-white"
							>
								<div
									class="flex shrink-0 items-center justify-between gap-2 border-b border-corp-gray/10 px-6 py-4"
								>
									<div
										class="flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
									>
										<Network class="size-4 text-corp-blue" />
										Red de colaboración
									</div>
									<button
										type="button"
										title="Ayuda: cómo leer esta red"
										class="flex size-7 items-center justify-center rounded-full text-corp-gray transition-colors hover:bg-corp-gray/10 hover:text-[#1A1A1A]"
										onclick={() => (showGraphHelp = true)}
									>
										<Info class="size-4" />
									</button>
								</div>
								<div class="min-h-0 flex-1">
									<CollaborationGraph {academic} />
								</div>
							</section>
						{:else if activeTab === "stats"}
							<AcademicStats academicId={id} />
						{:else}
							<WorksSection
								{academic}
								readonly
								bind:yearFrom={yearParams.yearFrom}
								bind:yearTo={yearParams.yearTo}
							/>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}

	<Dialog
		bind:open={requestEditDialogOpen}
		title="Solicitar edición de perfil"
		description="Esta funcionalidad no está disponible temporalmente."
	>
		<div class="flex flex-col items-center py-4 text-center">
			<Construction class="size-10 text-amber-500" />
			<p class="mt-3 text-sm text-corp-gray">
				Esta funcionalidad estará disponible próximamente.
			</p>
		</div>

		<div class="flex justify-end gap-2">
			<Button variant="secondary" onclick={() => (requestEditDialogOpen = false)}>
				Cerrar
			</Button>
		</div>
	</Dialog>

	<CollaborationGraphHelpDialog bind:open={showGraphHelp} />
</div>
