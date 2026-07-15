<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { academicService } from "$lib/academic/academics/service"
	import { categoryService } from "$lib/academic/categories/service"
	import { optionService } from "$lib/academic/options/service"
	import { positionService } from "$lib/university/work-positions/service"
	import { usersService } from "$lib/users/service"
	import { GraduationCap, Users, Tags, ListOrdered, Briefcase } from "@lucide/svelte"

	const academicsQuery = createQuery(() => ({
		queryKey: ["admin", "academics"],
		queryFn: () => academicService.list(),
	}))

	const usersQuery = createQuery(() => ({
		queryKey: ["admin", "users"],
		queryFn: () => usersService.list(),
	}))

	const categoriesQuery = createQuery(() => ({
		queryKey: ["admin", "categories"],
		queryFn: () => categoryService.list(),
	}))

	const optionsQuery = createQuery(() => ({
		queryKey: ["admin", "options"],
		queryFn: () => optionService.list(),
	}))

	const positionsQuery = createQuery(() => ({
		queryKey: ["admin", "positions"],
		queryFn: () => positionService.list(),
	}))
</script>

<div>
	<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-5">
		<a
			href="/admin/categories"
			class="rounded-xl border border-corp-gray/20 bg-white p-5 transition-colors hover:border-corp-blue/30"
		>
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<Tags class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">
				{categoriesQuery.data?.length ?? "—"}
			</p>
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
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">
				{optionsQuery.data?.length ?? "—"}
			</p>
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
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">
				{positionsQuery.data?.length ?? "—"}
			</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Cargos</p>
		</a>

		<a
			href="/academics"
			class="rounded-xl border border-corp-gray/20 bg-white p-5 transition-colors hover:border-corp-blue/30"
		>
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<GraduationCap class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">
				{academicsQuery.data?.length ?? "—"}
			</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Académicos</p>
		</a>

		<a
			href="/admin/users"
			class="rounded-xl border border-corp-gray/20 bg-white p-5 transition-colors hover:border-corp-blue/30"
		>
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<Users class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">
				{usersQuery.data?.length ?? "—"}
			</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Usuarios</p>
		</a>
	</div>
</div>
