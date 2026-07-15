<script lang="ts">
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query"
	import Dialog from "$lib/shared/components/ui/dialog.svelte"
	import Button from "$lib/shared/components/ui/button.svelte"
	import { academicService } from "$lib/academic/academics/service"
	import { departmentService } from "$lib/university/departments/service"
	import { careerService } from "$lib/university/careers/service"
	import { positionService } from "$lib/university/work-positions/service"
	import { categoryService } from "$lib/academic/categories/service"
	import { optionService } from "$lib/academic/options/service"
	import { countryItems } from "$lib/shared/countries"
	import { toast } from "svelte-sonner"
	import { createAcademicDTOSchema } from "../dtos"
	import type { CreateAcademicDTO } from "$lib/academic/academics/dtos"
	import { SexValue } from "$shared/value-objects/sex.value"
	import { AcademicOptionValue } from "$academics/value-objects/option.value"

	interface Props {
		open: boolean
		onClose: () => void
	}

	let { open = $bindable(), onClose }: Props = $props()

	const form = createForm({ schema: createAcademicDTOSchema })

	let selectedCategoryId = $state("")
	let selectedCategoryError: string | undefined = $state(undefined)
	let selectedDeptId = $state("")

	$effect(() => {
		if (open) {
			reset(form, {
				initialInput: {
					rut: "",
					names: "",
					paternalSurname: "",
					maternalSurname: "",
					email: "",
					orcid: null,
					sex: "H",
					birthDate: "",
					joinedAt: "",
					workPositionId: "",
					departmentId: "",
					careerId: null,
					acadCategoryOptionsId: "",
					jce: 0,
					annualDiscountHours: 0,
					nationalityCode: "CL",
					city: "",
				},
			})
			selectedCategoryId = ""
			selectedCategoryError = undefined
			selectedDeptId = ""
		}
	})

	const queryClient = useQueryClient()

	const createAcad = createMutation(() => ({
		mutationFn: (output: CreateAcademicDTO) => academicService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["academics"] })
			toast.success("Académico creado")
			open = false
		},
		onError: () => toast.error("Error al crear el académico"),
	}))

	const departmentsQuery = createQuery(() => ({
		queryKey: ["departments"],
		queryFn: () => departmentService.list(),
	}))

	const careersQuery = createQuery(() => ({
		queryKey: ["careers", selectedDeptId],
		queryFn: () =>
			careerService.list(selectedDeptId ? { department_id: selectedDeptId } : undefined),
		enabled: Boolean(selectedDeptId),
	}))

	const positionsQuery = createQuery(() => ({
		queryKey: ["positions"],
		queryFn: () => positionService.list(),
	}))

	const categoriesQuery = createQuery(() => ({
		queryKey: ["categories"],
		queryFn: () => categoryService.list(),
	}))

	const optionsQuery = createQuery(() => ({
		queryKey: ["category-options", selectedCategoryId],
		queryFn: () => optionService.list({ category_id: selectedCategoryId }),
		enabled: Boolean(selectedCategoryId),
	}))

	const departments = $derived(departmentsQuery.data ?? [])
	const careers = $derived(careersQuery.data ?? [])
	const positions = $derived(positionsQuery.data ?? [])
	const categories = $derived(categoriesQuery.data ?? [])
	const options = $derived(optionsQuery.data ?? [])

	function handleSubmit(output: CreateAcademicDTO) {
		if (!selectedCategoryId) {
			selectedCategoryError = "Seleccione primero una categoría"
			return
		}
		selectedCategoryError = undefined
		createAcad.mutate(output)
	}

	function handleClose() {
		open = false
		onClose()
	}
</script>

