<script lang="ts">
	import {
		createQuery,
		createMutation as createTanMutation,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { positionsService } from "$lib/services/positions.service"
	import Dialog from "$lib/components/ui/dialog.svelte"
	import Button from "$lib/components/ui/button.svelte"
	import Input from "$lib/components/ui/input.svelte"
	import { Plus, Loader2, Trash2 } from "@lucide/svelte"
	import { toast } from "svelte-sonner"

	const queryClient = useQueryClient()

	const query = createQuery(() => ({
		queryKey: ["admin", "positions"],
		queryFn: () => positionsService.list(),
	}))

	let showCreate = $state(false)
	let name = $state("")

	const createPos = createTanMutation(() => ({
		mutationFn: () => positionsService.create({ name }),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["admin", "positions"] })
			toast.success("Cargo creado")
			showCreate = false
			name = ""
		},
		onError: () => toast.error("Error al crear el cargo"),
	}))
</script>

<div>
	<div class="mb-6 flex items-center justify-between">
		<div>
			<h1 class="text-lg font-semibold text-[#1A1A1A]">Cargos Laborales</h1>
			<p class="mt-1 text-sm text-corp-gray">Gestiona los cargos laborales de los académicos.</p>
		</div>
		<Button onclick={() => (showCreate = true)}>
			<Plus class="size-4" />
			Nuevo
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
						<th class="px-4 py-3 w-16"></th>
					</tr>
				</thead>
				<tbody>
					{#each query.data ?? [] as pos (pos.id)}
						<tr class="border-b border-corp-gray/10 last:border-0">
							<td class="px-4 py-3 text-[#1A1A1A]">{pos.name}</td>
							<td class="px-4 py-3">
								<button
									class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-red-50 hover:text-red-600"
								>
									<Trash2 class="size-4" />
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
			{#if (query.data ?? []).length === 0}
				<p class="px-4 py-8 text-center text-sm text-corp-gray">No hay cargos registrados.</p>
			{/if}
		</div>
	{/if}
</div>

<Dialog bind:open={showCreate} title="Nuevo cargo laboral">
	<form
		class="grid gap-4"
		onsubmit={(e) => {
			e.preventDefault()
			createPos.mutate()
		}}
	>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Nombre</span>
			<Input bind:value={name} placeholder="Ej: Docente" required />
		</label>
		<div class="mt-2 flex justify-end gap-2">
			<Button variant="secondary" type="button" onclick={() => (showCreate = false)}
				>Cancelar</Button
			>
			<Button type="submit" disabled={createPos.isPending || !name}>
				{createPos.isPending ? "Creando..." : "Crear"}
			</Button>
		</div>
	</form>
</Dialog>
