<script lang="ts">
	import type { Snippet } from "svelte"
	import { page } from "$app/stores"
	import { Tags, ListOrdered, GraduationCap, Briefcase, LayoutDashboard } from "@lucide/svelte"

	let { children }: { children: Snippet } = $props()

	const navItems = [
		{ href: "/admin", label: "Dashboard", icon: LayoutDashboard },
		{ href: "/admin/categories", label: "Categorías", icon: Tags },
		{ href: "/admin/options", label: "Opciones", icon: ListOrdered },
		{ href: "/admin/degrees", label: "Grados", icon: GraduationCap },
		{ href: "/admin/positions", label: "Cargos", icon: Briefcase },
	] as const
</script>

<div class="mx-auto flex max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
	<aside class="hidden w-56 shrink-0 lg:block">
		<nav class="sticky top-20 space-y-1">
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

	<div class="min-w-0 flex-1 lg:ml-8">
		{@render children()}
	</div>
</div>
