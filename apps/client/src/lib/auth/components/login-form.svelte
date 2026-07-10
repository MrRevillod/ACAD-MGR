<script lang="ts">
	import { createForm, Field, Form } from "@formisch/svelte"
	import { loginSchema, type LoginInput } from "$lib/auth/auth.dtos"
	import { createMutation } from "@tanstack/svelte-query"
	import { toast } from "svelte-sonner"
	import { LogIn, Eye, EyeOff } from "@lucide/svelte"
	import { authService } from "$lib/auth/auth.service"
	import { authStore } from "$lib/auth/auth.store.svelte"
	import type { ApiResponse } from "$lib/shared/http/response"

	interface Props {
		onSuccess?: () => void
	}

	let { onSuccess }: Props = $props()

	const form = createForm({ schema: loginSchema })

	let showPassword = $state(false)

	const loginMutation = createMutation(() => ({
		mutationFn: (payload: LoginInput) => authService.login(payload),
		onSuccess: async (user) => {
			authStore.setSession(user)
			onSuccess?.()
		},
		onError: (error: ApiResponse) => {
			toast.error(error.message ?? "Error al iniciar sesión")
		},
	}))

	const loading = $derived(loginMutation.isPending)
</script>

<section class="w-full max-w-md rounded-xl border border-corp-gray/20 bg-white px-6 py-8">
	<div class="mb-8 text-center">
		<h1 class="text-lg font-semibold text-[#1A1A1A]">Iniciar sesión</h1>
		<p class="mt-1 text-sm text-corp-gray">Ingresa a la web de Administración de la Plataforma.</p>
	</div>

	<Form of={form} onsubmit={(output) => loginMutation.mutate(output)}>
		<div class="grid gap-5">
			<Field of={form} path={["email"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Correo electrónico</span
						>
						<div class="relative">
							<input
								{...field.props}
								value={field.input}
								type="text"
								autocomplete="email"
								placeholder="jdoe@domain.com"
								class="h-10 w-full rounded-lg border bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10 {field.errors
									? 'border-red-500'
									: 'border-corp-gray/20'}"
							/>
						</div>
						{#if field.errors}
							<p class="text-xs text-red-600">{field.errors[0]}</p>
						{/if}
					</label>
				{/snippet}
			</Field>

			<Field of={form} path={["password"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Contraseña</span
						>
						<div class="relative">
							<input
								{...field.props}
								value={field.input}
								type={showPassword ? "text" : "password"}
								autocomplete="current-password"
								placeholder="••••••••"
								class="h-10 w-full rounded-lg border bg-white pl-3 pr-10 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10 {field.errors
									? 'border-red-500'
									: 'border-corp-gray/20'}"
							/>
							<button
								type="button"
								class="absolute right-2.5 top-1/2 -translate-y-1/2 text-corp-gray/50 transition-colors hover:text-corp-gray"
								onclick={() => (showPassword = !showPassword)}
								tabindex={-1}
							>
								{#if showPassword}
									<EyeOff class="size-4" />
								{:else}
									<Eye class="size-4" />
								{/if}
							</button>
						</div>
						{#if field.errors}
							<p class="text-xs text-red-600">{field.errors[0]}</p>
						{/if}
					</label>
				{/snippet}
			</Field>

			<button
				class="mt-2 flex h-10 w-full items-center justify-center gap-1.5 rounded-lg bg-corp-blue px-4 text-sm font-medium text-white outline-none transition-colors hover:bg-corp-blue/90 focus:ring-2 focus:ring-corp-blue/30 disabled:cursor-not-allowed disabled:opacity-50"
				type="submit"
				disabled={loading}
			>
				<LogIn size={16} aria-hidden="true" />
				{loading ? "Ingresando..." : "Ingresar"}
			</button>
		</div>
	</Form>

	<p class="mt-6 text-center text-sm text-corp-gray">
		<a
			class="font-medium text-corp-blue underline decoration-corp-blue/30 underline-offset-3 hover:decoration-corp-blue/60"
			href="https://chpass.inf.uct.cl"
			target="_blank"
			rel="noopener noreferrer"
		>
			¿Olvidaste tu contraseña?
		</a>
	</p>
</section>
