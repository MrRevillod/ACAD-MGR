<script lang="ts">
	import * as v from "valibot"
	import type { Degree } from "$degrees/entity"

	import { page } from "$app/state"
	import { toast } from "svelte-sonner"
	import { useSearchParams } from "runed/kit"
	import {
		GraduationCap,
		Briefcase,
		BookOpen,
		Loader,
		CircleAlert,
		Pencil,
		Plus,
		Network,
		Info,
	} from "@lucide/svelte"

	import { useQuery, useMutation } from "$shared/http/tanstack"
	import { authStore } from "$lib/auth/store.svelte"
	import { CLf64Value } from "$shared/value-objects/cl-f64.value"
	import { degreeService } from "$degrees/service"
	import { academicService } from "$academics/service"
	import { DegreeKindValue } from "$degrees/value-objects/kind.value"

	import Badge from "$shared/components/ui/badge.svelte"
	import DegreeDialog from "$degrees/components/degree-dialog.svelte"
	import WorksSection from "$works/components/works-section.svelte"
	import AcademicStats from "$stats/components/academic-stats.svelte"
	import AcademicSidebar from "$academics/components/academic-sidebar.svelte"
	import AcademicEditDialog from "$academics/components/academic-edit-dialog.svelte"
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
			v.fallback(
				v.picklist(["academic-info", "publications", "stats", "collaborations"]),
				"academic-info",
			),
			"academic-info",
		),
	})

	const tabParams = useSearchParams(tabParamsSchema, { pushHistory: true })
	const activeTab = $derived(tabParams.tab)

	const academicQuery = useQuery(() => ({
		queryKey: ["academic", id],
		queryFn: () => academicService.get(id),
		enabled: Boolean(id),
	}))

	const degreesQuery = useQuery(() => ({
		queryKey: ["degrees", id],
		queryFn: () => degreeService.listByAcademic(id),
		enabled: Boolean(id),
	}))

	const academic = $derived(academicQuery.data)
	const degreeSlots = $derived.by<
		Array<
			| (Degree & { isPlaceholder: false })
			| { kind: (typeof DegreeKindValue.KINDS)[number]; isPlaceholder: true }
		>
	>(() =>
		DegreeKindValue.KINDS.map((kind) => {
			const found = (degreesQuery.data ?? []).find((d) => d.kind.code === kind)
			return found
				? { ...found, isPlaceholder: false as const }
				: { kind, isPlaceholder: true as const }
		}),
	)

	const degreeKindMeta: Record<
		string,
		{ label: string; badge: "base" | "advanced" | "doctor"; dot: string }
	> = {
		professional: {
			label: DegreeKindValue.LABELS.professional,
			badge: "base",
			dot: "bg-corp-blue",
		},
		magister: {
			label: DegreeKindValue.LABELS.magister,
			badge: "advanced",
			dot: "bg-corp-yellow",
		},
		doctor: {
			label: DegreeKindValue.LABELS.doctor,
			badge: "doctor",
			dot: "bg-corp-gold",
		},
	}

	let showDegreeDialog = $state(false)
	let editingDegree = $state<Degree | null>(null)
	let createKind = $state<(typeof DegreeKindValue.KINDS)[number]>("professional")

	function openCreate(k: (typeof DegreeKindValue.KINDS)[number]) {
		editingDegree = null
		createKind = k
		showDegreeDialog = true
	}

	function openEdit(deg: Degree) {
		editingDegree = deg
		showDegreeDialog = true
	}

	let showEditAcademicDialog = $state(false)
	let showGraphHelp = $state(false)

	const isAdmin = $derived(authStore.isAuthenticated)

	const sendEditCodesMutation = useMutation(() => ({
		mutationFn: () => academicService.sendEditCodes(id),
		onSuccess: () => toast.success("Códigos enviados al correo del académico"),
		onError: () => toast.error("Error al enviar los códigos de edición"),
	}))

	function handleSendEditCodes() {
		if (!confirm("¿Enviar códigos de edición al académico?")) return
		sendEditCodesMutation.mutate()
	}

	function closeEditAcademic() {
		showEditAcademicDialog = false
	}
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
			<a href="/academics" class="mt-4 text-sm font-medium text-corp-blue hover:underline"
				>Volver al listado</a
			>
		</div>
	{:else}
		<div class="mx-auto max-w-[1600px] px-4 py-8 sm:px-6 lg:px-8">
			<div class="grid grid-cols-1 gap-6 lg:grid-cols-[320px_1fr]">
				<AcademicSidebar
					{academic}
					onEdit={() => (showEditAcademicDialog = true)}
					onSendCodes={handleSendEditCodes}
				/>

				<div class="flex h-[calc(100dvh-10rem)] flex-col">
					<div class="mb-4 flex shrink-0 rounded-lg bg-corp-gray/10 p-1">
						<button
							type="button"
							class="flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors {activeTab ===
							'academic-info'
								? 'bg-white text-corp-blue shadow-sm'
								: 'text-corp-gray hover:text-[#1a1a1a]'}"
							onclick={() => (tabParams.tab = "academic-info")}
						>
							Información Académica
						</button>
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
						{#if activeTab === "academic-info"}
							<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
								<div
									class="mb-5 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
								>
									<Briefcase class="size-4 text-corp-blue" />
									Información Laboral
								</div>
								<div class="grid grid-cols-1 gap-x-8 gap-y-4 sm:grid-cols-3">
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Departamento
										</p>
										<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
											{academic.department}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Carrera
										</p>
										<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
											{academic.career ?? "—"}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Ingreso
										</p>
										<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
											{academic.joinedAt.toDisplayDate()}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Cargo
										</p>
										<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
											{academic.workPosition ?? "—"}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Jornada Completa Equivalente
										</p>
										<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
											{CLf64Value.format(academic.jce.number)} horas
										</p>
									</div>
								</div>
							</section>

							<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
								<div
									class="mb-5 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
								>
									<BookOpen class="size-4 text-corp-blue" />
									Categorización Académica
								</div>
								<div class="grid grid-cols-1 gap-x-8 gap-y-4 sm:grid-cols-3">
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Planta
										</p>
										<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
											{academic.planta.toDisplay()}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Categoría
										</p>
										<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
											{academic.category}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Opción
										</p>
										<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
											{academic.option.toDisplay()}
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Horas de categoría y opción
										</p>
										<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
											{academic.acadCategoryHours?.toLocaleString("es-CL") ??
												"—"} horas
										</p>
									</div>
									<div>
										<p
											class="text-xs font-medium tracking-wide uppercase text-corp-gray"
										>
											Descuento anual
										</p>
										<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
											{CLf64Value.format(academic.annualDiscountHours)} horas
										</p>
									</div>
								</div>
							</section>

							<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
								<div
									class="mb-6 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
								>
									<GraduationCap class="size-4 text-corp-blue" />
									Grados Académicos
								</div>

								{#if degreesQuery.isPending}
									<div class="flex items-center justify-center py-8">
										<Loader class="size-5 animate-spin text-corp-gray" />
									</div>
								{:else}
									<div class="relative">
										{#each degreeSlots as slot, i (slot.kind)}
											{@const meta =
												degreeKindMeta[
													slot.isPlaceholder ? slot.kind : slot.kind.code
												]}
											<div
												class="relative flex gap-5 {i <
												degreeSlots.length - 1
													? 'pb-8'
													: ''}"
											>
												<div class="flex flex-col items-center">
													<div
														class="z-10 size-3 shrink-0 rounded-full {slot.isPlaceholder
															? 'bg-corp-gray/30'
															: meta.dot}"
													></div>
													{#if i < degreeSlots.length - 1}
														<div
															class="mt-1 w-px grow bg-corp-gray/20"
														></div>
													{/if}
												</div>
												<div class="min-w-0 flex-1">
													<div class="mb-1 flex items-center gap-2">
														<Badge variant={meta.badge}>
															{meta.label}
														</Badge>
														{#if !slot.isPlaceholder && isAdmin}
															<button
																class="flex size-6 items-center justify-center rounded-md text-corp-gray/40 transition-colors hover:text-corp-blue"
																onclick={() => openEdit(slot)}
															>
																<Pencil class="size-3.5" />
															</button>
														{/if}
													</div>
													{#if slot.isPlaceholder && isAdmin}
														<button
															class="mt-1 inline-flex items-center gap-1.5 text-sm text-corp-gray/50 transition-colors hover:text-corp-blue"
															onclick={() => openCreate(slot.kind)}
														>
															<Plus class="size-3.5" />
															Agregar
														</button>
													{:else}
														{@const degree = slot as Degree}
														<p
															class="text-[15px] font-medium text-[#1a1a1a]"
														>
															{degree.name}
														</p>
														<p class="mt-1 text-sm text-corp-gray">
															{degree.university}
															<span class="mx-1.5 text-corp-gray/40"
																>·</span
															>
															{degree.country.toDisplay()}
															<span class="mx-1.5 text-corp-gray/40"
																>·</span
															>
															{degree.obtainedAt.toDisplayDate()}
														</p>
													{/if}
												</div>
											</div>
										{/each}
									</div>
								{/if}
							</section>
						{:else if activeTab === "collaborations"}
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
								bind:yearFrom={yearParams.yearFrom}
								bind:yearTo={yearParams.yearTo}
							/>
						{/if}
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<DegreeDialog
	academicId={id}
	degree={editingDegree}
	{createKind}
	bind:open={showDegreeDialog}
	onClose={() => (showDegreeDialog = false)}
/>

<CollaborationGraphHelpDialog bind:open={showGraphHelp} />

{#if isAdmin && academic}
	<AcademicEditDialog {academic} bind:open={showEditAcademicDialog} onClose={closeEditAcademic} />
{/if}
