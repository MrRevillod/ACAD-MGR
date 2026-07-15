<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { page } from "$app/state"
	import {
		ChevronLeft,
		GraduationCap,
		Briefcase,
		BookOpen,
		Loader2,
		AlertCircle,
		Pencil,
		Plus,
		ExternalLink,
	} from "@lucide/svelte"

	import { authStore } from "$lib/auth/store.svelte"
	import { academicService } from "$academics/service"
	import { degreeService } from "$degrees/service"
	import DegreeDialog from "$degrees/components/degree-dialog.svelte"
	import AcademicEditDialog from "$academics/components/academic-edit-dialog.svelte"
	import WorksSection from "$lib/research/components/works-section.svelte"
	import Badge from "$shared/components/ui/badge.svelte"
	import { CLf64Value } from "$shared/value-objects/cl-f64.value"
	import { DateValue } from "$shared/value-objects/date.value"
	import { CountryValue } from "$shared/value-objects/country.value"
	import { DEGREE_KIND } from "$degrees/enums"
	import type { Degree } from "$degrees/dtos"

	const id = $derived(page.params.id ?? "")

	const academicQuery = createQuery(() => ({
		queryKey: ["academic", id],
		queryFn: () => academicService.get(id),
		enabled: Boolean(id),
	}))

	const degreesQuery = createQuery(() => ({
		queryKey: ["degrees", id],
		queryFn: () => degreeService.listByAcademic(id),
		enabled: Boolean(id),
	}))

	const academic = $derived(academicQuery.data)
	const degreeSlots = $derived.by<
		Array<
			| (Degree & { isPlaceholder: false })
			| { kind: (typeof DEGREE_KIND)[number]; isPlaceholder: true }
		>
	>(() =>
		DEGREE_KIND.map((kind) => {
			const found = (degreesQuery.data ?? []).find((d) => d.kind === kind)
			return found
				? { ...found, isPlaceholder: false as const }
				: { kind, isPlaceholder: true as const }
		}),
	)

	const fullName = $derived(
		academic ? `${academic.names} ${academic.paternalSurname} ${academic.maternalSurname}` : "",
	)

	const initials = $derived(
		academic
			? (academic.names.charAt(0) + academic.paternalSurname.charAt(0)).toUpperCase()
			: "",
	)

	let showDegreeDialog = $state(false)
	let editingDegree = $state<Degree | null>(null)
	let createKind = $state<(typeof DEGREE_KIND)[number]>("base")

	function openCreate(k: (typeof DEGREE_KIND)[number]) {
		editingDegree = null
		createKind = k
		showDegreeDialog = true
	}

	function openEdit(deg: Degree) {
		editingDegree = deg
		showDegreeDialog = true
	}

	const isAdmin = $derived(authStore.isAuthenticated())

	let showEditAcademicDialog = $state(false)
	let activeTab = $state<"publications" | "academic-info">("academic-info")

	function closeEditAcademic() {
		showEditAcademicDialog = false
	}
</script>

<div class="h-full overflow-y-auto">
	{#if academicQuery.isPending}
		<div class="flex h-full items-center justify-center">
			<Loader2 class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if academicQuery.isError || !academic}
		<div class="flex h-full flex-col items-center justify-center text-center">
			<AlertCircle class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Académico no encontrado.</p>
			<a href="/academics" class="mt-4 text-sm font-medium text-corp-blue hover:underline"
				>Volver al listado</a
			>
		</div>
	{:else}
		<div class="mx-auto max-w-[1600px] px-4 py-8 sm:px-6 lg:px-8">
			<div class="grid grid-cols-1 gap-6 lg:grid-cols-[320px_1fr]">
				<aside class="relative overflow-hidden rounded-xl bg-corp-blue text-white">
					<button
						class="absolute left-3 top-3 z-10 flex size-8 items-center justify-center rounded-full bg-white text-corp-blue shadow-sm active:scale-95"
						onclick={() => window.history.back()}
					>
						<ChevronLeft class="size-4" />
					</button>
					{#if isAdmin}
						<button
							class="absolute right-3 top-3 z-10 flex size-8 items-center justify-center rounded-full bg-white text-corp-blue shadow-sm active:scale-95"
							onclick={() => (showEditAcademicDialog = true)}
						>
							<Pencil class="size-4" />
						</button>
					{/if}
					<div class="p-6 pb-4 text-center">
						<div
							class="mx-auto mb-4 flex size-24 items-center justify-center rounded-full bg-white/10 text-2xl font-bold tracking-widest text-white ring-2 ring-white/15"
						>
							{initials}
						</div>
						<h1 class="text-lg font-semibold leading-snug">{fullName}</h1>
						<p class="mt-1.5 text-sm text-white/60">{academic.email}</p>
					</div>

					<div class="border-t border-white/10 px-6 py-4">
						<div class="space-y-4">
							<div>
								<p
									class="text-[11px] font-medium tracking-wide uppercase text-white/50"
								>
									País de Nacionalidad
								</p>
								<p class="mt-1 text-sm font-semibold text-white">
									{academic.nationality.toDisplay()}
								</p>
							</div>
							<div>
								<p
									class="text-[11px] font-medium tracking-wide uppercase text-white/50"
								>
									Ciudad de Residencia
								</p>
								<p class="mt-1 text-sm font-semibold text-white">{academic.city}</p>
							</div>
							<div>
								<p
									class="text-[11px] font-medium tracking-wide uppercase text-white/50"
								>
									Fecha de Nacimiento
								</p>
								<p class="mt-1 text-sm font-semibold text-white">
									{DateValue.formatDate(academic.birthDate.iso)}
								</p>
							</div>
							<div>
								<p
									class="text-[11px] font-medium tracking-wide uppercase text-white/50"
								>
									Sexo
								</p>
								<p class="mt-1 text-sm font-semibold text-white">
									{academic.sex.toDisplay()}
								</p>
							</div>
							{#if academic.orcid}
								<a
									href={academic.orcid}
									target="_blank"
									rel="noopener"
									class="flex items-center gap-3 text-sm transition-colors hover:text-white/80"
								>
									<ExternalLink class="size-4 shrink-0 text-corp-yellow" />
									<div class="min-w-0">
										<p class="truncate text-white/90">{academic.orcid}</p>
										<p class="text-xs text-white/60">ORCID</p>
									</div>
								</a>
							{/if}
						</div>
					</div>

					<div class="border-t border-white/10 px-6 py-4">
						<button
							class="group flex w-full items-center justify-center gap-2 rounded-lg bg-white/10 px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-white/15 active:scale-[0.97]"
							onclick={() =>
								(activeTab =
									activeTab === "academic-info"
										? "publications"
										: "academic-info")}
						>
							<BookOpen class="size-4" />
							{activeTab === "academic-info"
								? "Mostrar Publicaciones"
								: "Mostrar Información Académica"}
						</button>
					</div>
				</aside>

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
											{DateValue.formatDate(academic.joinedAt.iso)}
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
										<Loader2 class="size-5 animate-spin text-corp-gray" />
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
															: slot.kind === 'base'
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
															{CountryValue.format(
																degree.countryCode,
															)}
															<span class="mx-1.5 text-corp-gray/40"
																>·</span
															>
															{DateValue.formatDate(
																degree.obtainedAt,
															)}
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
