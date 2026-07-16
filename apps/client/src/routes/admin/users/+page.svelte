<script lang="ts">
	import type { User } from "$lib/users/entity"
	import type { TableFeatures } from "@tanstack/svelte-table"

	import { toast } from "svelte-sonner"
	import { createQuery } from "@tanstack/svelte-query"
	import { usersService } from "$users/service"
	import { inlineTryAsync } from "$shared/try"
	import { createColumnHelper } from "@tanstack/svelte-table"
	import { Loader, CircleAlert, Plus } from "@lucide/svelte"

	import DataTable from "$shared/components/ui/data-table.svelte"
	import UserDialog from "$auth/components/user-dialog.svelte"

	let search = $state("")
	let showDialog = $state(false)
	let editingUser: User | null = $state(null)

	const usersQuery = createQuery(() => ({
		queryKey: ["users", search],
		queryFn: () => usersService.list(search ? { search } : undefined),
	}))

	const users = $derived(usersQuery.data ?? [])

	function openCreate() {
		editingUser = null
		showDialog = true
	}

	function openEdit(u: User) {
		editingUser = u
		showDialog = true
	}

	async function handleDelete(u: User) {
		if (!window.confirm(`¿Eliminar a "${u.name}"? Esta acción no se puede deshacer.`)) return

		const [_, e] = await inlineTryAsync(async () => {
			await usersService.delete(u.id)
			toast.success("Usuario eliminado")
			void usersQuery.refetch()
		})

		if (e !== null) {
			toast.error("Error al eliminar el usuario")
		}
	}

	const helper = createColumnHelper<TableFeatures, User>()

	const columns = [
		helper.accessor("name", { header: "Nombre" }),
		helper.accessor("email", { header: "Email" }),
		helper.accessor((row) => (row.role.code === "admin" ? "Administrador" : row.role.code), {
			header: "Rol",
		}),
	]
</script>

<div>
	<div class="mb-4 flex items-center justify-between">
		<div class="relative flex-1 max-w-sm">
			<input
				type="text"
				bind:value={search}
				placeholder="Buscar usuarios..."
				class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white pl-10 pr-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
			/>
			<svg
				class="absolute left-3 top-1/2 size-4 -translate-y-1/2 text-corp-gray/50"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				viewBox="0 0 24 24"
			>
				<circle cx="11" cy="11" r="8" />
				<path d="m21 21-4.3-4.3" />
			</svg>
		</div>
		<button
			class="inline-flex items-center gap-1.5 rounded-lg bg-corp-blue px-4 py-2 text-sm font-medium text-white shadow-sm transition-colors hover:bg-corp-blue/90 active:scale-95"
			onclick={openCreate}
		>
			<Plus class="size-4" />
			Crear usuario
		</button>
	</div>

	{#if usersQuery.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if usersQuery.isError}
		<div class="flex flex-col items-center justify-center py-16 text-center">
			<CircleAlert class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Error al cargar los usuarios.</p>
		</div>
	{:else}
		<DataTable data={users} {columns} onRowClick={(u: User) => openEdit(u)} />
	{/if}
</div>

<UserDialog
	user={editingUser}
	bind:open={showDialog}
	onClose={() => (showDialog = false)}
	onDelete={handleDelete}
/>
