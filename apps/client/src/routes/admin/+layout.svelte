<script lang="ts">
	import type { Snippet } from "svelte"
	import { page } from "$app/stores"
	import {
		BookOpen,
		Tags,
		ListOrdered,
		Briefcase,
		LayoutDashboard,
		GraduationCap,
		Users,
	} from "@lucide/svelte"

	let { children }: { children: Snippet } = $props()

	const navItems = [
		{ href: "/admin", label: "Dashboard", icon: LayoutDashboard },
		{ href: "/admin/categories", label: "Categorías", icon: Tags },
		{ href: "/admin/options", label: "Opciones", icon: ListOrdered },
		{ href: "/admin/positions", label: "Cargos", icon: Briefcase },
		{ href: "/academics", label: "Académicos", icon: GraduationCap },
		{ href: "/works", label: "Publicaciones", icon: BookOpen },
		{ href: "/admin/users", label: "Usuarios", icon: Users },
	] as const
</script>

<div class="mx-auto flex h-full max-w-[1600px] flex-col px-4 py-8 sm:px-6 lg:px-8">
	<div class="flex min-h-0 flex-1 gap-8">
		<aside
			class="hidden w-72 shrink-0 self-start rounded-xl border border-corp-gray/20 bg-white p-4 lg:block"
		>
			<h1 class="text-lg font-semibold text-[#1A1A1A]">Administración</h1>
			<p class="mt-1 text-sm text-corp-gray">Gestión del sistema</p>

			<nav class="mt-6 space-y-1">
				{#each navItems as item (item.href)}
					<a
						href={item.href}
						class="flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm font-medium transition-colors {$page
							.url.pathname === item.href
							? 'bg-corp-gray/10 text-[#1A1A1A]'
							: 'text-corp-gray hover:bg-corp-gray/5 hover:text-[#1A1A1A]'}"
					>
						<item.icon class="size-4" />
						{item.label}
					</a>
				{/each}
			</nav>
		</aside>

		<div class="min-w-0 flex-1 overflow-hidden">
			{@render children()}
		</div>
	</div>
</div>
