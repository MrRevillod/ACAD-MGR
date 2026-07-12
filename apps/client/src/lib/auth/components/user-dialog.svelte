<script lang="ts">
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { createMutation, useQueryClient } from "@tanstack/svelte-query"
	import Dialog from "$lib/shared/components/ui/dialog.svelte"
	import Button from "$lib/shared/components/ui/button.svelte"
	import { userService } from "$lib/auth/users.service"
	import { toast } from "svelte-sonner"
	import { createUserSchema, updateUserSchema } from "$lib/auth/auth.dtos"
	import type { User } from "$lib/auth/auth.dtos"
	import type { CreateUserDto, UpdateUserDto } from "$lib/auth/auth.dtos"

	interface Props {
		user?: User | null
		open: boolean
		onClose: () => void
		onDelete?: (u: User) => void
	}

	let { user = null, open = $bindable(), onClose, onDelete }: Props = $props()

	// svelte-ignore state_referenced_locally
	const schema = user ? updateUserSchema : createUserSchema
	const form = createForm({ schema })

	$effect(() => {
		if (!open) return
		if (user) {
			reset(form, {
				initialInput: {
					name: user.name,
					email: user.email,
					role: user.role,
				},
			})
		} else {
			reset(form, {
				initialInput: {
					name: "",
					email: "",
					password: "",
					role: "admin" as const,
				},
			})
		}
	})

	const queryClient = useQueryClient()

	const createUserMut = createMutation(() => ({
		mutationFn: (output: CreateUserDto) => userService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["users"] })
			toast.success("Usuario creado")
			open = false
		},
		onError: () => toast.error("Error al crear el usuario"),
	}))

	const updateUserMut = createMutation(() => ({
		mutationFn: ({ id, data }: { id: string; data: UpdateUserDto }) =>
			userService.update(id, data),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["users"] })
			toast.success("Usuario actualizado")
			open = false
		},
		onError: () => toast.error("Error al actualizar el usuario"),
	}))

	function handleCreate(output: CreateUserDto) {
		createUserMut.mutate(output)
	}

	function handleUpdate(output: UpdateUserDto) {
		if (!user) return
		const data: UpdateUserDto = {
			name: output.name,
			email: output.email,
		}
		if (output.password) data.password = output.password
		updateUserMut.mutate({ id: user.id, data })
	}

	const pending = $derived(createUserMut.isPending || updateUserMut.isPending)
</script>

<Dialog bind:open title={user ? "Editar usuario" : "Nuevo usuario"} class="max-w-xl">
	<Form
		of={form}
		onsubmit={(output) =>
			user ? handleUpdate(output as UpdateUserDto) : handleCreate(output as CreateUserDto)}
	>
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
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						/>
						{#if field.errors}
							<p class="text-xs text-red-600">{field.errors[0]}</p>
						{/if}
					</label>
				{/snippet}
			</Field>
			<Field of={form} path={["email"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Email</span
						>
						<input
							{...field.props}
							value={field.input}
							type="email"
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						/>
						{#if field.errors}
							<p class="text-xs text-red-600">{field.errors[0]}</p>
						{/if}
					</label>
				{/snippet}
			</Field>
			<Field of={form} path={["password"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray">
							Contraseña
							{#if user}
								(dejar vacío para mantener){/if}
						</span>
						<input
							{...field.props}
							value={field.input ?? ""}
							type="password"
							placeholder={user ? "Sin cambios" : ""}
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						/>
						{#if field.errors}
							<p class="text-xs text-red-600">{field.errors[0]}</p>
						{/if}
					</label>
				{/snippet}
			</Field>
			<div class="mt-2 flex items-center justify-between gap-2">
				{#if user && onDelete}
					<button
						type="button"
						class="rounded-lg px-3 py-2 text-sm font-medium text-red-600 transition-colors hover:bg-red-50"
						onclick={() => onDelete(user)}
					>
						Eliminar usuario
					</button>
				{:else}
					<span></span>
				{/if}
				<div class="flex gap-2">
					<Button variant="secondary" type="button" onclick={onClose}>Cancelar</Button>
					<Button type="submit" disabled={pending}>
						{pending ? "Guardando..." : "Guardar"}
					</Button>
				</div>
			</div>
		</div>
	</Form>
</Dialog>
