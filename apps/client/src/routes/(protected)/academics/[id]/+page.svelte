<script lang="ts">
	import type { Degree } from "$degrees/entity"

	import { page } from "$app/state"
	import {
		GraduationCap,
		Briefcase,
		BookOpen,
		Loader,
		CircleAlert,
		Pencil,
		Plus,
	} from "@lucide/svelte"

	import { useQuery } from "$shared/http/tanstack"
	import { authStore } from "$lib/auth/store.svelte"
	import { CLf64Value } from "$shared/value-objects/cl-f64.value"
	import { degreeService } from "$degrees/service"
	import { academicService } from "$academics/service"
	import { DegreeKindValue } from "$degrees/value-objects/kind.value"

	import Badge from "$shared/components/ui/badge.svelte"
	import DegreeDialog from "$degrees/components/degree-dialog.svelte"
	import WorksSection from "$works/components/works-section.svelte"
	import AcademicSidebar from "$academics/components/academic-sidebar.svelte"
	import AcademicEditDialog from "$academics/components/academic-edit-dialog.svelte"

	const id = $derived(page.params.id ?? "")

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

	let showDegreeDialog = $state(false)
	let editingDegree = $state<Degree | null>(null)
	let createKind = $state<(typeof DegreeKindValue.KINDS)[number]>("base")

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
	let activeTab = $state<"publications" | "academic-info">("academic-info")

	const isAdmin = $derived(authStore.isAuthenticated())

	function closeEditAcademic() {
		showEditAcademicDialog = false
	}

	function toggleTab() {
		activeTab = activeTab === "academic-info" ? "publications" : "academic-info"
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
					bind:activeTab
					onToggleTab={toggleTab}
					onEdit={() => (showEditAcademicDialog = true)}
				/>

				<div class="flex h-[calc(100dvh-10rem)] flex-col">
					<div class="mb-4 flex shrink-0 rounded-lg bg-corp-gray/10 p-1">
						<button
							type="button"
							class="flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors {activeTab ===
							'academic-info'
								? 'bg-white text-corp-blue shadow-sm'
								: 'text-corp-gray hover:text-[#1a1a1a]'}"
							onclick={() => (activeTab = "academic-info")}
						>
							Información Académica
						</button>
						<button
							type="button"
							class="flex-1 rounded-md px-3 py-1.5 text-xs font-semibold transition-colors {activeTab ===
							'publications'
								? 'bg-white text-corp-blue shadow-sm'
								: 'text-corp-gray hover:text-[#1a1a1a]'}"
							onclick={() => (activeTab = "publications")}
						>
							Publicaciones
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
											{CLf64Value.format(academic.jce.number)}
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
															: slot.kind.code === 'base'
																? 'bg-corp-blue'
																: 'bg-corp-yellow'}"
													></div>
													{#if i < degreeSlots.length - 1}
														<div
															class="mt-1 w-px grow bg-corp-gray/20"
														></div>
													{/if}
												</div>
												<div class="min-w-0 flex-1">
													<div class="mb-1 flex items-center gap-2">
														<Badge
															variant={slot.kind === "base"
																? "base"
																: "advanced"}
														>
															{slot.kind === "base"
																? "Título Profesional"
																: "Grado Académico"}
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
						{:else}
							<WorksSection {academic} />
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

{#if isAdmin && academic}
	<AcademicEditDialog {academic} bind:open={showEditAcademicDialog} onClose={closeEditAcademic} />
{/if}