<Dialog bind:open title="Nuevo académico" class="max-w-5xl">
	<Form of={form} onsubmit={(output) => handleSubmit(output)}>
		<div class="grid gap-4">
			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["rut"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>RUT</span
							>
							<input
								{...field.props}
								value={field.input}
								placeholder="XXXXXXXX-X"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["email"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Email</span
							>
							<input
								{...field.props}
								value={field.input}
								type="email"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-3 gap-4">
				<Field of={form} path={["names"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Nombres</span
							>
							<input
								{...field.props}
								value={field.input}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["paternalSurname"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Apellido paterno</span
							>
							<input
								{...field.props}
								value={field.input}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["maternalSurname"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Apellido materno</span
							>
							<input
								{...field.props}
								value={field.input}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["sex"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Sexo</span
							>
							<select
								{...field.props}
								value={field.input}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							>
								{#each Object.entries(SexValue.LABELS) as [value, label] (value)}
									<option {value}>{label}</option>
								{/each}
							</select>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["orcid"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>ORCID</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								placeholder="0000-0000-0000-0000"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["birthDate"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Fecha de nacimiento</span
							>
							<input
								{...field.props}
								value={field.input}
								type="date"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["joinedAt"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Fecha de ingreso</span
							>
							<input
								{...field.props}
								value={field.input}
								type="date"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["departmentId"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Departamento</span
							>
							<select
								{...field.props}
								value={field.input}
								onchange={(e) => {
									const el = e.currentTarget
									if (el instanceof HTMLSelectElement) selectedDeptId = el.value
								}}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							>
								<option value="">Seleccionar departamento...</option>
								{#each departments as d (d.id)}
									<option value={d.id}>{d.name}</option>
								{/each}
							</select>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["workPositionId"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Cargo</span
							>
							<select
								{...field.props}
								value={field.input}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							>
								<option value="">Seleccionar cargo...</option>
								{#each positions as p (p.id)}
									<option value={p.id}>{p.name}</option>
								{/each}
							</select>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["careerId"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Carrera</span
							>
							<select
								{...field.props}
								value={field.input ?? ""}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							>
								<option value="">Sin carrera</option>
								{#each careers as c (c.id)}
									<option value={c.id}>{c.name}</option>
								{/each}
							</select>
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["nationalityCode"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Nacionalidad</span
							>
							<select
								{...field.props}
								value={field.input}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							>
								<option value="">Seleccionar país...</option>
								{#each countryItems as c (c.value)}
									<option value={c.value}>{c.label}</option>
								{/each}
							</select>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<div class="grid gap-1.5">
					<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
						>Categoría</span
					>
					<select
						bind:value={selectedCategoryId}
						class="h-10 w-full rounded-lg border {selectedCategoryError
							? 'border-red-500'
							: 'border-corp-gray/20'} bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
					>
						<option value="">Seleccionar categoría...</option>
						{#each categories as c (c.id)}
							<option value={c.id}>{c.name}</option>
						{/each}
					</select>
					{#if selectedCategoryError}
						<p class="text-xs text-red-600">{selectedCategoryError}</p>
					{/if}
				</div>
				<Field of={form} path={["acadCategoryOptionsId"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Opción</span
							>
							<select
								{...field.props}
								value={field.input}
								disabled={!selectedCategoryId}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10 disabled:bg-gray-50"
							>
								<option value="">
									{selectedCategoryId
										? "Seleccionar opción..."
										: "Seleccione una categoría primero"}
								</option>
								{#each options as o (o.id)}
									{@const catLabel =
										categories.find((c) => c.id === o.categoryId)?.name ??
										o.categoryId}
									<option value={o.id}>
										{catLabel} · {AcademicOptionValue.LABELS[o.option]}{o.hours !=
										null
											? ` · ${o.hours} hrs`
											: ""}
									</option>
								{/each}
							</select>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["jce"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>JCE</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								type="number"
								step="any"
								min="0"
								max="1"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["annualDiscountHours"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Horas descuento anual</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								type="number"
								step="any"
								min="0"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
			</div>

			<Field of={form} path={["city"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Ciudad</span
						>
						<input
							{...field.props}
							value={field.input}
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						/>
						{#if field.errors}
							<p class="text-xs text-red-600">{field.errors[0]}</p>
						{/if}
					</label>
				{/snippet}
			</Field>

			<div class="mt-2 flex justify-end gap-2">
				<Button variant="secondary" type="button" onclick={handleClose}>Cancelar</Button>
				<Button type="submit" disabled={createAcad.isPending}>
					{createAcad.isPending ? "Creando..." : "Crear académico"}
				</Button>
			</div>
		</div>
	</Form>
</Dialog>
