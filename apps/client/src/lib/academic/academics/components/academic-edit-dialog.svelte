<script lang="ts">
	import type { Academic } from "$academics/entity"
	import type { UpdateAcademicDTO } from "$academics/dtos"

	import { SexValue } from "$shared/value-objects/sex.value"
	import { countryItems } from "$shared/countries"
	import { academicService } from "$academics/service"
	import { updateAcademicDTOSchema } from "$academics/dtos"

	import { queryClient, useMutation } from "$shared/http/tanstack"
	import { createForm, Field, Form, reset } from "@formisch/svelte"

	import Dialog from "$shared/components/ui/dialog.svelte"
	import Select from "$shared/components/ui/form/select.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import FormFooter from "$shared/components/ui/form/footer.svelte"

	interface Props {
		academic: Academic
		open: boolean
		onClose: () => void
	}

	let { academic, open = $bindable(), onClose }: Props = $props()

	const form = createForm({ schema: updateAcademicDTOSchema })

	$effect(() => {
		if (!open) return
		const matched = countryItems.find((i) => i.label.includes(academic.nationality.toDisplay()))

		const initialInput = {
			names: academic.names,
			paternalSurname: academic.paternalSurname,
			maternalSurname: academic.maternalSurname,
			email: academic.email,
			orcid: academic.orcid ?? null,
			sex: academic.sex.code,
			birthDate: academic.birthDate.iso ?? "",
			city: academic.city,
			nationalityCode: matched?.value ?? "CL",
			jce: academic.jce.number,
			annualDiscountHours: academic.annualDiscountHours,
		}

		reset(form, { initialInput })
	})

	const updateAcademic = useMutation(() => ({
		mutationFn: (output: UpdateAcademicDTO) => academicService.update(academic.id, output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["academic", academic.id] })
			open = false
		},
	}))

	const sexOptions = $derived(
		Object.entries(SexValue.LABELS).map(([value, label]) => ({ label, value })),
	)

	const countryOptions = $derived(countryItems.map((c) => ({ label: c.label, value: c.value })))
</script>

<Dialog bind:open title="Editar académico" class="max-w-2xl">
	<Form of={form} onsubmit={(output) => updateAcademic.mutate(output)}>
		<div class="grid grid-cols-2 gap-4">
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
			<Field of={form} path={["jce"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
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
						input={field.input}
						errors={field.errors}
						type="number"
						label="Horas descuento anual"
					/>
				{/snippet}
			</Field>
		</div>

		<FormFooter onCancel={onClose} submitLabel="Guardar" isPending={updateAcademic.isPending} />
	</Form>
</Dialog>
