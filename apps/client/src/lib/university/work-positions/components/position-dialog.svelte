<script lang="ts">
	import { toast } from "svelte-sonner"
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { createMutation, useQueryClient } from "@tanstack/svelte-query"

	import { positionService } from "$work-positions/service"
	import { createPositionSchema, createPositionDTOInitialInput } from "$work-positions/dtos"

	import Dialog from "$shared/components/ui/dialog.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import FormFooter from "$shared/components/ui/form/footer.svelte"

	interface Props {
		open: boolean
		onClose: () => void
	}

	let { open = $bindable(), onClose }: Props = $props()

	const form = createForm({ schema: createPositionSchema })

	$effect(() => {
		if (open) reset(form, { initialInput: createPositionDTOInitialInput })
	})

	const queryClient = useQueryClient()

	const createPos = createMutation(() => ({
		mutationFn: (output: { name: string }) => positionService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["admin", "positions"] })
			toast.success("Cargo creado")
			open = false
		},
		onError: () => toast.error("Error al crear el cargo"),
	}))

	function handleClose() {
		open = false
		onClose()
	}
</script>

<Dialog bind:open title="Nuevo cargo laboral">
	<Form of={form} onsubmit={(output) => createPos.mutate(output)}>
		<div class="grid gap-4">
			<Field of={form} path={["name"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type="text"
						label="Nombre"
						placeholder="Ej: Docente"
					/>
				{/snippet}
			</Field>
		</div>

		<FormFooter onCancel={handleClose} submitLabel="Crear" isPending={createPos.isPending} />
	</Form>
</Dialog>
