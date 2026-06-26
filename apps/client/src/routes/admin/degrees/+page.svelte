<script lang="ts">
	import {
		createQuery,
		createMutation as createTanMutation,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { degreesService } from "$lib/services/degrees.service"
	import { academicsService } from "$lib/services/academics.service"
	import Dialog from "$lib/components/ui/dialog.svelte"
	import Button from "$lib/components/ui/button.svelte"
	import Select from "$lib/components/ui/select.svelte"
	import Input from "$lib/components/ui/input.svelte"
	import Badge from "$lib/components/ui/badge.svelte"
	import { Plus, Loader2, Trash2, Search } from "@lucide/svelte"
	import { toast } from "svelte-sonner"
	import type { DegreeKind } from "$lib/types"

	const queryClient = useQueryClient()

	const academicsQuery = createQuery(() => ({
		queryKey: ["admin", "academics"],
		queryFn: () => academicsService.list(),
	}))

	let selectedAcademic = $state<string | null>(null)

	const degreesQuery = createQuery(() => ({
		queryKey: ["admin", "degrees", selectedAcademic],
		queryFn: () => degreesService.listByAcademic(selectedAcademic as string),
		enabled: Boolean(selectedAcademic),
	}))

	let showCreate = $state(false)
	let academicId = $state("")
	let name = $state("")
	let university = $state("")
	let obtainedAt = $state("")
	let kind = $state<DegreeKind>("base")
	let countryCode = $state("CL")

	const createDeg = createTanMutation(() => ({
		mutationFn: () =>
			degreesService.create({ academicId, name, university, obtainedAt, kind, countryCode }),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["admin", "degrees", selectedAcademic] })
			toast.success("Grado creado")
			showCreate = false
			academicId = ""
			name = ""
			university = ""
			obtainedAt = ""
			kind = "base"
			countryCode = "CL"
		},
		onError: () => toast.error("Error al crear el grado"),
	}))

	const academics = $derived(academicsQuery.data ?? [])

	function kindLabel(k: string): string {
		switch (k) {
			case "base":
				return "BASE"
			case "advanced":
				return "AVANZADO"
			default:
				return k
		}
	}

	function fmt(iso: string): string {
		const d = new Date(iso + "T00:00:00")
		return d.toLocaleDateString("es-CL", { day: "2-digit", month: "2-digit", year: "numeric" })
	}
</script>

<div>
	<div class="mb-6 flex items-center justify-between">
		<div>
			<h1 class="text-lg font-semibold text-[#1A1A1A]">Grados Académicos</h1>
			<p class="mt-1 text-sm text-corp-gray">Gestiona los grados académicos por académico.</p>
		</div>
		<Button onclick={() => (showCreate = true)}>
			<Plus class="size-4" />
			Nuevo
		</Button>
	</div>

	<div class="mb-6">
		<label class="grid gap-1.5 max-w-xs">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Académico</span>
			<div class="relative">
				<Search class="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-corp-gray/50" />
				<select
					class="h-10 w-full appearance-none rounded-lg border border-corp-gray/20 bg-white pl-10 pr-10 text-sm text-[#1A1A1A] outline-none transition-colors focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
					bind:value={selectedAcademic}
				>
					<option value={null}>Seleccionar académico...</option>
					{#each academics as a (a.id)}
						<option value={a.id}>
							{a.names}
							{a.paternalSurname}
							{a.maternalSurname} — {a.email}
						</option>
					{/each}
				</select>
			</div>
		</label>
	</div>

	{#if degreesQuery.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader2 class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if selectedAcademic}
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
							>Universidad</th
						>
						<th
							class="px-4 py-3 text-left text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Tipo</th
						>
						<th
							class="px-4 py-3 text-left text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Fecha</th
						>
						<th
							class="px-4 py-3 text-left text-xs font-medium tracking-wide uppercase text-corp-gray"
							>País</th
						>
						<th class="px-4 py-3 w-16"></th>
					</tr>
				</thead>
				<tbody>
					{#each degreesQuery.data ?? [] as deg (deg.id)}
						<tr class="border-b border-corp-gray/10 last:border-0">
							<td class="px-4 py-3 font-medium text-[#1A1A1A]">{deg.name}</td>
							<td class="px-4 py-3 text-corp-gray">{deg.university}</td>
							<td class="px-4 py-3">
								<Badge variant={deg.kind === "base" ? "base" : "advanced"}>
									{kindLabel(deg.kind)}
								</Badge>
							</td>
							<td class="px-4 py-3 text-corp-gray">{fmt(deg.obtainedAt)}</td>
							<td class="px-4 py-3 text-corp-gray">{deg.countryCode}</td>
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
			{#if (degreesQuery.data ?? []).length === 0}
				<p class="px-4 py-8 text-center text-sm text-corp-gray">
					No hay grados registrados para este académico.
				</p>
			{/if}
		</div>
	{:else}
		<div class="flex items-center justify-center py-16">
			<p class="text-sm text-corp-gray">Selecciona un académico para ver sus grados.</p>
		</div>
	{/if}
</div>

<Dialog bind:open={showCreate} title="Nuevo grado académico">
	<form
		class="grid gap-4"
		onsubmit={(e) => {
			e.preventDefault()
			createDeg.mutate()
		}}
	>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Académico</span>
			<Select
				bind:value={academicId}
				items={academics.map((a) => ({
					value: a.id,
					label: `${a.names} ${a.paternalSurname}`,
				}))}
				placeholder="Seleccionar académico..."
			/>
		</label>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
				>Nombre del grado</span
			>
			<Input bind:value={name} placeholder="Ej: Magíster en Ciencias" required />
		</label>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Universidad</span>
			<Input bind:value={university} placeholder="Ej: Universidad Católica de Temuco" required />
		</label>
		<div class="grid grid-cols-2 gap-4">
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Fecha</span>
				<Input type="date" bind:value={obtainedAt} required />
			</label>
			<label class="grid gap-1.5">
				<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">Tipo</span>
				<Select
					bind:value={kind}
					items={[
						{ value: "base", label: "Base" },
						{ value: "advanced", label: "Avanzado" },
					]}
				/>
			</label>
		</div>
		<label class="grid gap-1.5">
			<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">País</span>
			<Input bind:value={countryCode} placeholder="CL" maxlength={2} required />
		</label>
		<div class="mt-2 flex justify-end gap-2">
			<Button variant="secondary" type="button" onclick={() => (showCreate = false)}
				>Cancelar</Button
			>
			<Button
				type="submit"
				disabled={createDeg.isPending || !academicId || !name || !university || !obtainedAt}
			>
				{createDeg.isPending ? "Creando..." : "Crear"}
			</Button>
		</div>
	</form>
</Dialog>
