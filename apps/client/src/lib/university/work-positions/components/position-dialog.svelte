<script lang="ts">
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { createMutation, useQueryClient } from "@tanstack/svelte-query"
	import Dialog from "$lib/shared/components/ui/dialog.svelte"
	import Button from "$lib/shared/components/ui/button.svelte"
	import { positionService } from "$lib/university/work-positions/service"
	import { toast } from "svelte-sonner"
	import { createPositionSchema } from "../dtos"

	interface Props {
		open: boolean
		onClose: () => void
	}

	let { open = $bindable(), onClose }: Props = $props()

	const form = createForm({ schema: createPositionSchema })

	$effect(() => {
		if (open) reset(form, { initialInput: { name: "" } })
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
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Nombre</span
						>
						<input
							{...field.props}
							value={field.input}
							placeholder="Ej: Docente"
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
				<Button type="submit" disabled={createPos.isPending}>
					{createPos.isPending ? "Creando..." : "Crear"}
				</Button>
			</div>
		</div>
	</Form>
</Dialog>
