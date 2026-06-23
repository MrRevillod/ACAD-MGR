<script lang="ts">
	import { mockAcademic, mockDegrees, mockRut } from "$lib/mock-data"
	import type { Sex, AcademicPlanta, AcademicOption } from "$lib/types"
	import { ChevronLeft, MapPin, Calendar, GraduationCap, Briefcase, BookOpen } from "@lucide/svelte"

	const academic = mockAcademic
	const degrees = mockDegrees

	const fullName = $derived(
		`${academic.names} ${academic.paternalSurname} ${academic.maternalSurname}`,
	)

	const initials = $derived(
		(academic.names.charAt(0) + academic.paternalSurname.charAt(0)).toUpperCase(),
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
	<div class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		<!-- Back -->
		<a
			href="/"
			class="mb-6 inline-flex items-center gap-1 text-sm font-medium text-corp-gray transition-colors hover:text-corp-blue"
		>
			<ChevronLeft class="size-4" />
			Volver
		</a>

		<div class="grid grid-cols-1 gap-6 lg:grid-cols-[320px_1fr]">
			<!-- ==================== SIDEBAR ==================== -->
			<aside class="overflow-hidden rounded-xl bg-corp-blue text-white">
				<!-- Avatar + name -->
				<div class="p-6 pb-4 text-center">
					<div
						class="mx-auto mb-4 flex size-24 items-center justify-center rounded-full bg-white/10 text-2xl font-bold tracking-widest text-white ring-2 ring-white/15"
					>
						{initials}
					</div>
					<h1 class="text-lg font-semibold leading-snug">{fullName}</h1>
					<p class="mt-1.5 text-sm text-white/60">{academic.email}</p>
				</div>

				<!-- RUT + ORCID -->
				<div class="border-t border-white/10 px-6 py-3">
					<div class="flex items-center justify-between text-xs">
						<span class="text-white/50">RUT</span>
						<span class="font-medium text-white/90">{mockRut}</span>
					</div>
					{#if academic.orcid}
						<div class="mt-2 flex items-center justify-between text-xs">
							<span class="text-white/50">ORCID</span>
							<span class="font-medium text-white/90">{academic.orcid}</span>
						</div>
					{/if}
				</div>

				<!-- Personal data -->
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

				<!-- Joined date footer -->
				<div class="border-t border-white/10 px-6 py-3">
					<p class="text-xs text-white/50">
						Ingreso {fmt(academic.joinedAt)}
					</p>
				</div>
			</aside>

			<!-- ==================== MAIN CONTENT ==================== -->
			<main class="space-y-6">
				<!-- Información Laboral -->
				<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
					<div
						class="mb-5 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
					>
						<Briefcase class="size-4 text-corp-blue" />
						Información Laboral
					</div>
					<div class="grid grid-cols-1 gap-x-8 gap-y-4 sm:grid-cols-2">
						<div>
							<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Departamento</p>
							<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">{academic.department}</p>
						</div>
						<div>
							<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Carrera</p>
							<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">{academic.career ?? "—"}</p>
						</div>
						<div>
							<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Cargo</p>
							<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">{academic.workPosition}</p>
							{#if academic.workPositionDetails}
								<p class="text-xs text-corp-gray">{academic.workPositionDetails}</p>
							{/if}
						</div>
						<div>
							<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Jornada UCT</p>
							<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
								{academic.uctWorkingHours.toLocaleString("es-CL")} horas semanales
							</p>
						</div>
					</div>
				</section>

				<!-- Categorización Académica -->
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
								{academic.acadCategoryHours.toLocaleString("es-CL")} semanales
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

				<!-- Grados Académicos -->
				<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
					<div
						class="mb-6 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
					>
						<GraduationCap class="size-4 text-corp-blue" />
						Grados Académicos
					</div>

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
										<span
											class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold tracking-wide {deg.kind ===
											'base'
												? 'bg-corp-blue/10 text-corp-blue'
												: 'bg-corp-yellow/15 text-[#7a6400]'}"
										>
											{deg.kind === "base" ? "BASE" : "AVANZADO"}
										</span>
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

					{#if degrees.length === 0}
						<p class="text-sm text-corp-gray">No se registran grados académicos.</p>
					{/if}
				</section>
			</main>
		</div>
	</div>
</div>
