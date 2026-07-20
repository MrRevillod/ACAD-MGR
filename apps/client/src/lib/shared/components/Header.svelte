<script lang="ts">
	import { page } from "$app/state"
	import { goto } from "$app/navigation"
	import { authStore } from "$lib/auth/store.svelte"
	import { authService } from "$lib/auth/service"
	import { useMutation } from "$shared/http/tanstack"
	import { Users, Settings, LogOut, LogIn, BookOpen, ChartBar } from "@lucide/svelte"

	const logoutMutation = useMutation(() => ({
		mutationFn: () => authService.logout(),
		onSuccess: () => {
			authStore.clearSession()
			void goto("/")
		},
	}))

	const path = $derived(page.url.pathname)

	function navClass(prefix: string) {
		return path.startsWith(prefix)
			? "inline-flex items-center gap-1.5 rounded-lg px-3 py-2 text-sm font-medium text-corp-blue bg-corp-blue/5 transition-colors"
			: "inline-flex items-center gap-1.5 rounded-lg px-3 py-2 text-sm font-medium text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-[#1A1A1A]"
	}

	function handleLogout() {
		logoutMutation.mutate()
	}
</script>

<header class="sticky top-0 z-30 border-b border-corp-gray/20 bg-white/95 backdrop-blur-sm">
	<div class="mx-auto flex h-16 max-w-[1600px] items-center justify-between px-4 sm:px-6 lg:px-8">
		<a href="/" class="select-none -ml-7">
			<img src="/logo-header.png" alt="Facultad de Ingeniería" class="h-16 w-auto" />
		</a>

		<nav class="flex items-center gap-1">
			<a href="/works" class={navClass("/works")}>
				<BookOpen class="size-4" />
				<span class="hidden sm:inline">Publicaciones</span>
			</a>

			<a href="/stats" class={navClass("/stats")}>
				<ChartBar class="size-4" />
				<span class="hidden sm:inline">Estadísticas</span>
			</a>

			{#if !authStore.isAuthenticated}
				<a href="/public/academics" class={navClass("/public/academics")}>
					<Users class="size-4" />
					<span class="hidden sm:inline">Académicos</span>
				</a>
			{/if}

			{#if authStore.isAuthenticated}
				<a href="/academics" class={navClass("/academics")}>
					<Users class="size-4" />
					<span class="hidden sm:inline">Académicos</span>
				</a>
			{/if}

			{#if authStore.isAuthenticated}
				<a href="/admin" class={navClass("/admin")}>
					<Settings class="size-4" />
					<span class="hidden sm:inline">Administración</span>
				</a>

				<span class="mx-2 h-5 w-px bg-corp-gray/20"></span>

				<span class="text-xs text-corp-gray truncate max-w-28 hidden sm:inline">
					{authStore.user?.name}
				</span>

				<button
					class="inline-flex items-center gap-1.5 rounded-lg px-3 py-2 text-sm font-medium text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-red-600"
					onclick={handleLogout}
					disabled={logoutMutation.isPending}
				>
					<LogOut class="size-4" />
				</button>
			{:else}
				<a
					href="/login"
					class="inline-flex items-center gap-1.5 rounded-lg px-3 py-2 text-sm font-medium text-corp-blue transition-colors hover:bg-corp-blue/5"
				>
					<LogIn class="size-4" />
					<span class="hidden sm:inline">Ingresar</span>
				</a>
			{/if}
		</nav>
	</div>
</header>
