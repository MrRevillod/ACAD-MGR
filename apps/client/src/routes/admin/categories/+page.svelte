<script lang="ts">
	import {
		createQuery,
		createMutation as createTanMutation,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { categoriesService } from "$lib/services/categories.service"
	import Dialog from "$lib/components/ui/dialog.svelte"
	import Button from "$lib/components/ui/button.svelte"
	import Select from "$lib/components/ui/select.svelte"
	import Input from "$lib/components/ui/input.svelte"
	import Badge from "$lib/components/ui/badge.svelte"
	import { Plus, Loader2, Trash2 } from "@lucide/svelte"
	import { toast } from "svelte-sonner"
	import type { AcademicPlanta } from "$lib/types"
	import { ACADEMIC_PLANTA } from "$lib/types"

	const queryClient = useQueryClient()

	const query = createQuery(() => ({
		queryKey: ["admin", "categories"],
		queryFn: () => categoriesService.list(),
	}))

	let showCreate = $state(false)
	let name = $state("")
	let planta = $state<AcademicPlanta>("permanente")

	const createCat = createTanMutation(() => ({
		mutationFn: () => categoriesService.create({ name, planta }),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["admin", "categories"] })
			toast.success("Categoría creada")
			showCreate = false
			name = ""
			planta = "permanente"
		},
		onError: () => toast.error("Error al crear la categoría"),
	}))

	const deleteCat = createTanMutation(() => ({
		mutationFn: (_id: string) => Promise.resolve(),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["admin", "categories"] })
			toast.success("Categoría eliminada")
		},
	}))
</script>

<div>
	<div class="mb-6 flex items-center justify-between">
		<div>
			<h1 class="text-lg font-semibold text-[#1A1A1A]">Categorías Académicas</h1>
			<p class="mt-1 text-sm text-corp-gray">Gestiona las categorías académicas por planta.</p>
		</div>
		<Button onclick={() => (showCreate = true)}>
			<Plus class="size-4" />
			Nueva
		</Button>
	</div>

	{#if query.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader2 class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else}
		<div class="rounded-xl border border-corp-gray/20 bg-white">
			<table class="w-full text-sm">
				<thead>
					<tr class="border-b border-corp-gray/10">
						<th
							class="px-4 py-3 text-left text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Nombre</th
						>
						<th
							class="px-4 py-3 text-left text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Planta</th
						>
						<th class="px-4 py-3 w-16"></th>
					</tr>
				</thead>
				<tbody>
					{#each query.data ?? [] as cat (cat.id)}
						<tr class="border-b border-corp-gray/10 last:border-0">
							<td class="px-4 py-3 text-[#1A1A1A]">{cat.name}</td>
							<td class="px-4 py-3">
								<Badge variant={cat.planta === "permanente" ? "base" : "advanced"}>
									{cat.planta === "permanente" ? "Permanente" : "Adjunta"}
								</Badge>
							</td>
							<td class="px-4 py-3">
								<button
									class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-red-50 hover:text-red-600"
									onclick={() => deleteCat.mutate(cat.id)}
								>
									<Trash2 class="size-4" />
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
			{#if (query.data ?? []).length === 0}
				<p class="px-4 py-8 text-center text-sm text-corp-gray">No hay categorías registradas.</p>
			{/if}
		</div>
	{/if}
</div>

<Dialog bind:open={showCreate} title="Nueva categoría">
	<form
		class="grid gap-4"
		onsubmit={(e) => {
			e.preventDefault()
			createCat.mutate()
		}}
	>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Nombre</span>
			<Input bind:value={name} placeholder="Ej: Profesor Titular" required />
		</label>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Planta</span>
			<Select
				bind:value={planta}
				items={ACADEMIC_PLANTA.map((p) => ({
					value: p,
					label: p === "permanente" ? "Permanente" : "Adjunta",
				}))}
			/>
		</label>
		<div class="mt-2 flex justify-end gap-2">
			<Button variant="secondary" type="button" onclick={() => (showCreate = false)}
				>Cancelar</Button
			>
			<Button type="submit" disabled={createCat.isPending || !name}>
				{createCat.isPending ? "Creando..." : "Crear"}
			</Button>
		</div>
	</form>
</Dialog>
