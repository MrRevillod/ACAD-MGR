<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { page } from "$app/stores"
	import {
		ChevronLeft,
		MapPin,
		Calendar,
		GraduationCap,
		Briefcase,
		BookOpen,
		Loader2,
		AlertCircle,
	} from "@lucide/svelte"

	import { academicsService } from "$lib/services/academics.service"
	import { degreesService } from "$lib/services/degrees.service"
	import Badge from "$lib/components/ui/badge.svelte"
	import type { Sex, AcademicPlanta, AcademicOption } from "$lib/types"

	const id = $derived($page.params.id ?? "")

	const academicQuery = createQuery(() => ({
		queryKey: ["academic", id],
		queryFn: () => academicsService.get(id),
		enabled: Boolean(id),
	}))

	const degreesQuery = createQuery(() => ({
		queryKey: ["degrees", id],
		queryFn: () => degreesService.listByAcademic(id),
		enabled: Boolean(id),
	}))

	const academic = $derived(academicQuery.data)
	const degrees = $derived(degreesQuery.data ?? [])

	const fullName = $derived(
		academic ? `${academic.names} ${academic.paternalSurname} ${academic.maternalSurname}` : "",
	)

	const initials = $derived(
		academic ? (academic.names.charAt(0) + academic.paternalSurname.charAt(0)).toUpperCase() : "",
	)

	function fmt(iso: string): string {
		const d = new Date(iso + "T00:00:00")
		return d.toLocaleDateString("es-CL", { day: "2-digit", month: "2-digit", year: "numeric" })
	}

	function sexLabel(s: Sex): string {
		switch (s) {
			case "H":
				return "Masculino"
			case "M":
				return "Femenino"
			case "O":
				return "Otro"
		}
	}

	function plantaLabel(p: AcademicPlanta): string {
		switch (p) {
			case "adjunta":
				return "Adjunta"
			case "permanente":
				return "Permanente"
		}
	}

	function optionLabel(o: AcademicOption): string {
		switch (o) {
			case "teaching":
				return "Docencia"
			case "research":
				return "Investigación"
		}
	}
</script>

<div class="min-h-screen bg-white">
	{#if academicQuery.isPending}
		<div class="flex items-center justify-center py-24">
			<Loader2 class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if academicQuery.isError || !academic}
		<div class="flex flex-col items-center justify-center py-24 text-center">
			<AlertCircle class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Académico no encontrado.</p>
			<a href="/academics" class="mt-4 text-sm font-medium text-corp-blue hover:underline"
				>Volver al listado</a
			>
		</div>
	{:else}
		<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
			<a
				href="/academics"
				class="mb-6 inline-flex items-center gap-1 text-sm font-medium text-corp-gray transition-colors hover:text-corp-blue"
			>
				<ChevronLeft class="size-4" />
				Volver
			</a>

			<div class="grid grid-cols-1 gap-6 lg:grid-cols-[320px_1fr]">
				<!-- Sidebar -->
				<aside class="overflow-hidden rounded-xl bg-corp-blue text-white">
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
						<div class="space-y-3">
							<div class="flex items-center gap-3 text-sm">
								<MapPin class="size-4 shrink-0 text-corp-yellow" />
								<div>
									<p class="text-white/90">{academic.nationality}</p>
									<p class="text-xs text-white/60">{academic.city}</p>
								</div>
							</div>
							<div class="flex items-center gap-3 text-sm">
								<Calendar class="size-4 shrink-0 text-corp-yellow" />
								<div>
									<p class="text-white/90">{fmt(academic.birthDate)}</p>
									<p class="text-xs text-white/60">{sexLabel(academic.sex)}</p>
								</div>
							</div>
						</div>
					</div>

					<div class="border-t border-white/10 px-6 py-3">
						<p class="text-xs text-white/50">
							Ingreso {fmt(academic.joinedAt)}
						</p>
					</div>
				</aside>

				<!-- Main -->
				<main class="space-y-6">
					<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
						<div
							class="mb-5 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
						>
							<Briefcase class="size-4 text-corp-blue" />
							Información Laboral
						</div>
						<div class="grid grid-cols-1 gap-x-8 gap-y-4 sm:grid-cols-2">
							<div>
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">
									Departamento
								</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">{academic.department}</p>
							</div>
							<div>
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Carrera</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">{academic.career ?? "—"}</p>
							</div>
							<div>
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Cargo</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
									{academic.workPosition ?? "—"}
								</p>
							</div>
							<div>
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">JCE</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
									{academic.jce.toLocaleString("es-CL")}
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
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Planta</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
									{plantaLabel(academic.planta)}
								</p>
							</div>
							<div>
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Categoría</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">{academic.category}</p>
							</div>
							<div>
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Opción</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
									{optionLabel(academic.option)}
								</p>
							</div>
							<div>
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">
									Horas categoría
								</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
									{academic.acadCategoryHours?.toLocaleString("es-CL") ?? "—"} semanales
								</p>
							</div>
							<div>
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">
									Descuento anual
								</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
									{academic.annualDiscountHours.toLocaleString("es-CL")} horas
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
						{:else if degrees.length > 0}
							<div class="relative">
								{#each degrees as deg, i (deg.id)}
									<div class="relative flex gap-5 {i < degrees.length - 1 ? 'pb-8' : ''}">
										<div class="flex flex-col items-center">
											<div
												class="z-10 size-3 shrink-0 rounded-full {deg.kind === 'base'
													? 'bg-corp-blue'
													: 'bg-corp-yellow'}"
											></div>
											{#if i < degrees.length - 1}
												<div class="mt-1 w-px grow bg-corp-gray/20"></div>
											{/if}
										</div>
										<div class="min-w-0 flex-1">
											<div class="mb-1 flex items-center gap-2">
												<Badge variant={deg.kind === "base" ? "base" : "advanced"}>
													{deg.kind === "base" ? "BASE" : "AVANZADO"}
												</Badge>
											</div>
											<p class="text-[15px] font-medium text-[#1a1a1a]">{deg.name}</p>
											<p class="mt-1 text-sm text-corp-gray">
												{deg.university}
												<span class="mx-1.5 text-corp-gray/40">·</span>
												{deg.countryCode}
												<span class="mx-1.5 text-corp-gray/40">·</span>
												{fmt(deg.obtainedAt)}
											</p>
										</div>
									</div>
								{/each}
							</div>
						{:else}
							<p class="text-sm text-corp-gray">No se registran grados académicos.</p>
						{/if}
					</section>
				</main>
			</div>
		</div>
	{/if}
</div>
