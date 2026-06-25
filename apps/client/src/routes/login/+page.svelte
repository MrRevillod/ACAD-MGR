<script lang="ts">
	import * as v from "valibot"
	import { loginSchema, type LoginInput } from "$lib/auth/auth.dtos"

	import { createMutation } from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { toast } from "svelte-sonner"
	import { LogIn } from "@lucide/svelte"
	import { authService } from "$lib/auth/auth.service"
	import { authStore } from "$lib/auth/auth.store.svelte"
	import { ApiResponse } from "$lib/shared/http/response"

	let username = $state("")
	let password = $state("")
	let errors = $state<Record<string, string>>({})

	const loginMutation = createMutation(() => ({
		mutationFn: (payload: LoginInput) => authService.login(payload),
		onSuccess: async (user) => {
			authStore.setSession(user)
			password = ""
			await goto(resolve("/"))
		},
		onError: (error) => {
			toast.error(error instanceof ApiResponse ? error.message : "Ocurrió un error inesperado.")
		},
	}))

	const loading = $derived(loginMutation.isPending)

	const handleSubmit = async (event: SubmitEvent) => {
		event.preventDefault()
		errors = {}

		const result = v.safeParse(loginSchema, { username, password })

		if (!result.success) {
			const flat = v.flatten(result.issues)
			if (flat.nested) {
				for (const [key, msgs] of Object.entries(flat.nested)) {
					if (msgs?.length) errors[key] = msgs[0]
				}
			}
			return
		}

		await loginMutation.mutateAsync(result.output)
	}
</script>

<main class="min-h-dvh px-4 py-8 sm:px-6 sm:py-12">
	<div class="mx-auto flex min-h-[calc(100dvh-4rem)] max-w-5xl items-center justify-center">
		<section
			class="-mt-10 w-full max-w-xl rounded-xl border border-zinc-200/60 bg-white p-8 sm:p-10"
		>
			<h1 class="m-0 mt-2 text-3xl leading-tight text-black sm:text-[2.15rem]">Iniciar sesión</h1>
			<p class="mt-3 mb-7 max-w-md text-base leading-relaxed text-zinc-700">
				Usa tus credenciales Pillan/LDAP para ingresar.
			</p>

			<form class="grid gap-4" onsubmit={handleSubmit}>
				<label class="grid gap-1.5">
					<span class="text-sm text-zinc-800">Usuario</span>
					<input
						class="h-10 w-full rounded-lg border border-zinc-300/60 bg-zinc-50 px-3 text-sm text-zinc-900 outline-none transition-colors placeholder:text-zinc-400 focus:border-corp-blue/50 focus:bg-white focus:ring-2 focus:ring-corp-blue/10"
						class:border-red-500={errors.username}
						type="text"
						bind:value={username}
						autocomplete="username"
					/>
					{#if errors.username}
						<p class="text-xs text-red-600">{errors.username}</p>
					{/if}
				</label>

				<label class="grid gap-1.5">
					<span class="text-sm text-zinc-800">Contraseña</span>
					<input
						class="h-10 w-full rounded-lg border border-zinc-300/60 bg-zinc-50 px-3 text-sm text-zinc-900 outline-none transition-colors placeholder:text-zinc-400 focus:border-corp-blue/50 focus:bg-white focus:ring-2 focus:ring-corp-blue/10"
						class:border-red-500={errors.password}
						type="password"
						bind:value={password}
						autocomplete="current-password"
					/>
					{#if errors.password}
						<p class="text-xs text-red-600">{errors.password}</p>
					{/if}
				</label>

				<button
					class="mt-2 flex w-full items-center justify-center gap-1.5 rounded-lg bg-corp-blue px-4 py-2.5 text-base font-medium text-white outline-none transition-colors hover:bg-corp-blue/90 focus:ring-2 focus:ring-corp-blue/30 disabled:cursor-not-allowed disabled:opacity-50"
					type="submit"
					disabled={loading}
				>
					<LogIn size={16} aria-hidden="true" />
					{loading ? "Ingresando..." : "Ingresar"}
				</button>
			</form>

			<p class="mt-5 mb-0 text-sm text-zinc-700">
				<a
					class="font-medium text-zinc-900 underline decoration-zinc-400 underline-offset-3 hover:decoration-zinc-900"
					href="https://chpass.inf.uct.cl"
					target="_blank"
					rel="noopener noreferrer"
				>
					¿Olvidaste tu contraseña?
				</a>
			</p>
		</section>
	</div>
</main>
