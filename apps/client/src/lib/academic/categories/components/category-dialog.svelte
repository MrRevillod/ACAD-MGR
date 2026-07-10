<script lang="ts">
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { createMutation, useQueryClient } from "@tanstack/svelte-query"
	import Dialog from "$lib/shared/components/ui/dialog.svelte"
	import Button from "$lib/shared/components/ui/button.svelte"
	import { categoryService } from "$lib/academic/categories/service"
	import { toast } from "svelte-sonner"
	import { ACADEMIC_PLANTA, PLANTA_LABELS } from "$lib/academic/academics/enums"
	import { createCategorySchema, type CreateCategoryDto } from "../dtos"

	interface Props {
		open: boolean
		onClose: () => void
	}

	let { open = $bindable(), onClose }: Props = $props()

	const form = createForm({ schema: createCategorySchema })

	$effect(() => {
		if (open) reset(form, { initialInput: { name: "", planta: "permanente" } })
	})

	const queryClient = useQueryClient()

	const createCat = createMutation(() => ({
		mutationFn: (output: CreateCategoryDto) => categoryService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["admin", "categories"] })
			toast.success("Categoría creada")
			open = false
		},
		onError: () => toast.error("Error al crear la categoría"),
	}))

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
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Nombre</span>
						<input
							{...field.props}
							value={field.input}
							placeholder="Ej: Profesor Titular"
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						/>
						{#if field.errors}
							<p class="text-xs text-red-600">{field.errors[0]}</p>
						{/if}
					</label>
				{/snippet}
			</Field>
			<Field of={form} path={["planta"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Planta</span>
						<select
							{...field.props}
							value={field.input}
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						>
							{#each ACADEMIC_PLANTA as p (p)}
								<option value={p}>{PLANTA_LABELS[p]}</option>
							{/each}
						</select>
					</label>
				{/snippet}
			</Field>
			<div class="mt-2 flex justify-end gap-2">
				<Button variant="secondary" type="button" onclick={handleClose}>Cancelar</Button>
				<Button type="submit" disabled={createCat.isPending}>
					{createCat.isPending ? "Creando..." : "Crear"}
				</Button>
			</div>
		</div>
	</Form>
</Dialog>
