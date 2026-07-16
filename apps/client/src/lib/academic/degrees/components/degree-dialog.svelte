<script lang="ts">
	import type { Degree } from "$degrees/entity"
	import type { DegreeKind } from "$degrees/value-objects/kind.value"

	import { countryItems } from "$shared/countries"
	import { degreeService } from "$degrees/service"
	import { DegreeKindValue } from "$degrees/value-objects/kind.value"
	import { createDegreeSchema, createDegreeDTOInitialInput } from "$degrees/dtos"

	import { queryClient, useMutation } from "$shared/http/tanstack"
	import { createForm, Field, Form, reset } from "@formisch/svelte"

	import Dialog from "$lib/shared/components/ui/dialog.svelte"
	import Select from "$lib/shared/components/ui/form/select.svelte"
	import TextInput from "$lib/shared/components/ui/form/text-input.svelte"
	import FormFooter from "$lib/shared/components/ui/form/footer.svelte"

	interface Props {
		academicId: string
		degree?: Degree | null
		createKind?: DegreeKind
		open: boolean
		onClose: () => void
	}

	let {
		academicId,
		degree = null,
		createKind = "base",
		open = $bindable(),
		onClose,
	}: Props = $props()

	const form = createForm({ schema: createDegreeSchema })

	$effect(() => {
		if (!open) return
		if (degree) {
			reset(form, {
				initialInput: {
					academicId,
					name: degree.name,
					university: degree.university,
					obtainedAt: degree.obtainedAt.iso ?? "",
					kind: degree.kind.code as DegreeKind,
					countryCode: degree.country.code ?? "",
				},
			})
		} else {
			reset(form, {
				initialInput: { ...createDegreeDTOInitialInput, academicId, kind: createKind },
			})
		}
	})

	const createDeg = useMutation(() => ({
		mutationFn: (output: {
			academicId: string
			name: string
			university: string
			obtainedAt: string
			kind: DegreeKind
			countryCode: string
		}) => degreeService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["degrees", academicId] })
			open = false
		},
	}))

	const updateDeg = useMutation(() => ({
		mutationFn: ({
			degId,
			data,
		}: {
			degId: string
			data: { name?: string; university?: string; obtainedAt?: string; countryCode?: string }
		}) => degreeService.update(degId, data),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["degrees", academicId] })
			open = false
		},
	}))

	function handleSubmit(output: {
		academicId: string
		name: string
		university: string
		obtainedAt: string
		kind: DegreeKind
		countryCode: string
	}) {
		if (degree) {
			updateDeg.mutate({
				degId: degree.id,
				data: {
					name: output.name,
					university: output.university,
					obtainedAt: output.obtainedAt,
					countryCode: output.countryCode,
				},
			})
		} else {
			createDeg.mutate(output)
		}
	}

	const pending = $derived(createDeg.isPending || updateDeg.isPending)

	const kindOptions = $derived([
		{ label: DegreeKindValue.LABELS.base, value: "base" },
		{ label: DegreeKindValue.LABELS.advanced, value: "advanced" },
	])

	const countryOptions = $derived(countryItems.map((c) => ({ label: c.label, value: c.value })))
</script>

<Dialog bind:open title={degree ? "Editar grado" : "Nuevo grado"} class="max-w-xl">
	<Form of={form} onsubmit={(output) => handleSubmit(output)}>
		<div class="grid gap-4">
			<Field of={form} path={["name"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type="text"
						label="Nombre"
						placeholder="Ej: Magíster en Ciencias"
					/>
				{/snippet}
			</Field>

			<Field of={form} path={["university"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type="text"
						label="Universidad"
						placeholder="Ej: Universidad Católica de Temuco"
					/>
				{/snippet}
			</Field>

			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["obtainedAt"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input}
							errors={field.errors}
							type="date"
							label="Fecha"
						/>
					{/snippet}
				</Field>
				<Field of={form} path={["kind"]}>
					{#snippet children(field)}
						<Select
							{...field.props}
							input={field.input}
							errors={field.errors}
							label="Tipo"
							options={kindOptions}
							disabled
						/>
					{/snippet}
				</Field>
			</div>

			<Field of={form} path={["countryCode"]}>
				{#snippet children(field)}
					<Select
						{...field.props}
						input={field.input}
						errors={field.errors}
						label="País"
						placeholder="Seleccionar país..."
						options={countryOptions}
					/>
				{/snippet}
			</Field>
		</div>

		<FormFooter onCancel={onClose} submitLabel="Guardar" isPending={pending} />
	</Form>
</Dialog>
