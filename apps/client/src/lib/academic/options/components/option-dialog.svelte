<script lang="ts">
	import type { CreateOptionDto } from "$options/dtos"

	import { toast } from "svelte-sonner"
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { queryClient, useMutation, useQuery } from "$shared/http/tanstack"

	import { optionService } from "$options/service"
	import { categoryService } from "$categories/service"
	import { AcademicOptionValue } from "$options/value-objects/option.value"
	import { createOptionSchema, createOptionDTOInitialInput } from "$options/dtos"

	import Dialog from "$shared/components/ui/dialog.svelte"
	import Select from "$shared/components/ui/form/select.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import FormFooter from "$shared/components/ui/form/footer.svelte"

	interface Props {
		open: boolean
		onClose: () => void
	}

	let { open = $bindable(), onClose }: Props = $props()

	const form = createForm({ schema: createOptionSchema })

	$effect(() => {
		if (open) reset(form, { initialInput: createOptionDTOInitialInput })
	})

	const createOpt = useMutation(() => ({
		mutationFn: (output: CreateOptionDto) => optionService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["admin", "options"] })
			toast.success("Opción creada")
			open = false
		},
		onError: () => toast.error("Error al crear la opción"),
	}))

	const categoriesQuery = useQuery(() => ({
		queryKey: ["admin", "categories"],
		queryFn: () => categoryService.list(),
	}))

	const categories = $derived(categoriesQuery.data ?? [])

	const optionValues = $derived(
		AcademicOptionValue.OPTIONS.map((v) => ({
			label: AcademicOptionValue.LABELS[v],
			value: v,
		})),
	)

	const categoryOptions = $derived(categories.map((c) => ({ label: c.name, value: c.id })))

	function handleClose() {
		open = false
		onClose()
	}
</script>

<Dialog bind:open title="Nueva opción" class="max-w-xl">
	<Form of={form} onsubmit={(output) => createOpt.mutate(output)}>
		<div class="grid gap-4">
			<Field of={form} path={["categoryId"]}>
				{#snippet children(field)}
					<Select
						{...field.props}
						input={field.input}
						errors={field.errors}
						label="Categoría"
						placeholder="Seleccionar categoría..."
						options={categoryOptions}
					/>
				{/snippet}
			</Field>

			<Field of={form} path={["option"]}>
				{#snippet children(field)}
					<Select
						{...field.props}
						input={field.input}
						errors={field.errors}
						label="Opción"
						options={optionValues}
					/>
				{/snippet}
			</Field>

			<Field of={form} path={["hours"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type="number"
						label="Horas"
						placeholder="Opcional"
					/>
				{/snippet}
			</Field>
		</div>

		<FormFooter onCancel={handleClose} submitLabel="Crear" isPending={createOpt.isPending} />
	</Form>
</Dialog>
