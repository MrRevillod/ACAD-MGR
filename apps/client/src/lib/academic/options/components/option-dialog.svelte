<script lang="ts">
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query"
	import Dialog from "$lib/shared/components/ui/dialog.svelte"
	import Button from "$lib/shared/components/ui/button.svelte"
	import { optionService } from "$lib/academic/options/service"
	import { categoryService } from "$lib/academic/categories/service"
	import { toast } from "svelte-sonner"
	import { createOptionSchema, type CreateOptionDto } from "../dtos"

	interface Props {
		open: boolean
		onClose: () => void
	}

	let { open = $bindable(), onClose }: Props = $props()

	const form = createForm({ schema: createOptionSchema })

	$effect(() => {
		if (open) reset(form, { initialInput: { categoryId: "", option: "teaching", hours: null } })
	})

	const queryClient = useQueryClient()

	const createOpt = createMutation(() => ({
		mutationFn: (output: CreateOptionDto) => optionService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["admin", "options"] })
			toast.success("Opción creada")
			open = false
		},
		onError: () => toast.error("Error al crear la opción"),
	}))

	const categoriesQuery = createQuery(() => ({
		queryKey: ["admin", "categories"],
		queryFn: () => categoryService.list(),
	}))

	const categories = $derived(categoriesQuery.data ?? [])

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
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Categoría</span
						>
						<select
							{...field.props}
							value={field.input}
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						>
							<option value="">Seleccionar categoría...</option>
							{#each categories as c (c.id)}
								<option value={c.id}>{c.name}</option>
							{/each}
						</select>
						{#if field.errors}
							<p class="text-xs text-red-600">{field.errors[0]}</p>
						{/if}
					</label>
				{/snippet}
			</Field>
			<Field of={form} path={["option"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Opción</span>
						<select
							{...field.props}
							value={field.input}
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						>
							<option value="teaching">Docencia</option>
							<option value="research">Investigación</option>
						</select>
					</label>
				{/snippet}
			</Field>
			<Field of={form} path={["hours"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Horas</span>
						<input
							{...field.props}
							value={field.input ?? ""}
							type="number"
							step="any"
							min="0"
							placeholder="Opcional"
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						/>
					</label>
				{/snippet}
			</Field>
			<div class="mt-2 flex justify-end gap-2">
				<Button variant="secondary" type="button" onclick={handleClose}>Cancelar</Button>
				<Button type="submit" disabled={createOpt.isPending}>
					{createOpt.isPending ? "Creando..." : "Crear"}
				</Button>
			</div>
		</div>
	</Form>
</Dialog>
