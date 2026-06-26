<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { academicsService } from "$lib/services/academics.service"
	import { categoriesService } from "$lib/services/categories.service"
	import { optionsService } from "$lib/services/options.service"
	import { positionsService } from "$lib/services/positions.service"
	import { Users, Tags, ListOrdered, Briefcase } from "@lucide/svelte"

	const academicsQuery = createQuery(() => ({
		queryKey: ["admin", "academics"],
		queryFn: () => academicsService.list(),
	}))

	const categoriesQuery = createQuery(() => ({
		queryKey: ["admin", "categories"],
		queryFn: () => categoriesService.list(),
	}))

	const optionsQuery = createQuery(() => ({
		queryKey: ["admin", "options"],
		queryFn: () => optionsService.list(),
	}))

	const positionsQuery = createQuery(() => ({
		queryKey: ["admin", "positions"],
		queryFn: () => positionsService.list(),
	}))
</script>

<div>
	<h1 class="text-lg font-semibold text-[#1A1A1A]">Administración</h1>
	<p class="mt-1 text-sm text-corp-gray">Panel de gestión del sistema.</p>

	<div class="mt-8 grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
		<a
			href="/admin/categories"
			class="rounded-xl border border-corp-gray/20 bg-white p-5 transition-colors hover:border-corp-blue/30"
		>
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<Tags class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">{categoriesQuery.data?.length ?? "—"}</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Categorías</p>
		</a>

		<a
			href="/admin/options"
			class="rounded-xl border border-corp-gray/20 bg-white p-5 transition-colors hover:border-corp-blue/30"
		>
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<ListOrdered class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">{optionsQuery.data?.length ?? "—"}</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Opciones</p>
		</a>

		<a
			href="/admin/positions"
			class="rounded-xl border border-corp-gray/20 bg-white p-5 transition-colors hover:border-corp-blue/30"
		>
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<Briefcase class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">{positionsQuery.data?.length ?? "—"}</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Cargos</p>
		</a>

		<div class="rounded-xl border border-corp-gray/20 bg-white p-5">
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<Users class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">{academicsQuery.data?.length ?? "—"}</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Académicos</p>
		</div>
	</div>
</div>
