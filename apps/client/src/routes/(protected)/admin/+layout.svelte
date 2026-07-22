<script lang="ts">
	import type { Snippet } from "svelte"

	import { page } from "$app/state"
	import {
		BookOpen,
		Tags,
		ListOrdered,
		Briefcase,
		LayoutDashboard,
		GraduationCap,
		Users,
		GitFork,
		ChevronLeft,
	} from "@lucide/svelte"

	let { children }: { children: Snippet } = $props()

	const navItems = [
		{ href: "/admin", label: "Dashboard", icon: LayoutDashboard },
		{ href: "/admin/categories", label: "Categorías", icon: Tags },
		{ href: "/admin/options", label: "Opciones", icon: ListOrdered },
		{ href: "/admin/positions", label: "Cargos", icon: Briefcase },
		{ href: "/admin/research-lines", label: "Líneas Investigación", icon: GitFork },
		{ href: "/academics", label: "Académicos", icon: GraduationCap },
		{ href: "/works", label: "Publicaciones", icon: BookOpen },
		{ href: "/admin/users", label: "Usuarios", icon: Users },
	] as const

	const currentPagePath = $derived(page.url.pathname)

	let collapsed = $state(true)
</script>

<div class="mx-auto flex h-full max-w-[1600px] flex-col px-4 py-8 sm:px-6 lg:px-8">
	<div class="flex min-h-0 flex-1 gap-8">
		<aside
			class="relative hidden shrink-0 self-start rounded-xl border border-corp-gray/20 bg-white transition-all duration-200 lg:block {collapsed
				? 'w-16'
				: 'w-72'}"
		>
			<button
				type="button"
				class="absolute -right-3 top-5 z-10 flex size-6 items-center justify-center rounded-full border border-corp-gray/20 bg-white text-corp-gray shadow-sm transition-colors hover:text-[#1A1A1A]"
				onclick={() => (collapsed = !collapsed)}
			>
				<ChevronLeft
					class="size-3.5 transition-transform duration-200 {collapsed
						? 'rotate-180'
						: ''}"
				/>
			</button>

			<div class="p-4 {collapsed ? 'px-3' : ''}">
				{#if !collapsed}
					<h1 class="text-lg font-semibold text-[#1A1A1A]">Administración</h1>
					<p class="mt-1 text-sm text-corp-gray">Gestión del sistema</p>
				{/if}
			</div>

			<nav class="mt-2 space-y-1 px-2">
				{#each navItems as item (item.href)}
					<a
						href={item.href}
						class="flex items-center rounded-lg px-3 py-2 text-sm font-medium transition-colors {currentPagePath ===
						item.href
							? 'bg-corp-gray/10 text-[#1A1A1A]'
							: 'text-corp-gray hover:bg-corp-gray/5 hover:text-[#1A1A1A]'} {collapsed
							? 'justify-center px-0'
							: 'gap-2.5'}"
						title={collapsed ? item.label : undefined}
					>
						<item.icon class="size-4 shrink-0" />
						{#if !collapsed}
							{item.label}
						{/if}
					</a>
				{/each}
			</nav>
		</aside>

		<div class="min-w-0 flex-1 overflow-hidden">
			{@render children()}
		</div>
	</div>
</div>
