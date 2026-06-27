<script lang="ts">
	import { createQuery, createMutation, useQueryClient } from "@tanstack/svelte-query"
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
		Pencil,
		Plus,
	} from "@lucide/svelte"

	import { academicsService } from "$lib/services/academics.service"
	import { degreesService } from "$lib/services/degrees.service"
	import Dialog from "$lib/components/ui/dialog.svelte"
	import Input from "$lib/components/ui/input.svelte"
	import Select from "$lib/components/ui/select.svelte"
	import Button from "$lib/components/ui/button.svelte"
	import Badge from "$lib/components/ui/badge.svelte"
	import { CLf64Value } from "$lib/shared/value-objects/cl-f64.value"
	import { DateValue } from "$lib/shared/value-objects/date.value"
	import { CountryValue } from "$lib/shared/value-objects/country.value"
	import { countryItems } from "$lib/shared/countries"
	import {
		DEGREE_KIND,
		type Degree,
		type DegreeKind,
		type Sex,
		type AcademicPlanta,
		type AcademicOption,
		type UpdateAcademicDto,
	} from "$lib/types"

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
	const degreeSlots = $derived.by<
		Array<Degree | { kind: (typeof DEGREE_KIND)[number]; isPlaceholder: true }>
	>(() =>
		DEGREE_KIND.map((kind) => {
			const found = (degreesQuery.data ?? []).find((d) => d.kind === kind)
			return found ?? { kind, isPlaceholder: true as const }
		}),
	)

	const fullName = $derived(
		academic ? `${academic.names} ${academic.paternalSurname} ${academic.maternalSurname}` : "",
	)

	const initials = $derived(
		academic ? (academic.names.charAt(0) + academic.paternalSurname.charAt(0)).toUpperCase() : "",
	)

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

	const queryClient = useQueryClient()

	let showDialog = $state(false)
	let editingDegree = $state<Degree | null>(null)
	let name = $state("")
	let university = $state("")
	let obtainedAt = $state("")
	let kind = $state<DegreeKind>("base")
	let countryCode = $state("CL")

	function openCreate(k: DegreeKind) {
		editingDegree = null
		kind = k
		name = ""
		university = ""
		obtainedAt = ""
		countryCode = "CL"
		showDialog = true
	}

	function openEdit(deg: Degree) {
		editingDegree = deg
		kind = deg.kind
		name = deg.name
		university = deg.university
		obtainedAt = deg.obtainedAt
		countryCode = deg.countryCode
		showDialog = true
	}

	const createDeg = createMutation(() => ({
		mutationFn: () =>
			degreesService.create({
				academicId: id,
				name,
				university,
				obtainedAt,
				kind,
				countryCode,
			}),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["degrees", id] })
			showDialog = false
		},
	}))

	const updateDeg = createMutation(() => ({
		mutationFn: (degId: string) =>
			degreesService.update(degId, {
				name: name || undefined,
				university: university || undefined,
				obtainedAt: obtainedAt || undefined,
				countryCode: countryCode || undefined,
			}),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["degrees", id] })
			showDialog = false
		},
	}))

	function handleSave() {
		if (editingDegree) {
			updateDeg.mutate(editingDegree.id)
		} else {
			createDeg.mutate()
		}
	}

	let showEditAcademicDialog = $state(false)
	let editNames = $state("")
	let editPaternalSurname = $state("")
	let editMaternalSurname = $state("")
	let editEmail = $state("")
	let editOrcid = $state("")
	let editSex = $state<Sex>("H")
	let editBirthDate = $state("")
	let editCity = $state("")
	let editNationalityCode = $state("CL")
	let editJce = $state("")
	let editAnnualDiscountHours = $state("")

	function openEditAcademic() {
		if (!academic) return
		editNames = academic.names
		editPaternalSurname = academic.paternalSurname
		editMaternalSurname = academic.maternalSurname
		editEmail = academic.email
		editOrcid = academic.orcid ?? ""
		editSex = academic.sex
		editBirthDate = academic.birthDate
		editCity = academic.city
		const matched = countryItems.find((i) => i.label.includes(academic.nationality))
		editNationalityCode = matched?.value ?? "CL"
		editJce = String(academic.jce)
		editAnnualDiscountHours = String(academic.annualDiscountHours)
		showEditAcademicDialog = true
	}

	const updateAcademic = createMutation(() => ({
		mutationFn: () => {
			const payload: UpdateAcademicDto = {}
			if (editNames !== academic?.names) payload.names = editNames
			if (editPaternalSurname !== academic?.paternalSurname)
				payload.paternalSurname = editPaternalSurname
			if (editMaternalSurname !== academic?.maternalSurname)
				payload.maternalSurname = editMaternalSurname
			if (editEmail !== academic?.email) payload.email = editEmail
			if (editOrcid !== (academic?.orcid ?? "")) payload.orcid = editOrcid || null
			if (editSex !== academic?.sex) payload.sex = editSex
			if (editBirthDate !== academic?.birthDate) payload.birthDate = editBirthDate
			if (editCity !== academic?.city) payload.city = editCity
			const currentNatCode =
				countryItems.find((i) => i.label.includes(academic?.nationality ?? ""))?.value ?? "CL"
			if (editNationalityCode !== currentNatCode) payload.nationalityCode = editNationalityCode
			if (+editJce !== academic?.jce && editJce !== "") payload.jce = +editJce
			if (
				+editAnnualDiscountHours !== academic?.annualDiscountHours &&
				editAnnualDiscountHours !== ""
			)
				payload.annualDiscountHours = +editAnnualDiscountHours
			return academicsService.update(id, payload)
		},
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["academic", id] })
			showEditAcademicDialog = false
		},
	}))
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
					<a
						href="/academics"
						class="absolute left-3 top-3 z-10 flex size-8 items-center justify-center rounded-full bg-white text-corp-blue shadow-sm active:scale-95"
					>
						<ChevronLeft class="size-4" />
					</a>
					<button
						class="absolute right-3 top-3 z-10 flex size-8 items-center justify-center rounded-full bg-white text-corp-blue shadow-sm active:scale-95"
						onclick={openEditAcademic}
					>
						<Pencil class="size-4" />
					</button>
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
									<p class="text-white/90">{DateValue.formatDate(academic.birthDate)}</p>
									<p class="text-xs text-white/60">{sexLabel(academic.sex)}</p>
								</div>
							</div>
						</div>
					</div>
				</aside>

				<div class="space-y-6">
					<section class="rounded-xl border border-corp-gray/20 bg-white p-6">
						<div
							class="mb-5 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
						>
							<Briefcase class="size-4 text-corp-blue" />
							Información Laboral
						</div>
						<div class="grid grid-cols-1 gap-x-8 gap-y-4 sm:grid-cols-3">
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
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Ingreso</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
									{DateValue.formatDate(academic.joinedAt)}
								</p>
							</div>
							<div>
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Cargo</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
									{academic.workPosition ?? "—"}
								</p>
							</div>
							<div>
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">
									Jornada Completa Equivalente
								</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
									{CLf64Value.format(academic.jce)}
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
									Horas de categoría y opción
								</p>
								<p class="mt-1 text-[15px] font-medium text-[#1a1a1a]">
									{academic.acadCategoryHours?.toLocaleString("es-CL") ?? "—"} horas
								</p>
							</div>
							<div>
								<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">
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
									<div class="relative flex gap-5 {i < degreeSlots.length - 1 ? 'pb-8' : ''}">
										<div class="flex flex-col items-center">
											<div
												class="z-10 size-3 shrink-0 rounded-full {slot.isPlaceholder
													? 'bg-corp-gray/30'
													: slot.kind === 'base'
														? 'bg-corp-blue'
														: 'bg-corp-yellow'}"
											></div>
											{#if i < degreeSlots.length - 1}
												<div class="mt-1 w-px grow bg-corp-gray/20"></div>
											{/if}
										</div>
										<div class="min-w-0 flex-1">
											<div class="mb-1 flex items-center gap-2">
												<Badge variant={slot.kind === "base" ? "base" : "advanced"}>
													{slot.kind === "base" ? "Título Profesional" : "Grado Académico"}
												</Badge>
												{#if !slot.isPlaceholder}
													<button
														class="flex size-6 items-center justify-center rounded-md text-corp-gray/40 transition-colors hover:text-corp-blue"
														onclick={() => openEdit(slot)}
													>
														<Pencil class="size-3.5" />
													</button>
												{/if}
											</div>
											{#if slot.isPlaceholder}
												<button
													class="mt-1 inline-flex items-center gap-1.5 text-sm text-corp-gray/50 transition-colors hover:text-corp-blue"
													onclick={() => openCreate(slot.kind)}
												>
													<Plus class="size-3.5" />
													Agregar
												</button>
											{:else}
												<p class="text-[15px] font-medium text-[#1a1a1a]">{slot.name}</p>
												<p class="mt-1 text-sm text-corp-gray">
													{slot.university}
													<span class="mx-1.5 text-corp-gray/40">·</span>
													{CountryValue.format(slot.countryCode)}
													<span class="mx-1.5 text-corp-gray/40">·</span>
													{DateValue.formatDate(slot.obtainedAt)}
												</p>
											{/if}
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</section>
				</div>
			</div>
		</div>
	{/if}
</div>

<Dialog bind:open={showDialog} title={editingDegree ? "Editar grado" : "Nuevo grado"}>
	<form
		class="grid gap-4"
		onsubmit={(e) => {
			e.preventDefault()
			handleSave()
		}}
	>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Nombre</span>
			<Input bind:value={name} placeholder="Ej: Magíster en Ciencias" required />
		</label>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Universidad</span>
			<Input bind:value={university} placeholder="Ej: Universidad Católica de Temuco" required />
		</label>
		<div class="grid grid-cols-2 gap-4">
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Fecha</span>
				<Input type="date" bind:value={obtainedAt} required />
			</label>
			{#if !editingDegree}
				<label class="grid gap-1.5">
					<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Tipo</span>
					<Select
						bind:value={kind}
						items={[
							{ value: "base", label: "Título Profesional" },
							{ value: "advanced", label: "Grado Académico" },
						]}
					/>
				</label>
			{/if}
		</div>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">País</span>
			<Select items={countryItems} bind:value={countryCode} placeholder="Seleccionar país..." />
		</label>
		<div class="mt-2 flex justify-end gap-2">
			<Button variant="secondary" type="button" onclick={() => (showDialog = false)}
				>Cancelar</Button
			>
			<Button
				type="submit"
				disabled={createDeg.isPending ||
					updateDeg.isPending ||
					!name ||
					!university ||
					!obtainedAt ||
					!countryCode}
			>
				{createDeg.isPending || updateDeg.isPending ? "Guardando..." : "Guardar"}
			</Button>
		</div>
	</form>
</Dialog>

<Dialog bind:open={showEditAcademicDialog} title="Editar académico">
	<form
		class="grid gap-4"
		onsubmit={(e) => {
			e.preventDefault()
			updateAcademic.mutate()
		}}
	>
		<div class="grid grid-cols-2 gap-4">
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Nombres</span>
				<Input bind:value={editNames} />
			</label>
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
					>Apellido paterno</span
				>
				<Input bind:value={editPaternalSurname} />
			</label>
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
					>Apellido materno</span
				>
				<Input bind:value={editMaternalSurname} />
			</label>
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Email</span>
				<Input type="email" bind:value={editEmail} />
			</label>
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">ORCID</span>
				<Input bind:value={editOrcid} placeholder="0000-0000-0000-0000" />
			</label>
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Sexo</span>
				<Select
					bind:value={editSex}
					items={[
						{ value: "H", label: "Masculino" },
						{ value: "M", label: "Femenino" },
						{ value: "O", label: "Otro" },
					]}
				/>
			</label>
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
					>Fecha de nacimiento</span
				>
				<Input type="date" bind:value={editBirthDate} />
			</label>
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Ciudad</span>
				<Input bind:value={editCity} />
			</label>
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Nacionalidad</span>
				<Select
					items={countryItems}
					bind:value={editNationalityCode}
					placeholder="Seleccionar país..."
				/>
			</label>
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">JCE</span>
				<Input type="number" step="0.01" min="0" max="1" bind:value={editJce} />
			</label>
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
					>Horas descuento anual</span
				>
				<Input type="number" step="0.5" min="0" bind:value={editAnnualDiscountHours} />
			</label>
		</div>
		<div class="mt-2 flex justify-end gap-2">
			<Button variant="secondary" type="button" onclick={() => (showEditAcademicDialog = false)}
				>Cancelar</Button
			>
			<Button type="submit" disabled={updateAcademic.isPending}>
				{updateAcademic.isPending ? "Guardando..." : "Guardar"}
			</Button>
		</div>
	</form>
</Dialog>
