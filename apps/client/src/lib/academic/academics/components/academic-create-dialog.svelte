<script lang="ts">
	import type { CreateAcademicDTO } from "$academics/dtos"

	import { toast } from "svelte-sonner"
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { useMutation, useQuery, queryClient } from "$shared/http/tanstack"

	import { careerService } from "$careers/service"
	import { optionService } from "$options/service"
	import { academicService } from "$academics/service"
	import { positionService } from "$work-positions/service"
	import { categoryService } from "$categories/service"
	import { departmentService } from "$departments/service"
	import { createAcademicDTOInitialInput, createAcademicDTOSchema } from "$academics/dtos"

	import { SexValue } from "$shared/value-objects/sex.value"
	import { countryItems } from "$shared/countries"

	import Dialog from "$shared/components/ui/dialog.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import Select from "$shared/components/ui/form/select.svelte"
	import FormFooter from "$shared/components/ui/form/footer.svelte"

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
		if (!open) return
		reset(form, { initialInput: createAcademicDTOInitialInput })
		selectedCategoryId = ""
		selectedCategoryError = undefined
		selectedDeptId = ""
	})

	const createAcad = useMutation(() => ({
		mutationFn: (output: CreateAcademicDTO) => academicService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["academics"] })
			toast.success("Académico creado")
			open = false
		},
		onError: () => toast.error("Error al crear el académico"),
	}))

	const departmentsQuery = useQuery(() => ({
		queryKey: ["departments"],
		queryFn: () => departmentService.list(),
	}))

	const careersQuery = useQuery(() => ({
		queryKey: ["careers", selectedDeptId],
		queryFn: () =>
			careerService.list(selectedDeptId ? { department_id: selectedDeptId } : undefined),
		enabled: Boolean(selectedDeptId),
	}))

	const positionsQuery = useQuery(() => ({
		queryKey: ["positions"],
		queryFn: () => positionService.list(),
	}))

	const categoriesQuery = useQuery(() => ({
		queryKey: ["categories"],
		queryFn: () => categoryService.list(),
	}))

	const optionsQuery = useQuery(() => ({
		queryKey: ["category-options", selectedCategoryId],
		queryFn: () => optionService.list({ category_id: selectedCategoryId }),
		enabled: Boolean(selectedCategoryId),
	}))

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

	const sexOptions = $derived(
		Object.entries(SexValue.LABELS).map(([value, label]) => ({ label, value })),
	)

	const categoryData = $derived(categoriesQuery.data ?? [])

	const deptOptions = $derived(
		(departmentsQuery.data ?? []).map((d) => ({ label: d.name, value: d.id })),
	)

	const positionOptions = $derived(
		(positionsQuery.data ?? []).map((p) => ({ label: p.name, value: p.id })),
	)

	const careerOptions = $derived([
		{ label: "Sin carrera", value: "" },
		...(careersQuery.data ?? []).map((c) => ({ label: c.name, value: c.id })),
	])

	const countryOptions = $derived(countryItems.map((c) => ({ label: c.label, value: c.value })))

	const optionOptions = $derived(
		(optionsQuery.data ?? []).map((opt) => {
			const catLabel =
				categoryData.find((c) => c.id === opt.categoryId)?.name ?? opt.categoryId
			return {
				label: `${catLabel} · ${opt.option.toDisplay()}${opt.hours != null ? ` · ${opt.hours} hrs` : ""}`,
				value: opt.id,
			}
		}),
	)
</script>

<Dialog bind:open title="Nuevo académico" class="max-w-5xl">
	<Form of={form} onsubmit={(output) => handleSubmit(output)}>
		<div class="grid gap-4">
			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["rut"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="text"
							label="RUT"
							placeholder="XXXXXXXX-X"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["email"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="email"
							label="Email"
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-3 gap-4">
				<Field of={form} path={["names"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="text"
							label="Nombres"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["paternalSurname"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="text"
							label="Apellido paterno"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["maternalSurname"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="text"
							label="Apellido materno"
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["sex"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Sexo"
							options={sexOptions}
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["orcid"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input ?? ""}
							errors={field.errors}
							type="text"
							label="ORCID"
							placeholder="0000-0000-0000-0000"
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["birthDate"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="date"
							label="Fecha de nacimiento"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["joinedAt"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="date"
							label="Fecha de ingreso"
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["departmentId"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Departamento"
							placeholder="Seleccionar departamento..."
							options={deptOptions}
							onValueChange={(v) => (selectedDeptId = v)}
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["workPositionId"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Cargo"
							placeholder="Seleccionar cargo..."
							options={positionOptions}
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["careerId"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input ?? ""}
							errors={field.errors}
							label="Carrera"
							placeholder="Sin carrera"
							options={careerOptions}
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["nationalityCode"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Nacionalidad"
							placeholder="Seleccionar país..."
							options={countryOptions}
						/>
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
						onchange={(e) => {
							const el = e.currentTarget
							if (el instanceof HTMLSelectElement) {
								selectedCategoryId = el.value
								selectedCategoryError = undefined
							}
						}}
						class="h-10 w-full rounded-lg border bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors {selectedCategoryError
							? 'border-red-500'
							: 'border-corp-gray/20 focus:border-corp-blue/50'}"
					>
						<option value="" selected>Seleccionar categoría...</option>
						{#each categoryData as cat (cat.id)}
							<option value={cat.id}>{cat.name}</option>
						{/each}
					</select>
					{#if selectedCategoryError}
						<p class="text-xs text-red-600">{selectedCategoryError}</p>
					{/if}
				</div>
				<Field of={form} path={["acadCategoryOptionsId"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Opción"
							placeholder={selectedCategoryId
								? "Seleccionar opción..."
								: "Seleccione una categoría primero"}
							options={optionOptions}
							disabled={!selectedCategoryId}
						/>
					{/snippet}
				</Field>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["jce"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input ?? ""}
							errors={field.errors}
							type="number"
							label="JCE"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["annualDiscountHours"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input ?? ""}
							errors={field.errors}
							type="number"
							label="Horas descuento anual"
						/>
					{/snippet}
				</Field>
			</div>

			<Field of={form} path={["city"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type="text"
						label="Ciudad"
					/>
				{/snippet}
			</Field>

			<FormFooter
				onCancel={handleClose}
				submitLabel="Crear"
				isPending={createAcad.isPending}
			/>
		</div>
	</Form>
</Dialog>
