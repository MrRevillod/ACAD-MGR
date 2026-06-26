<script lang="ts">
	import {
		createQuery,
		createMutation as createTanMutation,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { optionsService } from "$lib/services/options.service"
	import { categoriesService } from "$lib/services/categories.service"
	import Dialog from "$lib/components/ui/dialog.svelte"
	import Button from "$lib/components/ui/button.svelte"
	import Select from "$lib/components/ui/select.svelte"
	import Input from "$lib/components/ui/input.svelte"
	import Badge from "$lib/components/ui/badge.svelte"
	import { Plus, Loader2, Trash2 } from "@lucide/svelte"
	import { toast } from "svelte-sonner"
	import type { AcademicOption } from "$lib/types"

	const queryClient = useQueryClient()

	const query = createQuery(() => ({
		queryKey: ["admin", "options"],
		queryFn: () => optionsService.list(),
	}))

	const categoriesQuery = createQuery(() => ({
		queryKey: ["admin", "categories"],
		queryFn: () => categoriesService.list(),
	}))

	let showCreate = $state(false)
	let categoryId = $state("")
	let optionValue = $state<AcademicOption>("teaching")
	let hours = $state<number | "">("")

	const createOpt = createTanMutation(() => ({
		mutationFn: () =>
			optionsService.create({
				categoryId,
				option: optionValue,
				hours: hours === "" ? null : hours,
			}),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["admin", "options"] })
			toast.success("Opción creada")
			showCreate = false
			categoryId = ""
			optionValue = "teaching"
			hours = ""
		},
		onError: () => toast.error("Error al crear la opción"),
	}))

	function optionLabel(o: string): string {
		switch (o) {
			case "teaching":
				return "Docencia"
			case "research":
				return "Investigación"
			default:
				return o
		}
	}

	const categories = $derived(categoriesQuery.data ?? [])
	const categoryMap = $derived(Object.fromEntries(categories.map((c) => [c.id, c.name])))
</script>

<div>
	<div class="mb-6 flex items-center justify-between">
		<div>
			<h1 class="text-lg font-semibold text-[#1A1A1A]">Opciones de Categoría</h1>
			<p class="mt-1 text-sm text-corp-gray">Gestiona las opciones válidas por categoría.</p>
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
							>Categoría</th
						>
						<th
							class="px-4 py-3 text-left text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Opción</th
						>
						<th
							class="px-4 py-3 text-left text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Horas</th
						>
						<th class="px-4 py-3 w-16"></th>
					</tr>
				</thead>
				<tbody>
					{#each query.data ?? [] as opt (opt.id)}
						<tr class="border-b border-corp-gray/10 last:border-0">
							<td class="px-4 py-3 text-[#1A1A1A]"
								>{categoryMap[opt.categoryId] ?? opt.categoryId}</td
							>
							<td class="px-4 py-3">
								<Badge variant={opt.option === "research" ? "advanced" : "base"}>
									{optionLabel(opt.option)}
								</Badge>
							</td>
							<td class="px-4 py-3 text-corp-gray">{opt.hours?.toLocaleString("es-CL") ?? "—"}</td>
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
				<p class="px-4 py-8 text-center text-sm text-corp-gray">No hay opciones registradas.</p>
			{/if}
		</div>
	{/if}
</div>

<Dialog bind:open={showCreate} title="Nueva opción">
	<form
		class="grid gap-4"
		onsubmit={(e) => {
			e.preventDefault()
			createOpt.mutate()
		}}
	>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Categoría</span>
			<Select
				bind:value={categoryId}
				items={categories.map((c) => ({ value: c.id, label: c.name }))}
				placeholder="Seleccionar categoría..."
			/>
		</label>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Opción</span>
			<Select
				bind:value={optionValue}
				items={[
					{ value: "teaching", label: "Docencia" },
					{ value: "research", label: "Investigación" },
				]}
			/>
		</label>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Horas</span>
			<Input type="number" bind:value={hours} placeholder="Opcional" />
		</label>
		<div class="mt-2 flex justify-end gap-2">
			<Button variant="secondary" type="button" onclick={() => (showCreate = false)}
				>Cancelar</Button
			>
			<Button type="submit" disabled={createOpt.isPending || !categoryId}>
				{createOpt.isPending ? "Creando..." : "Crear"}
			</Button>
		</div>
	</form>
</Dialog>
