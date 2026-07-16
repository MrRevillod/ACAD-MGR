<script lang="ts">
	import type { CreateCategoryDTO } from "$categories/dtos"

	import { toast } from "svelte-sonner"
	import { useMutation, useQueryClient } from "$shared/http/tanstack"
	import { createForm, Field, Form, reset } from "@formisch/svelte"

	import { PlantaValue } from "$academics/value-objects/planta.value"
	import { categoryService } from "$categories/service"
	import { createCategoryDTOSchema } from "$categories/dtos"

	import Dialog from "$shared/components/ui/dialog.svelte"
	import Select from "$shared/components/ui/form/select.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import FormFooter from "$shared/components/ui/form/footer.svelte"

	interface Props {
		open: boolean
		onClose: () => void
	}

	let { open = $bindable(), onClose }: Props = $props()

	const form = createForm({ schema: createCategoryDTOSchema })

	$effect(() => {
		if (open) reset(form, { initialInput: { name: "", planta: "permanente" } })
	})

	const queryClient = useQueryClient()

	const createCat = useMutation(() => ({
		mutationFn: (output: CreateCategoryDTO) => categoryService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["admin", "categories"] })
			toast.success("Categoría creada")
			open = false
		},
		onError: () => toast.error("Error al crear la categoría"),
	}))

	const plantaOptions = $derived(
		PlantaValue.PLANTA.map((p) => ({ label: PlantaValue.LABELS[p], value: p })),
	)

	function handleClose() {
		open = false
		onClose()
	}
</script>

<Dialog bind:open title="Nueva categoría" class="max-w-xl">
	<Form of={form} onsubmit={(output) => createCat.mutate(output)}>
		<div class="grid gap-4">
			<Field of={form} path={["name"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type="text"
						label="Nombre"
						placeholder="Ej: Profesor Titular"
					/>
				{/snippet}
			</Field>

			<Field of={form} path={["planta"]}>
				{#snippet children(field)}
					<Select
						{...field.props}
						input={field.input}
						errors={field.errors}
						label="Planta"
						options={plantaOptions}
					/>
				{/snippet}
			</Field>
		</div>

		<FormFooter onCancel={handleClose} submitLabel="Crear" isPending={createCat.isPending} />
	</Form>
</Dialog>
